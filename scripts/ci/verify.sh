#!/usr/bin/env bash
set -euo pipefail

echo "== Eagle CI verification =="

python3 scripts/ci/verify-repository-consistency.py
echo "OS: $(uname -s)"
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"

run_npm() {
  echo "== Node.js project detected =="
  node --version
  npm --version

  if [[ -f package-lock.json ]]; then
    npm ci
  elif [[ -f npm-shrinkwrap.json ]]; then
    npm ci
  else
    echo "WARNING: package.json exists without a lockfile; using npm install."
    npm install
  fi

  npm run format:check --if-present
  npm run lint --if-present
  npm test --if-present
  npm run build --if-present
}

run_python() {
  echo "== Python project detected =="
  python3 --version

  if [[ -f requirements.txt ]]; then
    python3 -m pip install --disable-pip-version-check -r requirements.txt
  fi

  if [[ -f requirements-dev.txt ]]; then
    python3 -m pip install --disable-pip-version-check -r requirements-dev.txt
  fi

  if [[ -f pyproject.toml ]] || [[ -f pytest.ini ]] || [[ -d tests ]]; then
    python3 -m pytest -q
  else
    echo "No pytest configuration/tests detected; Python dependency installation only."
  fi
}

run_go() {
  echo "== Go project detected =="
  go version
  if [[ -f go.sum ]]; then
    go mod download
  fi
  go test ./...
}

run_rust() {
  echo "== Rust project detected =="
  rustc --version
  if [[ -f Cargo.lock ]]; then
    cargo test --locked
  else
    cargo test
  fi
}

detected=0

if [[ -f package.json ]]; then
  detected=1
  run_npm
fi

if [[ -f pyproject.toml || -f requirements.txt || -f requirements-dev.txt || -f pytest.ini ]]; then
  detected=1
  run_python
fi

if [[ -f go.mod ]]; then
  detected=1
  run_go
fi

if [[ -f Cargo.toml ]]; then
  detected=1
  run_rust
fi

if [[ "$detected" -eq 0 ]]; then
  echo "No supported application toolchain detected yet."
  echo "Repository baseline verification completed successfully."
fi

echo "== CI verification completed successfully =="
