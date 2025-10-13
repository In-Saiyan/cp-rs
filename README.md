# cp-rs
A library that makes rust easy to use for competitive programmers.

## Development Guidelines

When contributing to this project, please follow these important practices:

### Feature Development Rules
1. **Bundler Integration**: If adding features for competitive programming use, ensure they work with the AST bundler
2. **Test Coverage**: All new features MUST have comprehensive test units in `cp-lib/tests/`
3. **Testing Workflow**: Run tests and verify bundling before considering features complete

See [.copilot-instructions.md](.copilot-instructions.md) and [BUNDLER_INTEGRATION.md](BUNDLER_INTEGRATION.md) for detailed guidelines.

## Quick Start

### Development
```bash
# Run tests
cd cp-lib && cargo test

# Bundle for submission
cargo run --bin ast_bundler

# Use the convenient script
./bundle.sh
```

### Project Structure
- `cp-lib/` - Core competitive programming library
- `cp-lib/tests/` - Test units (required for all features)
- `bundler-lib/` - AST-based code bundler
- `src/main.rs` - Main solution file
- `bundled/` - Generated single-file submissions
