#!/bin/bash
# this_file: scripts/tag.sh

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

# Function to validate semantic version
validate_semver() {
    local version=$1
    if [[ $version =~ ^v?[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$ ]]; then
        return 0
    else
        return 1
    fi
}

# Function to check if tag already exists
check_tag_exists() {
    local tag=$1
    if git tag -l | grep -q "^${tag}$"; then
        return 0
    else
        return 1
    fi
}

# Function to update Cargo.toml version
update_cargo_version() {
    local version=$1
    local clean_version=$(echo "$version" | sed 's/^v//')
    
    print_info "Updating Cargo.toml version to $clean_version"
    
    # Use sed to update the version in Cargo.toml
    if command -v gsed >/dev/null 2>&1; then
        # macOS with GNU sed
        gsed -i "s/^version = \".*\"/version = \"$clean_version\"/" Cargo.toml
    else
        # Linux/Unix sed
        sed -i "s/^version = \".*\"/version = \"$clean_version\"/" Cargo.toml
    fi
    
    # Verify the change
    local new_version=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
    if [[ "$new_version" == "$clean_version" ]]; then
        print_success "Cargo.toml version updated to $clean_version"
    else
        print_error "Failed to update Cargo.toml version"
        return 1
    fi
}

# Function to generate tag message
generate_tag_message() {
    local version=$1
    local clean_version=$(echo "$version" | sed 's/^v//')
    
    echo "Release $version

This release includes:
- Version bump to $clean_version
- Automated build and release process
- Cross-platform binary support

For full changelog, see CHANGELOG.md"
}

# Function to create git tag
create_tag() {
    local version=$1
    local message=$(generate_tag_message "$version")
    
    print_info "Creating annotated tag: $version"
    
    # Create the annotated tag
    git tag -a "$version" -m "$message"
    
    print_success "Created tag: $version"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 <version>"
    echo ""
    echo "Examples:"
    echo "  $0 v1.0.6      # Create patch release"
    echo "  $0 v1.1.0      # Create minor release"
    echo "  $0 v2.0.0      # Create major release"
    echo "  $0 v1.0.0-rc1  # Create release candidate"
    echo ""
    echo "The script will:"
    echo "  1. Validate the version format"
    echo "  2. Check if the tag already exists"
    echo "  3. Update Cargo.toml version"
    echo "  4. Create an annotated git tag"
    echo "  5. Commit the Cargo.toml change"
}

# Main function
main() {
    # Check arguments
    if [[ $# -ne 1 ]]; then
        print_error "Invalid number of arguments"
        show_usage
        exit 1
    fi
    
    local version=$1
    
    # Ensure version starts with 'v'
    if [[ ! $version =~ ^v ]]; then
        version="v$version"
    fi
    
    # Validate version format
    if ! validate_semver "$version"; then
        print_error "Invalid semantic version format: $version"
        show_usage
        exit 1
    fi
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository"
        exit 1
    fi
    
    # Check if tag already exists
    if check_tag_exists "$version"; then
        print_error "Tag $version already exists"
        exit 1
    fi
    
    # Check if git is clean
    if [[ -n $(git status --porcelain) ]]; then
        print_warning "Git working directory is not clean"
        git status --short
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Aborted"
            exit 1
        fi
    fi
    
    print_info "Creating release $version"
    
    # Update Cargo.toml version
    update_cargo_version "$version"
    
    # Commit the version change
    git add Cargo.toml
    git commit -m "Bump version to $version"
    
    # Create the tag
    create_tag "$version"
    
    print_success "Release $version created successfully!"
    echo ""
    print_info "Next steps:"
    echo "  1. Review the changes: git show $version"
    echo "  2. Push the tag: git push origin $version"
    echo "  3. Push the commit: git push origin main"
    echo "  4. Create GitHub release: gh release create $version"
}

# Run main function
main "$@"