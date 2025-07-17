#!/bin/bash
# this_file: scripts/release.sh

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Release configuration
DRY_RUN=false
SKIP_TESTS=false
SKIP_BUILD=false
SKIP_PUBLISH=false
VERSION=""
CHANGELOG_FILE="CHANGELOG.md"

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS] [VERSION]"
    echo ""
    echo "Options:"
    echo "  -d, --dry-run           Show what would be done without executing"
    echo "  --skip-tests            Skip test execution"
    echo "  --skip-build            Skip build process"
    echo "  --skip-publish          Skip publishing to crates.io"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "Arguments:"
    echo "  VERSION                 Version to release (e.g., v1.0.6, 1.0.6)"
    echo ""
    echo "Examples:"
    echo "  $0                      # Interactive release"
    echo "  $0 v1.0.6               # Release specific version"
    echo "  $0 --dry-run v1.0.6     # Show what would happen"
    echo "  $0 --skip-tests v1.0.6  # Skip test execution"
}

# Function to parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -d|--dry-run)
                DRY_RUN=true
                shift
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --skip-build)
                SKIP_BUILD=true
                shift
                ;;
            --skip-publish)
                SKIP_PUBLISH=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            -*)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
            *)
                if [[ -z "$VERSION" ]]; then
                    VERSION="$1"
                else
                    print_error "Too many arguments"
                    show_usage
                    exit 1
                fi
                shift
                ;;
        esac
    done
}

# Function to validate prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository"
        exit 1
    fi
    
    # Check if we're in a Rust project
    if [[ ! -f Cargo.toml ]]; then
        print_error "Not in a Rust project directory"
        exit 1
    fi
    
    # Check for required tools
    local missing_tools=()
    
    if ! command -v cargo >/dev/null 2>&1; then
        missing_tools+=("cargo")
    fi
    
    if ! command -v git >/dev/null 2>&1; then
        missing_tools+=("git")
    fi
    
    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        print_error "Missing required tools: ${missing_tools[*]}"
        exit 1
    fi
    
    print_success "Prerequisites OK"
}

# Function to validate version format
validate_version() {
    local version=$1
    
    # Ensure version starts with 'v'
    if [[ ! $version =~ ^v ]]; then
        version="v$version"
    fi
    
    # Validate semantic version format
    if [[ ! $version =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$ ]]; then
        print_error "Invalid semantic version format: $version"
        return 1
    fi
    
    echo "$version"
}

# Function to get current version
get_current_version() {
    if git describe --tags --exact-match HEAD 2>/dev/null; then
        return 0
    fi
    
    local latest_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
    echo "$latest_tag"
}

# Function to suggest next version
suggest_next_version() {
    local current_version=$(get_current_version | sed 's/^v//')
    local major minor patch
    
    # Extract version parts
    if [[ $current_version =~ ^([0-9]+)\.([0-9]+)\.([0-9]+) ]]; then
        major=${BASH_REMATCH[1]}
        minor=${BASH_REMATCH[2]}
        patch=${BASH_REMATCH[3]}
    else
        print_error "Cannot parse current version: $current_version"
        return 1
    fi
    
    echo "Current version: v$current_version"
    echo ""
    echo "Suggested next versions:"
    echo "  1) Patch: v$major.$minor.$((patch + 1))"
    echo "  2) Minor: v$major.$((minor + 1)).0"
    echo "  3) Major: v$((major + 1)).0.0"
    echo ""
    
    local choice
    read -p "Select version type (1-3) or enter custom version: " choice
    
    case $choice in
        1)
            echo "v$major.$minor.$((patch + 1))"
            ;;
        2)
            echo "v$major.$((minor + 1)).0"
            ;;
        3)
            echo "v$((major + 1)).0.0"
            ;;
        *)
            echo "$choice"
            ;;
    esac
}

# Function to check git status
check_git_status() {
    print_info "Checking git status..."
    
    # Check if git is clean
    if [[ -n $(git status --porcelain) ]]; then
        print_warning "Git working directory is not clean"
        git status --short
        
        if [[ "$DRY_RUN" == false ]]; then
            read -p "Continue anyway? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                print_info "Aborted"
                exit 1
            fi
        fi
    fi
    
    # Check if we're on main branch
    local current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$current_branch" != "main" ]]; then
        print_warning "Not on main branch (current: $current_branch)"
        
        if [[ "$DRY_RUN" == false ]]; then
            read -p "Continue anyway? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                print_info "Aborted"
                exit 1
            fi
        fi
    fi
    
    print_success "Git status OK"
}

# Function to update changelog
update_changelog() {
    local version=$1
    local clean_version=$(echo "$version" | sed 's/^v//')
    
    print_info "Updating changelog..."
    
    if [[ ! -f "$CHANGELOG_FILE" ]]; then
        print_warning "Changelog file not found: $CHANGELOG_FILE"
        print_info "Creating new changelog..."
        
        if [[ "$DRY_RUN" == false ]]; then
            cat > "$CHANGELOG_FILE" << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [$clean_version] - $(date +%Y-%m-%d)

### Added
- Automated release process
- Git-tag-based versioning
- Comprehensive test suite
- Cross-platform build support

### Changed
- Updated build system
- Improved CI/CD pipeline

### Fixed
- Various bug fixes and improvements
EOF
        fi
    else
        print_info "Please update $CHANGELOG_FILE with release notes for $version"
        
        if [[ "$DRY_RUN" == false ]]; then
            read -p "Press Enter when changelog is updated..."
        fi
    fi
    
    print_success "Changelog updated"
}

# Function to run tests
run_tests() {
    if [[ "$SKIP_TESTS" == true ]]; then
        print_info "Skipping tests"
        return 0
    fi
    
    print_info "Running tests..."
    
    if [[ "$DRY_RUN" == false ]]; then
        if [[ -f "scripts/test.sh" ]]; then
            chmod +x scripts/test.sh
            ./scripts/test.sh
        else
            cargo test --verbose
            cargo clippy --all-targets --all-features -- -D warnings
            cargo fmt --all -- --check
        fi
    else
        print_info "Would run: ./scripts/test.sh"
    fi
    
    print_success "Tests completed"
}

# Function to build release
build_release() {
    if [[ "$SKIP_BUILD" == true ]]; then
        print_info "Skipping build"
        return 0
    fi
    
    print_info "Building release..."
    
    if [[ "$DRY_RUN" == false ]]; then
        if [[ -f "scripts/build.sh" ]]; then
            chmod +x scripts/build.sh
            ./scripts/build.sh
        else
            cargo build --release
        fi
    else
        print_info "Would run: ./scripts/build.sh"
    fi
    
    print_success "Build completed"
}

# Function to create git tag
create_git_tag() {
    local version=$1
    
    print_info "Creating git tag: $version"
    
    if [[ "$DRY_RUN" == false ]]; then
        if [[ -f "scripts/tag.sh" ]]; then
            chmod +x scripts/tag.sh
            ./scripts/tag.sh "$version"
        else
            # Manual tag creation
            local clean_version=$(echo "$version" | sed 's/^v//')
            
            # Update Cargo.toml
            sed -i "s/^version = \".*\"/version = \"$clean_version\"/" Cargo.toml
            
            # Commit and tag
            git add Cargo.toml
            git commit -m "Bump version to $version"
            git tag -a "$version" -m "Release $version"
        fi
    else
        print_info "Would run: ./scripts/tag.sh $version"
    fi
    
    print_success "Git tag created"
}

# Function to publish to crates.io
publish_crate() {
    if [[ "$SKIP_PUBLISH" == true ]]; then
        print_info "Skipping crates.io publish"
        return 0
    fi
    
    print_info "Publishing to crates.io..."
    
    if [[ "$DRY_RUN" == false ]]; then
        read -p "Publish to crates.io? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            cargo publish
        else
            print_info "Skipping crates.io publish"
        fi
    else
        print_info "Would run: cargo publish"
    fi
    
    print_success "Crates.io publish completed"
}

# Function to push to repository
push_release() {
    local version=$1
    
    print_info "Pushing release to repository..."
    
    if [[ "$DRY_RUN" == false ]]; then
        print_info "Pushing changes and tags..."
        git push origin main
        git push origin "$version"
        
        # Create GitHub release if gh is available
        if command -v gh >/dev/null 2>&1; then
            print_info "Creating GitHub release..."
            gh release create "$version" --generate-notes
        else
            print_info "GitHub CLI not available, skipping release creation"
        fi
    else
        print_info "Would run: git push origin main"
        print_info "Would run: git push origin $version"
        print_info "Would run: gh release create $version --generate-notes"
    fi
    
    print_success "Release pushed"
}

# Main function
main() {
    print_info "fontgrepc Release Script"
    echo "========================"
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check prerequisites
    check_prerequisites
    
    # Get version if not provided
    if [[ -z "$VERSION" ]]; then
        print_info "No version specified, starting interactive mode..."
        VERSION=$(suggest_next_version)
    fi
    
    # Validate version
    VERSION=$(validate_version "$VERSION")
    
    print_info "Releasing version: $VERSION"
    
    if [[ "$DRY_RUN" == true ]]; then
        print_warning "DRY RUN MODE - No changes will be made"
    fi
    
    # Check git status
    check_git_status
    
    # Update changelog
    update_changelog "$VERSION"
    
    # Run tests
    run_tests
    
    # Build release
    build_release
    
    # Create git tag
    create_git_tag "$VERSION"
    
    # Publish to crates.io
    publish_crate
    
    # Push to repository
    push_release "$VERSION"
    
    echo ""
    print_success "Release $VERSION completed successfully!"
    
    echo ""
    print_info "Next steps:"
    echo "  1. Verify the release on GitHub"
    echo "  2. Test the published crate: cargo install fontgrepc"
    echo "  3. Update documentation if needed"
    echo "  4. Announce the release"
}

# Run main function
main "$@"