# GitHub Workflows Setup

This directory contains GitHub Actions workflows for automated CI/CD. Due to GitHub App permissions, these files need to be manually moved to the correct location.

## Setup Instructions

1. **Create the workflows directory** in your repository:
   ```bash
   mkdir -p .github/workflows
   ```

2. **Move the workflow files**:
   ```bash
   mv github-workflows/*.yml .github/workflows/
   ```

3. **Remove this temporary directory**:
   ```bash
   rm -rf github-workflows
   ```

## Workflow Files

- **`ci.yml`** - Continuous Integration pipeline
  - Runs on push to main and pull requests
  - Multi-platform testing (Ubuntu, Windows, macOS)
  - Multiple Rust versions (stable, beta, nightly)
  - Code formatting, linting, and security auditing
  - Cross-compilation testing

- **`release.yml`** - Automated releases
  - Triggered on git tags matching `v*.*.*`
  - Multi-platform binary generation
  - Automated GitHub releases
  - Crates.io publishing
  - Checksum generation

- **`nightly.yml`** - Nightly builds and extended testing
  - Runs daily at 3 AM UTC
  - Extended test suite
  - Dependency update checking
  - Performance regression testing
  - Memory leak detection

- **`pr.yml`** - Pull request validation
  - Fast feedback on pull requests
  - Code quality checks
  - Version consistency validation

## Required GitHub Secrets

For the workflows to function properly, you'll need to configure these secrets in your GitHub repository settings:

1. **`CRATES_IO_TOKEN`** - For publishing to crates.io
   - Go to https://crates.io/me
   - Generate a new token with publish permissions
   - Add it as a repository secret

2. **`CODECOV_TOKEN`** (optional) - For code coverage reporting
   - Sign up at https://codecov.io
   - Add your repository
   - Copy the token and add it as a repository secret

## Usage

Once set up, the workflows will automatically:

1. **Run tests** on every push and pull request
2. **Create releases** when you push a git tag (e.g., `v1.0.6`)
3. **Publish to crates.io** when a release is created
4. **Generate binaries** for multiple platforms
5. **Run nightly checks** for maintenance

## Creating a Release

To create a new release:

1. **Use the release script**:
   ```bash
   ./scripts/release.sh v1.0.6
   ```

2. **Or manually**:
   ```bash
   # Update version in Cargo.toml
   # Update CHANGELOG.md
   git add .
   git commit -m "Bump version to v1.0.6"
   git tag -a v1.0.6 -m "Release v1.0.6"
   git push origin main
   git push origin v1.0.6
   ```

The release workflow will automatically create a GitHub release with binaries for all supported platforms.