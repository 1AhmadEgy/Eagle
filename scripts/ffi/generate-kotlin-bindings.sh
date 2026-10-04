#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${ROOT}/shared/build/generated/uniffi/kotlin"

cd "${ROOT}"
cargo build -p eagle-core-ffi --release

rm -rf "${OUT_DIR}"
mkdir -p "${OUT_DIR}"

cargo run -p eagle-core-ffi --features uniffi/cli --bin uniffi-bindgen -- \
  generate \
  --config "${ROOT}/core/ffi/uniffi.toml" \
  --library "${ROOT}/target/release/libeagle_core_ffi.so" \
  --language kotlin \
  --out-dir "${OUT_DIR}"

echo "Generated UniFFI Kotlin bindings under ${OUT_DIR}."
