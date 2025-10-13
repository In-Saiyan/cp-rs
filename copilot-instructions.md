# Copilot Instructions for CP-RS Project

## Development Guidelines

When adding new features to this competitive programming Rust project, follow these essential practices:

### 1. Bundler Integration
- **Critical**: If the feature is intended for use in competitive programming solutions, ensure it's properly handled by the AST bundler
- The bundler processes `src/main.rs` and resolves dependencies from `cp-lib/src/`
- Test that new modules/functions are correctly included in bundled output
- Update bundler logic in `bundler-lib/src/` if special handling is needed

### 2. Test Coverage Requirements
- **Every new feature MUST have corresponding test units**
- Place tests in the dedicated `cp-lib/tests/` folder structure
- Follow the existing pattern: `cp-lib/tests/{module}/{feature}.rs`
- Include comprehensive test cases covering:
  - Normal operation
  - Edge cases
  - Error conditions
  - Competitive programming scenarios

### 3. Project Structure
```
cp-lib/
├── src/           # Library implementation
└── tests/         # Test units (REQUIRED for all features)
bundler-lib/       # AST bundler (update if needed)
src/
├── main.rs        # Main solution file (bundled)
└── bin/           # Additional binaries
```

### 4. Testing Workflow
Before considering a feature complete:
1. Run `cargo test` in `cp-lib/` to verify all tests pass
2. Test bundling with `cargo run --bin ast_bundler`
3. Verify bundled code compiles and works correctly
4. Ensure no emojis or decorative characters in code

### 5. Competitive Programming Focus
- Prioritize performance and simplicity
- Use standard library when possible
- Keep external dependencies minimal
- Ensure all features work in single-file bundled output

## Examples

### Adding a new algorithm (e.g., segment tree):
1. Implement in `cp-lib/src/data_structures/segment_tree.rs`
2. Create tests in `cp-lib/tests/data_structures/segment_tree.rs`
3. Verify bundler includes it correctly when used in `main.rs`
4. Test performance with competitive programming constraints

### Adding I/O utilities:
1. Extend `cp-lib/src/io/` module
2. Add comprehensive tests in `cp-lib/tests/io/`
3. Ensure Scanner and other I/O work in bundled environment
4. Test with various input formats and edge cases

Remember: Every feature addition should maintain the project's goal of efficient competitive programming in Rust with reliable single-file bundling.