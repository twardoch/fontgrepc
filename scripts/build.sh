#!/bin/bash
# this_file: scripts/build.sh

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

# Default values
BUILD_TYPE="release"
TARGET=""
FEATURES=""
VERBOSE=false
CLEAN=false

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -t, --target TARGET     Build for specific target (e.g., x86_64-unknown-linux-musl)"
    echo "  -d, --debug             Build in debug mode (default: release)"
    echo "  -f, --features FEATURES Comma-separated list of features to enable"
    echo "  -v, --verbose           Enable verbose output"
    echo "  -c, --clean             Clean before building"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Release build"
    echo "  $0 --debug                           # Debug build"
    echo "  $0 --target x86_64-unknown-linux-musl  # Static musl build"
    echo "  $0 --clean --verbose                 # Clean verbose build"
}

# Function to parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -t|--target)
                TARGET="$2"
                shift 2
                ;;
            -d|--debug)
                BUILD_TYPE="debug"
                shift
                ;;
            -f|--features)
                FEATURES="$2"
                shift 2
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -c|--clean)
                CLEAN=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
}

# Function to check prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."
    
    # Check if cargo is installed
    if ! command -v cargo >/dev/null 2>&1; then
        print_error "Cargo is not installed"
        exit 1
    fi
    
    # Check if we're in a Rust project
    if [[ ! -f Cargo.toml ]]; then
        print_error "Not in a Rust project directory"
        exit 1
    fi
    
    # Check if target is installed (if specified)
    if [[ -n "$TARGET" ]]; then
        if ! rustup target list --installed | grep -q "^$TARGET$"; then
            print_warning "Target $TARGET is not installed"
            read -p "Install target $TARGET? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                rustup target add "$TARGET"
            else
                print_error "Target $TARGET is required but not installed"
                exit 1
            fi
        fi
    fi
    
    print_success "Prerequisites OK"
}

# Function to get version information
get_version_info() {
    print_info "Getting version information..."
    
    # Try to get version from git
    if git rev-parse --git-dir > /dev/null 2>&1; then
        local git_version=$(git describe --tags --always --dirty 2>/dev/null || echo "unknown")
        print_info "Git version: $git_version"
    fi
    
    # Get version from Cargo.toml
    local cargo_version=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
    print_info "Cargo version: $cargo_version"
}

# Function to clean build artifacts
clean_build() {
    if [[ "$CLEAN" == true ]]; then
        print_info "Cleaning build artifacts..."
        cargo clean
        print_success "Build artifacts cleaned"
    fi
}

# Function to build the project
build_project() {
    print_info "Building fontgrepc..."
    
    # Construct cargo build command
    local cargo_cmd="cargo build"
    
    # Add build type
    if [[ "$BUILD_TYPE" == "release" ]]; then
        cargo_cmd="$cargo_cmd --release"
    fi
    
    # Add target if specified
    if [[ -n "$TARGET" ]]; then
        cargo_cmd="$cargo_cmd --target $TARGET"
    fi
    
    # Add features if specified
    if [[ -n "$FEATURES" ]]; then
        cargo_cmd="$cargo_cmd --features $FEATURES"
    fi
    
    # Add verbose flag if requested
    if [[ "$VERBOSE" == true ]]; then
        cargo_cmd="$cargo_cmd --verbose"
    fi
    
    print_info "Running: $cargo_cmd"
    
    # Execute the build
    if eval "$cargo_cmd"; then
        print_success "Build completed successfully"
    else
        print_error "Build failed"
        exit 1
    fi
}

# Function to show build artifacts
show_artifacts() {
    print_info "Build artifacts:"
    
    local target_dir="target"
    if [[ -n "$TARGET" ]]; then
        target_dir="$target_dir/$TARGET"
    fi
    
    local build_dir="$target_dir/$BUILD_TYPE"
    
    if [[ -d "$build_dir" ]]; then
        echo ""
        echo "Binary location:"
        if [[ -f "$build_dir/fontgrepc" ]]; then
            echo "  $build_dir/fontgrepc"
            
            # Show binary size
            local size=$(du -h "$build_dir/fontgrepc" | cut -f1)
            echo "  Size: $size"
            
            # Show binary info if file command is available
            if command -v file >/dev/null 2>&1; then
                echo "  Type: $(file "$build_dir/fontgrepc" | cut -d: -f2- | xargs)"
            fi
        elif [[ -f "$build_dir/fontgrepc.exe" ]]; then
            echo "  $build_dir/fontgrepc.exe"
            
            # Show binary size
            local size=$(du -h "$build_dir/fontgrepc.exe" | cut -f1)
            echo "  Size: $size"
        else
            print_warning "Binary not found in expected location"
        fi
    else
        print_warning "Build directory not found: $build_dir"
    fi
}

# Function to run quick tests
run_tests() {
    print_info "Running quick tests..."
    
    # Build the binary first (if not already built)
    local target_dir="target"
    if [[ -n "$TARGET" ]]; then
        target_dir="$target_dir/$TARGET"
    fi
    
    local build_dir="$target_dir/$BUILD_TYPE"
    local binary="$build_dir/fontgrepc"
    
    if [[ ! -f "$binary" ]]; then
        print_error "Binary not found: $binary"
        return 1
    fi
    
    # Test help command
    if "$binary" --help >/dev/null 2>&1; then
        print_success "Help command works"
    else
        print_error "Help command failed"
        return 1
    fi
    
    # Test version command
    if "$binary" --version >/dev/null 2>&1; then
        print_success "Version command works"
    else
        print_error "Version command failed"
        return 1
    fi
    
    print_success "Quick tests passed"
}

# Main function
main() {
    print_info "fontgrepc Build Script"
    echo "======================"
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check prerequisites
    check_prerequisites
    
    # Show version information
    get_version_info
    
    # Clean if requested
    clean_build
    
    # Build the project
    build_project
    
    # Show build artifacts
    show_artifacts
    
    # Run quick tests
    run_tests
    
    echo ""
    print_success "Build completed successfully!"
    
    # Show next steps
    echo ""
    print_info "Next steps:"
    echo "  1. Test the binary: ./target/$BUILD_TYPE/fontgrepc --help"
    echo "  2. Run full tests: cargo test"
    echo "  3. Install locally: cargo install --path ."
}

# Run main function
main "$@"