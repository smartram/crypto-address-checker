#!/bin/bash

# Crypto Address Checker Browser Extension Build Script
# This script builds the Rust + WASM extension and prepares it for browser installation

set -e  # Exit on any error

echo "🚀 Building Crypto Address Checker Extension..."
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
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

# Check if required tools are installed
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check Rust/Cargo
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found! Please install Rust first:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_success "Cargo found: $(cargo --version)"
    
    # Check wasm-pack
    if ! command -v wasm-pack &> /dev/null; then
        print_error "wasm-pack not found! Please install it first:"
        echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
        exit 1
    fi
    print_success "wasm-pack found: $(wasm-pack --version)"
}

# Clean previous builds
clean_build() {
    print_status "Cleaning previous builds..."
    
    if [ -d "pkg" ]; then
        rm -rf pkg
        print_success "Removed old pkg directory"
    fi
    
    if [ -d "extension" ]; then
        rm -rf extension
        print_success "Removed old extension directory"
    fi
}

# Build WASM module
build_wasm() {
    print_status "Building WebAssembly module..."
    
    # Run cargo check first
    print_status "Running cargo check..."
    if ! cargo check; then
        print_error "Cargo check failed! Please fix compilation errors."
        exit 1
    fi
    print_success "Cargo check passed"
    
    # Build WASM
    print_status "Compiling Rust to WebAssembly..."
    if ! wasm-pack build --target web --out-dir pkg; then
        print_error "wasm-pack build failed!"
        exit 1
    fi
    print_success "WASM module built successfully"
}

# Create extension directory and copy files
prepare_extension() {
    print_status "Preparing extension directory..."
    
    # Create extension directory
    mkdir -p extension
    print_success "Created extension directory"
    
    # Copy extension files
    print_status "Copying extension files..."
    cp manifest.json extension/ || { print_error "Failed to copy manifest.json"; exit 1; }
    cp popup.html extension/ || { print_error "Failed to copy popup.html"; exit 1; }
    cp popup.js extension/ || { print_error "Failed to copy popup.js"; exit 1; }
    cp content.js extension/ || { print_error "Failed to copy content.js"; exit 1; }
    cp background.js extension/ || { print_error "Failed to copy background.js"; exit 1; }
    print_success "Extension files copied"
    
    # Copy WASM package
    print_status "Copying WASM package..."
    if [ ! -d "pkg" ]; then
        print_error "pkg directory not found! WASM build may have failed."
        exit 1
    fi
    cp -r pkg extension/ || { print_error "Failed to copy pkg directory"; exit 1; }
    print_success "WASM package copied"
}

# Verify extension structure
verify_extension() {
    print_status "Verifying extension structure..."
    
    required_files=(
        "extension/manifest.json"
        "extension/popup.html"
        "extension/popup.js"
        "extension/content.js"
        "extension/background.js"
        "extension/pkg/crypto_address_checker.js"
        "extension/pkg/crypto_address_checker_bg.wasm"
    )
    
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            print_error "Missing required file: $file"
            exit 1
        fi
    done
    
    print_success "All required files present"
}

# Print installation instructions
print_instructions() {
    echo
    echo "==============================================="
    print_success "🎉 Build completed successfully!"
    echo "==============================================="
    echo
    print_status "📁 Extension files are ready in the 'extension' directory"
    echo
    print_status "🌐 To install in Brave/Chrome:"
    echo "   1. Open brave://extensions/ (or chrome://extensions/)"
    echo "   2. Enable 'Developer mode'"
    echo "   3. Click 'Load unpacked'"
    echo "   4. Select the 'extension' folder from this project"
    echo
    print_status "🔧 To rebuild after making changes:"
    echo "   ./build.sh"
    echo
    print_status "📊 Extension size:"
    if command -v du &> /dev/null; then
        du -sh extension/
    fi
    echo
}

# Main build process
main() {
    echo "🦀 Crypto Address Checker - Build Script"
    echo "========================================="
    echo
    
    check_prerequisites
    clean_build
    build_wasm
    prepare_extension
    verify_extension
    print_instructions
}

# Handle script interruption
trap 'print_error "Build interrupted!"; exit 1' INT

# Run main function
main "$@"
