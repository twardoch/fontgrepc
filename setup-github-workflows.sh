#!/bin/bash
# this_file: setup-github-workflows.sh

# Setup script for GitHub workflows
# This script moves the workflow files to the correct location

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

print_info "Setting up GitHub workflows..."

# Check if we're in the right directory
if [[ ! -d "github-workflows" ]]; then
    print_error "github-workflows directory not found. Please run this script from the project root."
    exit 1
fi

# Create .github/workflows directory
print_info "Creating .github/workflows directory..."
mkdir -p .github/workflows

# Move workflow files
print_info "Moving workflow files..."
for file in github-workflows/*.yml; do
    if [[ -f "$file" ]]; then
        filename=$(basename "$file")
        print_info "Moving $filename..."
        mv "$file" .github/workflows/
    fi
done

# Check if files were moved successfully
workflow_count=$(find .github/workflows -name "*.yml" | wc -l)
print_success "Moved $workflow_count workflow files to .github/workflows/"

# List the moved files
print_info "Available workflows:"
for file in .github/workflows/*.yml; do
    if [[ -f "$file" ]]; then
        filename=$(basename "$file")
        echo "  - $filename"
    fi
done

# Remove the temporary directory
print_info "Cleaning up temporary directory..."
rm -rf github-workflows

print_success "GitHub workflows setup complete!"
echo ""
print_info "Next steps:"
echo "1. Commit and push the workflow files:"
echo "   git add .github/workflows/"
echo "   git commit -m 'Add GitHub Actions workflows'"
echo "   git push origin main"
echo ""
echo "2. Configure required secrets in your GitHub repository:"
echo "   - CRATES_IO_TOKEN (for publishing to crates.io)"
echo "   - CODECOV_TOKEN (optional, for code coverage)"
echo ""
echo "3. Create your first release:"
echo "   ./scripts/release.sh v1.0.6"