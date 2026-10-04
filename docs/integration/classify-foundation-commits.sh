#!/usr/bin/env bash
set -euo pipefail

BASE_REF="${1:-14a6f1d6}"
HEAD_REF="${2:-execution/core-foundation-v1}"

declare -a mixed

classify_path() {
  local path="$1"
  case "$path" in
    docs/*|README.md) printf '%s\n' A ;;
    .github/*|scripts/ci/*|rust-toolchain.toml) printf '%s\n' B ;;
    core/*|Cargo.toml|Cargo.lock|scripts/ffi/*) printf '%s\n' C ;;
    shared/*) printf '%s\n' D ;;
    app/*) printf '%s\n' E ;;
    *) printf '%s\n' X ;;
  esac
}

printf 'Foundation commit classification: %s..%s\n\n' "$BASE_REF" "$HEAD_REF"

mapfile -t commits < <(git log --reverse --format='%H' "$BASE_REF..$HEAD_REF")

for sha in "${commits[@]}"; do
  message="$(git show -s --format='%s' "$sha")"
  mapfile -t paths < <(git diff-tree --no-commit-id --name-only -r "$sha")

  unset seen || true
  declare -A seen=()

  for path in "${paths[@]}"; do
    seen["$(classify_path "$path")"]=1
  done

  layers=()
  for layer in A B C D E X; do
    [[ "${seen[$layer]+yes}" == yes ]] && layers+=("$layer")
  done

  real_count=0
  for layer in "${layers[@]}"; do
    [[ "$layer" != X ]] && ((real_count+=1))
  done

  printf '%s\t%s\t%s\n' "${sha:0:12}" "$message" "${layers[*]:-none}"

  if (( real_count >= 2 )); then
    mixed+=("$sha")
  fi
done

printf '\nMixed commits (A–E crossing): %d\n' "${#mixed[@]}"
for sha in "${mixed[@]}"; do
  printf '\n%s  %s\n' "$sha" "$(git show -s --format='%s' "$sha")"
  while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    printf '  [%s] %s\n' "$(classify_path "$path")" "$path"
  done < <(git diff-tree --no-commit-id --name-only -r "$sha")
done
