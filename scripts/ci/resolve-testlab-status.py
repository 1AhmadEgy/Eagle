#!/usr/bin/env python3
"""Resolve Test Lab status for the exact commit SHA using GitHub's workflow-runs API."""

import json
import os
import urllib.parse
import urllib.request
from pathlib import Path


def main() -> None:
    token = os.environ.get("GH_TOKEN")
    repository = os.environ["GITHUB_REPOSITORY"]
    sha = os.environ["GITHUB_SHA"]
    api_url = os.environ.get("GITHUB_API_URL", "https://api.github.com")

    if not token:
        raise SystemExit("GH_TOKEN is required")

    query = urllib.parse.urlencode({"head_sha": sha, "per_page": "20"})
    url = f"{api_url}/repos/{repository}/actions/workflows/ci.yml/runs?{query}"

    request = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )

    with urllib.request.urlopen(request, timeout=20) as response:
        payload = json.load(response)

    status = "UNCONFIRMED"
    source = "No completed CI run found for this exact commit SHA."

    runs = payload.get("workflow_runs", [])
    for run in runs:
        if run.get("head_sha") != sha:
            continue
        if run.get("status") != "completed":
            continue

        conclusion = run.get("conclusion")
        run_id = run.get("id")
        if conclusion == "success":
            status = "PENDING"
            source = (
                f"Exact-SHA CI run {sha}, workflow CI, run {run_id}: "
                "baseline verification succeeded; required Test Lab categories remain "
                "pending unless separately evidenced."
            )
            break
        if conclusion == "failure":
            status = "FAIL"
            source = (
                f"Exact-SHA CI run {sha}, workflow CI, run {run_id}: "
                "verification failed."
            )
            break

    output = {
        "status": status,
        "evidence_source": source,
        "commit": sha,
        "workflow": "CI",
    }

    path = Path(".ci/documentation/testlab-status.json")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
