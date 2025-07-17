#!/bin/bash
# this_file: scripts/test.sh

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

# Test configuration
VERBOSE=false
COVERAGE=false
BENCH=false
INTEGRATION=false
UNIT=true
CLIPPY=true
FORMAT=true
AUDIT=false

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -v, --verbose           Enable verbose output"
    echo "  -c, --coverage          Run tests with coverage"
    echo "  -b, --bench             Run benchmarks"
    echo "  -i, --integration       Run integration tests only"
    echo "  -u, --unit              Run unit tests only (default: true)"
    echo "  --no-clippy             Skip clippy linting"
    echo "  --no-format             Skip format checking"
    echo "  -a, --audit             Run security audit"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                      # Run all default tests"
    echo "  $0 --verbose --coverage # Run with coverage"
    echo "  $0 --integration        # Run integration tests only"
    echo "  $0 --bench              # Run benchmarks"
}

# Function to parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -c|--coverage)
                COVERAGE=true
                shift
                ;;
            -b|--bench)
                BENCH=true
                shift
                ;;
            -i|--integration)
                INTEGRATION=true
                UNIT=false
                shift
                ;;
            -u|--unit)
                UNIT=true
                INTEGRATION=false
                shift
                ;;
            --no-clippy)
                CLIPPY=false
                shift
                ;;
            --no-format)
                FORMAT=false
                shift
                ;;
            -a|--audit)
                AUDIT=true
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
    
    # Check for coverage tools if requested
    if [[ "$COVERAGE" == true ]]; then
        if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
            print_warning "cargo-tarpaulin not found, installing..."
            cargo install cargo-tarpaulin
        fi
    fi
    
    # Check for audit tool if requested
    if [[ "$AUDIT" == true ]]; then
        if ! command -v cargo-audit >/dev/null 2>&1; then
            print_warning "cargo-audit not found, installing..."
            cargo install cargo-audit
        fi
    fi
    
    print_success "Prerequisites OK"
}

# Function to run format check
run_format_check() {
    if [[ "$FORMAT" == true ]]; then
        print_info "Running format check..."
        
        if cargo fmt --all -- --check; then
            print_success "Format check passed"
        else
            print_error "Format check failed"
            print_info "Run 'cargo fmt --all' to fix formatting"
            return 1
        fi
    fi
}

# Function to run clippy
run_clippy() {
    if [[ "$CLIPPY" == true ]]; then
        print_info "Running clippy..."
        
        local clippy_cmd="cargo clippy --all-targets --all-features -- -D warnings"
        
        if [[ "$VERBOSE" == true ]]; then
            clippy_cmd="$clippy_cmd --verbose"
        fi
        
        if eval "$clippy_cmd"; then
            print_success "Clippy passed"
        else
            print_error "Clippy failed"
            return 1
        fi
    fi
}

# Function to run unit tests
run_unit_tests() {
    if [[ "$UNIT" == true ]]; then
        print_info "Running unit tests..."
        
        local test_cmd="cargo test --lib --bins"
        
        if [[ "$VERBOSE" == true ]]; then
            test_cmd="$test_cmd --verbose"
        fi
        
        if eval "$test_cmd"; then
            print_success "Unit tests passed"
        else
            print_error "Unit tests failed"
            return 1
        fi
    fi
}

# Function to run integration tests
run_integration_tests() {
    if [[ "$INTEGRATION" == true ]]; then
        print_info "Running integration tests..."
        
        local test_cmd="cargo test --test '*'"
        
        if [[ "$VERBOSE" == true ]]; then
            test_cmd="$test_cmd --verbose"
        fi
        
        if eval "$test_cmd"; then
            print_success "Integration tests passed"
        else
            print_error "Integration tests failed"
            return 1
        fi
    fi
}

# Function to run benchmarks
run_benchmarks() {
    if [[ "$BENCH" == true ]]; then
        print_info "Running benchmarks..."
        
        if cargo bench; then
            print_success "Benchmarks completed"
        else
            print_error "Benchmarks failed"
            return 1
        fi
    fi
}

# Function to run coverage
run_coverage() {
    if [[ "$COVERAGE" == true ]]; then
        print_info "Running coverage analysis..."
        
        local coverage_cmd="cargo tarpaulin --out Html --output-dir coverage"
        
        if [[ "$VERBOSE" == true ]]; then
            coverage_cmd="$coverage_cmd --verbose"
        fi
        
        if eval "$coverage_cmd"; then
            print_success "Coverage analysis completed"
            print_info "Coverage report generated in coverage/tarpaulin-report.html"
        else
            print_error "Coverage analysis failed"
            return 1
        fi
    fi
}

# Function to run security audit
run_audit() {
    if [[ "$AUDIT" == true ]]; then
        print_info "Running security audit..."
        
        if cargo audit; then
            print_success "Security audit passed"
        else
            print_error "Security audit failed"
            return 1
        fi
    fi
}

# Function to show test summary
show_summary() {
    print_info "Test Summary:"
    echo "=============="
    
    local passed=0
    local total=0
    
    if [[ "$FORMAT" == true ]]; then
        total=$((total + 1))
        echo "  Format check:      ✓"
        passed=$((passed + 1))
    fi
    
    if [[ "$CLIPPY" == true ]]; then
        total=$((total + 1))
        echo "  Clippy:           ✓"
        passed=$((passed + 1))
    fi
    
    if [[ "$UNIT" == true ]]; then
        total=$((total + 1))
        echo "  Unit tests:       ✓"
        passed=$((passed + 1))
    fi
    
    if [[ "$INTEGRATION" == true ]]; then
        total=$((total + 1))
        echo "  Integration tests: ✓"
        passed=$((passed + 1))
    fi
    
    if [[ "$BENCH" == true ]]; then
        total=$((total + 1))
        echo "  Benchmarks:       ✓"
        passed=$((passed + 1))
    fi
    
    if [[ "$COVERAGE" == true ]]; then
        total=$((total + 1))
        echo "  Coverage:         ✓"
        passed=$((passed + 1))
    fi
    
    if [[ "$AUDIT" == true ]]; then
        total=$((total + 1))
        echo "  Security audit:   ✓"
        passed=$((passed + 1))
    fi
    
    echo ""
    print_success "All tests passed ($passed/$total)"
}

# Main function
main() {
    print_info "fontgrepc Test Suite"
    echo "===================="
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check prerequisites
    check_prerequisites
    
    # Run tests in order
    local failed=false
    
    # Format check
    if ! run_format_check; then
        failed=true
    fi
    
    # Clippy
    if ! run_clippy; then
        failed=true
    fi
    
    # Unit tests
    if ! run_unit_tests; then
        failed=true
    fi
    
    # Integration tests
    if ! run_integration_tests; then
        failed=true
    fi
    
    # Benchmarks
    if ! run_benchmarks; then
        failed=true
    fi
    
    # Coverage
    if ! run_coverage; then
        failed=true
    fi
    
    # Security audit
    if ! run_audit; then
        failed=true
    fi
    
    # Show summary
    if [[ "$failed" == false ]]; then
        echo ""
        show_summary
        print_success "All tests completed successfully!"
    else
        echo ""
        print_error "Some tests failed!"
        exit 1
    fi
}

# Run main function
main "$@"