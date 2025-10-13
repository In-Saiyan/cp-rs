# Pull Request Guidelines

This document outlines the process and requirements for contributing to cp-rs via pull requests.

## Before Creating a Pull Request

### 1. Check Existing Issues
- Look through open issues to see if your contribution addresses an existing need
- If no relevant issue exists, consider creating one to discuss your proposed changes
- Reference the issue number in your PR description

### 2. Read Documentation
- Review [copilot-instructions.md](../copilot-instructions.md) for development guidelines
- Check [BUNDLER_INTEGRATION.md](../BUNDLER_INTEGRATION.md) for bundler compatibility requirements
- Understand the project's competitive programming focus

## Pull Request Requirements

### Essential Requirements ✅
- [ ] **All new features have comprehensive tests** in `cp-lib/tests/`
- [ ] **Features meant for CP work with the bundler** (test with `cargo run --bin ast_bundler`)
- [ ] All existing tests pass (`cargo test` in `cp-lib/`)
- [ ] Code follows Rust conventions (`cargo fmt`)
- [ ] No EXTRA compiler warnings (`cargo clippy`), If you aren't able to fix the warnings ask the maintainer(s)/collaborator(s) about it

### Documentation Requirements
- [ ] Update relevant documentation if adding new features
- [ ] Add code comments for complex algorithms
- [ ] Include examples in docstrings for public functions
- [ ] Update README.md if changing core functionality

## PR Creation Process

### 1. Fork and Branch
```bash
# Fork the repository on GitHub
git clone https://github.com/yourusername/cp-rs.git
cd cp-rs

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Development Workflow
```bash
# Make your changes
# ... edit files ...

# Run tests frequently during development
cd cp-lib && cargo test

# Test bundler integration if applicable
cd .. && cargo run --bin ast_bundler

# Verify bundled code works
cd bundled && rustc solution_*.rs && ./solution_*
```

### 3. Pre-submission Checklist
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run all tests
cd cp-lib && cargo test

# Test bundler (if your changes affect CP code)
cd .. && cargo run --bin ast_bundler

# Verify performance if applicable (see PERFORMANCE_TESTING.md)
hyperfine './target/release/your_feature'
```

## PR Structure

### Title Format
- **Feature**: `feat: add segment tree implementation`
- **Bug Fix**: `fix: resolve scanner buffer overflow`
- **Documentation**: `docs: update algorithm complexity notes`
- **Performance**: `perf: optimize graph traversal algorithms`
- **Tests**: `test: add comprehensive edge cases for scanner`

### Description Template
```markdown
## Description
Brief description of what this PR does.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Test coverage improvement

## Testing
- [ ] All existing tests pass
- [ ] Added comprehensive test coverage for new functionality
- [ ] Tested bundler integration (if applicable)
- [ ] Performance tested with realistic inputs (if applicable)

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my own code
- [ ] I have commented complex algorithms appropriately
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] New and existing tests pass locally
- [ ] Bundler integration works (if applicable)

## Related Issues
Fixes #(issue_number)
Related to #(issue_number)

## Additional Notes
Any additional information reviewers should know.
```

## Review Process

### What Reviewers Look For
1. **Code Quality**
   - Follows Rust best practices
   - Appropriate use of data structures for competitive programming
   - Clear, readable code with good naming

2. **Performance Considerations**
   - Algorithms are efficient for competitive programming constraints
   - No unnecessary allocations or inefficient operations
   - Appropriate time/space complexity

3. **Test Coverage**
   - Comprehensive test cases including edge cases
   - Tests follow existing patterns in `cp-lib/tests/`
   - Tests verify both correctness and performance characteristics

4. **Bundler Compatibility**
   - Features work correctly in bundled single-file output
   - No issues with module resolution or dependencies
   - Bundled code compiles and runs correctly

### Common Review Feedback
- **Missing tests**: Add comprehensive test coverage
- **Performance concerns**: Optimize for competitive programming constraints  
- **Bundler issues**: Fix compatibility with single-file bundling
- **Documentation**: Add or improve code comments and docs
- **Style**: Follow consistent formatting and naming conventions

## Merging Process

### Requirements for Merge
1. All automated checks pass (tests, linting, formatting)
2. At least one approving review from a maintainer
3. All conversations resolved
4. Branch is up to date with main

### After Merge
- Feature branches are automatically deleted
- Changes will be available in the next release
- Contributors will be recognized in CONTRIBUTORS.md

## Special Considerations for Competitive Programming

### Algorithm Implementations
- Include time/space complexity in comments
- Test with realistic competitive programming constraints
- Provide usage examples in tests
- Ensure algorithms work with Scanner input format

### I/O Utilities
- Must work with both interactive and batch input
- Should handle edge cases gracefully
- Performance should be suitable for large inputs
- Must be compatible with bundled single-file output

### Data Structures
- Implement common competitive programming operations efficiently
- Include clear documentation of supported operations
- Test with large datasets when appropriate
- Ensure compatibility with standard competitive programming patterns

## Getting Help

If you need assistance:
1. Check existing documentation first
2. Look at similar implementations in the codebase
3. Create a draft PR to get early feedback
4. Ask questions in the PR description or comments
5. Reference specific parts of the code you're uncertain about

Remember: We're here to help make competitive programming in Rust better for everyone, don't shy out from asking questions we're all here to help!