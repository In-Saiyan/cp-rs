#!/bin/bash
# bundle.sh - Simple script to bundle code and run

echo "Building and bundling code with AST-based bundler..."

# Run the main program and bundle simultaneously
cargo build --release
cargo run --bin ast_bundler

echo "Code bundled! Check bundled/solution.rs"
echo "Ready for competitive programming submission!"