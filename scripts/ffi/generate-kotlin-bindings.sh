#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UNIFFI_VERSION="0.32.1"
OUT_DIR="${ROOT}/shared/build/generated/uniffi/kotlin"

cd "${ROOT}"
cargo build -p eagle-core-ffi --release

if ! command -v uniffi-bindgen >/dev/null 2>&1; then
    cargo install "uniffi-bindgen@=${UNIFFI_VERSION}" --locked
fi

rm -rf "${OUT_DIR}"
mkdir -p "${OUT_DIR}"

uniffi-bindgen generate \
  --config "${ROOT}/core/ffi/uniffi.toml" \
  --library "${ROOT}/target/release/libeagle_core_ffi.so" \
  --language kotlin \
  --out-dir "${OUT_DIR}"

echo "Generated UniFFI Kotlin bindings under ${OUT_DIR}."
