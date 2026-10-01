#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

GRADLE_CMD="./gradlew"
if [[ ! -x "$GRADLE_CMD" ]]; then
  if command -v gradle >/dev/null 2>&1; then
    GRADLE_CMD="gradle"
  else
    echo "FAIL: Gradle wrapper is missing and no system Gradle is available."
    exit 1
  fi
fi

echo "=== Eagle Pre-Push Gate ==="
echo "[1/3] Unit tests"
"$GRADLE_CMD" :app:testDebugUnitTest --no-daemon --console=plain

echo "[2/3] Lint"
"$GRADLE_CMD" :app:lint --no-daemon --console=plain

echo "[3/3] Debug build"
"$GRADLE_CMD" :app:assembleDebug --no-daemon --console=plain

echo "PASS: mandatory pre-GitHub gate."
