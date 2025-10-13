# Bundler Integration Guide

## How the AST Bundler Works

The bundler processes `src/main.rs` and resolves all dependencies from `cp-lib/src/` into a single file for competitive programming submissions.

### Key Components

1. **AstBundler** (`bundler-lib/src/ast_bundler.rs`)
   - Parses Rust AST using `syn` crate
   - Resolves `use` statements and module dependencies
   - Combines all code into single file

2. **FileResolver** (`bundler-lib/src/file_resolver.rs`)
   - Maps module paths to actual files
   - Handles nested module structures
   - Resolves `pub use` re-exports

3. **FilenameGenerator** (`bundler-lib/src/filename_generator.rs`)
   - Extracts problem name from `const _PROBLEM`
   - Formats filename for submission (e.g., `d2._magic_powder_1760367815.rs`)

### When Adding New Features

#### Features that Need Bundler Consideration:
- New modules in `cp-lib/src/`
- Complex macro usage
- External crate dependencies
- File I/O operations (may need adjustment for single-file context)

#### Testing Bundler Integration:
```bash
# 1. Add your feature to main.rs
echo 'const _PROBLEM: &str = "A. Test Problem";' > src/main.rs
echo 'use cp_lib::your_new_feature;' >> src/main.rs

# 2. Run bundler
cargo run --bin ast_bundler

# 3. Verify bundled code compiles
cd bundled
rustc your_bundled_file.rs
./your_bundled_file
```

#### Common Bundler Issues and Solutions:

1. **Missing Dependencies**: Add explicit `use` statements in main.rs
2. **Macro Expansion**: Ensure macros are defined before usage in bundled file
3. **External Crates**: Consider if they're available in competitive programming environment
4. **File Paths**: Use relative imports from cp-lib root

### Example: Adding a New Data Structure

```rust
// cp-lib/src/data_structures/mod.rs
pub mod segment_tree;

// cp-lib/src/data_structures/segment_tree.rs
pub struct SegmentTree { /* implementation */ }

// cp-lib/src/lib.rs
pub mod data_structures;

// src/main.rs
const _PROBLEM: &str = "D. Range Queries";
use cp_lib::data_structures::segment_tree::SegmentTree;

fn main() {
    let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5]);
    // Use the segment tree
}
```

This ensures the bundler will:
1. Find the SegmentTree implementation
2. Include all necessary code in the bundled file
3. Generate proper filename: `d._range_queries_timestamp.rs`

### Verification Checklist
- [ ] Feature works in development (`cargo run`)
- [ ] Tests pass (`cargo test` in cp-lib)
- [ ] Bundler includes feature (`cargo run --bin ast_bundler`)
- [ ] Bundled code compiles (`rustc bundled_file.rs`)
- [ ] Bundled code produces correct output