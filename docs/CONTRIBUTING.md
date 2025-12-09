# Contributing to TDF

Thank you for your interest in contributing to the TrustDoc Financial (TDF) format project!

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Contribution Workflow](#contribution-workflow)
5. [Coding Standards](#coding-standards)
6. [Testing Requirements](#testing-requirements)
7. [Documentation](#documentation)
8. [Security Considerations](#security-considerations)
9. [Review Process](#review-process)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow project guidelines

## Getting Started

### Prerequisites

- Rust 1.70+ (for Rust components)
- Node.js 20+ (for TypeScript/JavaScript components)
- Git
- Basic understanding of cryptographic concepts

### First Contribution

1. **Fork the repository**
2. **Clone your fork**: `git clone <your-fork-url>`
3. **Create a branch**: `git checkout -b feature/my-feature`
4. **Make changes**
5. **Test your changes**: `cargo test --workspace`
6. **Commit**: `git commit -m "Add feature X"`
7. **Push**: `git push origin feature/my-feature`
8. **Create Pull Request**

## Development Setup

### Initial Setup

```bash
# Clone repository
git clone <repository-url>
cd TrustDoc

# Build workspace
cargo build --workspace

# Install TypeScript dependencies
cd tdf-ts && npm install && cd ..
cd tdf-viewer && npm install && cd ..
cd tdf-wasm && wasm-pack build --target web && cd ..

# Run tests
cargo test --workspace
```

### Development Tools

Recommended tools:

- **rustfmt**: Code formatting
- **clippy**: Linting
- **cargo test**: Testing
- **cargo bench**: Benchmarking

Install:

```bash
rustup component add rustfmt clippy
```

## Contribution Workflow

### 1. Choose an Issue

- Check [open issues](https://github.com/trustdoc/tdf/issues)
- Look for "good first issue" labels
- Comment on the issue to claim it

### 2. Create a Branch

```bash
git checkout -b feature/issue-number-description
# or
git checkout -b fix/issue-number-description
```

### 3. Make Changes

- Write clean, documented code
- Follow existing patterns
- Add tests for new features
- Update documentation

### 4. Test Your Changes

```bash
# Run all tests
cargo test --workspace

# Run specific tests
cargo test -p tdf-core

# Run security tests
cargo test -p tdf-core --test security_tests
cargo test -p tdf-core --test e2e_security_tests

# Check formatting
cargo fmt --check

# Run linter
cargo clippy --workspace -- -D warnings
```

### 5. Commit Changes

Follow conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `security`: Security fix

Examples:

```
feat(core): Add support for custom hash algorithms

docs(api): Update API documentation for ArchiveBuilder

fix(verification): Fix signature verification edge case
```

### 6. Push and Create PR

```bash
git push origin feature/my-feature
```

Then create a Pull Request on GitHub.

## Coding Standards

### Rust Code

- Follow `rustfmt` defaults
- Use `clippy` recommendations
- Document all public APIs
- Use meaningful variable names
- Keep functions focused and small
- Handle errors explicitly

Example:

```rust
/// Creates a new document with the given title and content.
///
/// # Arguments
///
/// * `title` - Document title
/// * `language` - ISO 639-1 language code
/// * `content` - Document content
///
/// # Returns
///
/// A new `Document` instance.
///
/// # Errors
///
/// Returns `TdfError::InvalidDocument` if validation fails.
pub fn new(title: String, language: String, content: DocumentContent) -> TdfResult<Document> {
    // Implementation
}
```

### TypeScript Code

- Use TypeScript strict mode
- Follow ESLint rules
- Document functions with JSDoc
- Use async/await for async operations

Example:

```typescript
/**
 * Loads a TDF document from a file.
 * @param file - The file to load
 * @returns Promise resolving to the loaded document
 * @throws {TdfError} If the file is invalid
 */
async function loadDocument(file: File): Promise<Document> {
    // Implementation
}
```

### File Organization

- One module per file
- Group related functionality
- Keep file sizes reasonable (< 1000 lines)
- Use subdirectories for large modules

## Testing Requirements

### Test Coverage

- Aim for 80%+ code coverage
- Test all public APIs
- Include edge cases and error conditions
- Test security-critical code thoroughly

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_happy_path() {
        // Test normal operation
    }

    #[test]
    fn test_feature_edge_case() {
        // Test edge cases
    }

    #[test]
    fn test_feature_error_handling() {
        // Test error conditions
    }
}
```

### Security Tests

All security-related changes must include:

1. **Unit tests**: Test individual components
2. **Integration tests**: Test full workflows
3. **Attack tests**: Test against known attack vectors

Example:

```rust
#[test]
fn test_tampering_detection() {
    // Create document
    // Tamper with content
    // Verify detection
}
```

## Documentation

### Code Documentation

- Document all public APIs
- Include examples in doc comments
- Explain complex algorithms
- Document error conditions

### User Documentation

- Update relevant guides in `docs/`
- Add examples if adding features
- Update API reference if changing APIs
- Update SPEC.md if changing format

### Documentation Format

Use Markdown for documentation:

- Clear headings
- Code examples
- Tables for structured data
- Diagrams for complex concepts

## Security Considerations

### Security Review

All changes affecting security must:

1. Include security tests
2. Be reviewed by security team
3. Document threat model changes
4. Update security documentation

### Security-Critical Areas

- Cryptographic operations
- Signature verification
- Integrity checking
- Key management
- Input validation

### Reporting Security Issues

**DO NOT** open a public issue for security vulnerabilities.

Instead, email: `security@trustdoc.org`

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Review Process

### Pull Request Requirements

1. **All tests pass**: CI must be green
2. **Code formatted**: `cargo fmt` must pass
3. **No linter warnings**: `clippy` must pass
4. **Documentation updated**: Relevant docs updated
5. **Security reviewed**: If security-related

### Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Security implications considered
- [ ] Performance impact assessed

### Review Feedback

- Address all review comments
- Ask questions if unclear
- Be open to suggestions
- Update PR based on feedback

## Areas for Contribution

### High Priority

- Performance improvements
- Security enhancements
- Documentation improvements
- Test coverage
- Bug fixes

### Medium Priority

- New features (discuss first)
- UI/UX improvements
- Developer experience
- Tooling improvements

### Low Priority

- Code cleanup
- Refactoring
- Examples and demos

## Getting Help

- **Documentation**: Check `docs/` directory
- **Issues**: Search existing issues
- **Discussions**: Use GitHub Discussions
- **Email**: `dev@trustdoc.org`

## Recognition

Contributors will be:

- Listed in CONTRIBUTORS.md
- Credited in release notes
- Acknowledged in documentation

Thank you for contributing to TDF! 🎉

---

*Last updated: 2025-12-09*

