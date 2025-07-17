#!/bin/bash
# this_file: scripts/version.sh

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

# Function to get current version from git
get_current_version() {
    if git describe --tags --exact-match HEAD 2>/dev/null; then
        return 0
    fi
    
    local latest_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
    local commit_count=$(git rev-list --count ${latest_tag}..HEAD 2>/dev/null || echo "0")
    local commit_hash=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    
    if [[ $commit_count -gt 0 ]]; then
        echo "${latest_tag}-${commit_count}-g${commit_hash}"
    else
        echo "$latest_tag"
    fi
}

# Function to get version from Cargo.toml
get_cargo_version() {
    grep '^version' Cargo.toml | head -1 | cut -d'"' -f2
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
    
    print_info "Current version: v$current_version"
    echo
    print_info "Suggested next versions:"
    echo "  Patch: v$major.$minor.$((patch + 1))"
    echo "  Minor: v$major.$((minor + 1)).0"
    echo "  Major: v$((major + 1)).0.0"
}

# Function to check if git is clean
check_git_clean() {
    if [[ -n $(git status --porcelain) ]]; then
        print_warning "Git working directory is not clean"
        git status --short
        return 1
    fi
    return 0
}

# Function to check if we're on main branch
check_main_branch() {
    local current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$current_branch" != "main" ]]; then
        print_warning "Not on main branch (current: $current_branch)"
        return 1
    fi
    return 0
}

# Main function
main() {
    print_info "fontgrepc Version Information"
    echo "=================================="
    
    # Check if we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "Not in a git repository"
        exit 1
    fi
    
    # Get current version information
    local git_version=$(get_current_version)
    local cargo_version=$(get_cargo_version)
    
    print_info "Git version: $git_version"
    print_info "Cargo.toml version: v$cargo_version"
    
    # Check if versions are in sync
    local git_clean_version=$(echo "$git_version" | sed 's/^v//' | sed 's/-.*$//')
    if [[ "$git_clean_version" != "$cargo_version" ]]; then
        print_warning "Git and Cargo versions are out of sync!"
    else
        print_success "Git and Cargo versions are in sync"
    fi
    
    echo
    
    # Check git status
    if check_git_clean; then
        print_success "Git working directory is clean"
    fi
    
    # Check branch
    if check_main_branch; then
        print_success "On main branch"
    fi
    
    echo
    
    # Suggest next version
    suggest_next_version
    
    echo
    print_info "To create a new version:"
    echo "  1. Update CHANGELOG.md"
    echo "  2. Run: ./scripts/tag.sh <version>"
    echo "  3. Run: git push --tags"
}

# Run main function
main "$@"