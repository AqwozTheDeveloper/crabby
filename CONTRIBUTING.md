# Contributing to Crabby

Thank you for your interest in contributing to Crabby! 🦀

## Getting Started

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/crabby.git
   cd crabby
   ```
3. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git

### Building from Source
```bash
cargo build
cargo run -- --help
```

### Running Tests
```bash
cargo test
```

### Release Build
```bash
cargo build --release
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for lints: `cargo clippy`
- Write clear commit messages
- Add tests for new features

## Pull Request Process

1. **Update documentation** if you're adding features
2. **Add tests** for new functionality
3. **Run `cargo fmt` and `cargo clippy`**
4. **Update RELEASE_NOTES.md** with your changes
5. **Submit PR** with clear description

## Areas for Contribution

### High Priority
- [ ] Parallel package downloads
- [ ] Full workspace support
- [ ] Security audit integration
- [ ] Performance benchmarks
- [ ] More comprehensive tests

### Medium Priority
- [ ] Watch mode for development
- [ ] Publishing to npm
- [ ] Better error messages
- [ ] Progress bars for downloads
- [ ] Offline mode

### Low Priority
- [ ] Plugin system
- [ ] Custom commands
- [ ] Shell completions
- [ ] Man pages

## Reporting Bugs

Use GitHub Issues with:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version)

## Feature Requests

Open an issue with:
- Use case description
- Proposed solution
- Alternative approaches considered

## Code of Conduct

Be respectful, inclusive, and constructive.

## Questions?

Open a discussion on GitHub or reach out to the maintainers.

---

**Thank you for contributing! 🙏**
