#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

echo "== cp-rs: workspace checks =="

if command -v cargo >/dev/null 2>&1; then
  echo "-> cargo test (root)"
  cargo test

  echo "-> cargo test (bundler-lib)"
  cargo test --manifest-path bundler-lib/Cargo.toml

  echo "-> cargo test (cp-lib)"
  cargo test --manifest-path cp-lib/Cargo.toml

  echo "-> cargo run --bin ast_bundler (also verifies bundled code)"
  cargo run --bin ast_bundler
else
  echo "ERROR: cargo not found in PATH" >&2
  exit 127
fi

echo "== standalone bundle compile check =="
if command -v rustc >/dev/null 2>&1; then
  rustc bundled/solution.rs -o bundled/solution_test --allow warnings
  rm -f bundled/solution_test
  echo "OK: bundled/solution.rs compiles"
else
  echo "WARN: rustc not found; skipped standalone compile check" >&2
fi

echo "== done =="
