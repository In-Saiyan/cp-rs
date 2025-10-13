# Contributors

Thank you to everyone who has contributed to making cp-rs a better competitive programming library for Rust!

## How to Contribute

We welcome contributions! Please follow these guidelines:

### Before Contributing
1. Read [copilot-instructions.md](copilot-instructions.md) for development guidelines
2. Check [BUNDLER_INTEGRATION.md](BUNDLER_INTEGRATION.md) for bundler compatibility requirements
3. Review [docs/PULL_REQUESTS.md](docs/PULL_REQUESTS.md) for the contribution process
4. Understand testing requirements in [docs/TESTING.md](docs/TESTING.md)
5. Look at existing issues or create a new one to discuss your proposed changes

### Contribution Requirements
- **All new features must include test coverage** in `cp-lib/tests/`
- **Features for competitive programming must work with the bundler**
- Follow existing code style and patterns
- Ensure all tests pass (`cargo test` in cp-lib)
- Verify bundler integration (`cargo run --bin ast_bundler`)

### Types of Contributions Welcome
- **Algorithms**: Graph algorithms, number theory, data structures
- **I/O Utilities**: Enhanced input/output handling for competitive programming
- **Performance Optimizations**: Making existing code faster
- **Documentation**: Improving guides, examples, and code comments
- **Bug Fixes**: Fixing issues in existing functionality
- **Testing**: Adding more comprehensive test cases

### Pull Request Process
See [docs/PULL_REQUESTS.md](docs/PULL_REQUESTS.md) for detailed guidelines. Quick overview:

1. Fork the repository and create a feature branch from `main`
2. Implement your changes following the development guidelines
3. Add comprehensive tests (see [docs/TESTING.md](docs/TESTING.md))
4. Test performance with hyperfine (see [docs/PERFORMANCE_TESTING.md](docs/PERFORMANCE_TESTING.md))
5. Ensure bundler compatibility if applicable
6. Update documentation as needed
7. Submit a pull request with clear description following the template

### Code Style
- Use standard Rust formatting (`cargo fmt`)
- Follow competitive programming best practices (performance-focused)
- Keep external dependencies minimal
- Write clear, self-documenting code
- Add comments for complex algorithms

## Recognition

Contributors will be recognized here based on their contributions:

- **Algorithm Implementations**: Added new competitive programming algorithms
- **Infrastructure**: Improved bundler, testing, or build systems  
- **Documentation**: Enhanced guides, examples, or code documentation
- **Bug Fixes**: Fixed critical issues or edge cases
- **Testing**: Added comprehensive test coverage

## Documentation

Comprehensive documentation is available in the `docs/` folder:

- **[PULL_REQUESTS.md](docs/PULL_REQUESTS.md)** - Complete PR guidelines, requirements, and templates
- **[TESTING.md](docs/TESTING.md)** - Testing patterns, competitive programming test strategies
- **[PERFORMANCE_TESTING.md](docs/PERFORMANCE_TESTING.md)** - Benchmarking with hyperfine, optimization guidelines

## Contact

For questions about contributing, please:
- Check the documentation in `docs/` folder first
- Open an issue on GitHub for discussions
- Follow the pull request template for contributions
- Be specific about what you'd like to contribute

We appreciate your interest in making competitive programming in Rust more accessible and efficient!