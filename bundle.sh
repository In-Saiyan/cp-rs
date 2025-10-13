#!/bin/bash
# bundle.sh - Simple script to bundle code and run

echo "🚀 Building and bundling code..."

# Run the main program and bundle simultaneously
cargo build --release
cargo run --bin bundler

echo "📦 Code bundled! Check bundled/solution.rs"
echo "🎯 Ready for competitive programming submission!"

# Optional: Also compile the bundled version to verify it works
echo "🔧 Verifying bundled code compiles..."
rustc bundled/solution.rs -o bundled/solution --allow warnings
if [ $? -eq 0 ]; then
    echo "✅ Bundled code compiles successfully!"
    ls -la bundled/solution.rs | awk '{print "📁 Size:", $5, "bytes"}'
else
    echo "❌ Bundled code has compilation errors"
fi