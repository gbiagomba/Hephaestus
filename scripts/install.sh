#!/usr/bin/env bash
#
# Hephaestus Installation Script
# Supports: Linux (Debian/Ubuntu, RHEL/Fedora/CentOS, Arch), macOS
#
# This script will:
# 1. Detect the OS and architecture
# 2. Install dependencies (Rust, git) using system package managers
# 3. Build and install Hephaestus from source
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print functions
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x64"
            ;;
        aarch64|arm64)
            ARCH="arm64"
            ;;
        *)
            print_error "Unsupported architecture: $ARCH"
            ;;
    esac

    print_info "Detected platform: $OS-$ARCH"
}

# Check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Rust using rustup
install_rust() {
    if command_exists rustc && command_exists cargo; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        print_info "Rust is already installed (version $RUST_VERSION)"
        return 0
    fi

    print_info "Installing Rust via rustup..."
    if command_exists curl; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    elif command_exists wget; then
        wget -qO- https://sh.rustup.rs | sh -s -- -y
    else
        print_error "Neither curl nor wget found. Please install one of them first."
    fi

    # Source cargo environment
    if [ -f "$HOME/.cargo/env" ]; then
        # shellcheck disable=SC1091
        source "$HOME/.cargo/env"
    fi

    print_info "Rust installed successfully"
}

# Install git based on OS
install_git() {
    if command_exists git; then
        GIT_VERSION=$(git --version | cut -d' ' -f3)
        print_info "Git is already installed (version $GIT_VERSION)"
        return 0
    fi

    print_info "Installing git..."

    case "$OS" in
        linux)
            if command_exists apt-get; then
                # Debian/Ubuntu
                sudo apt-get update
                sudo apt-get install -y git
            elif command_exists yum; then
                # RHEL/CentOS/Fedora
                sudo yum install -y git
            elif command_exists dnf; then
                # Fedora (newer)
                sudo dnf install -y git
            elif command_exists pacman; then
                # Arch Linux
                sudo pacman -Sy --noconfirm git
            elif command_exists apk; then
                # Alpine Linux
                sudo apk add --no-cache git
            else
                print_error "Unable to detect package manager. Please install git manually."
            fi
            ;;
        darwin)
            # macOS
            if command_exists brew; then
                brew install git
            else
                print_warn "Homebrew not found. Installing Homebrew first..."
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
                brew install git
            fi
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            ;;
    esac

    print_info "Git installed successfully"
}

# Install build dependencies
install_build_deps() {
    print_info "Installing build dependencies..."

    case "$OS" in
        linux)
            if command_exists apt-get; then
                sudo apt-get install -y build-essential pkg-config libssl-dev
            elif command_exists yum; then
                sudo yum groupinstall -y "Development Tools"
                sudo yum install -y openssl-devel
            elif command_exists dnf; then
                sudo dnf groupinstall -y "Development Tools"
                sudo dnf install -y openssl-devel
            elif command_exists pacman; then
                sudo pacman -Sy --noconfirm base-devel openssl
            elif command_exists apk; then
                sudo apk add --no-cache build-base openssl-dev
            fi
            ;;
        darwin)
            # macOS - Xcode command line tools should be sufficient
            if ! xcode-select -p &>/dev/null; then
                print_info "Installing Xcode command line tools..."
                xcode-select --install
            fi
            ;;
    esac
}

# Build and install Hephaestus
build_and_install() {
    print_info "Building Hephaestus from source..."

    # Ensure we have cargo in PATH
    if ! command_exists cargo; then
        # shellcheck disable=SC1091
        [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
    fi

    if ! command_exists cargo; then
        print_error "Cargo not found. Rust installation may have failed."
    fi

    # Build in release mode
    cargo build --release

    print_info "Installing Hephaestus..."
    cargo install --path . --force

    # Verify installation
    if command_exists hephaestus; then
        VERSION=$(hephaestus --version | cut -d' ' -f2)
        print_info "Hephaestus $VERSION installed successfully!"
        print_info "Run 'hephaestus --help' to get started"
    else
        print_warn "Hephaestus binary installed but not in PATH."
        print_warn "Add ~/.cargo/bin to your PATH:"
        print_warn "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    fi
}

# Main installation flow
main() {
    echo "========================================"
    echo "  Hephaestus Installation Script"
    echo "========================================"
    echo ""

    detect_platform
    install_git
    install_rust
    install_build_deps
    build_and_install

    echo ""
    echo "========================================"
    echo "  Installation Complete!"
    echo "========================================"
    echo ""
    print_info "To use Hephaestus, ensure ~/.cargo/bin is in your PATH"
    print_info "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo '  export PATH="$HOME/.cargo/bin:$PATH"'
    echo ""
    print_info "Then run: source ~/.bashrc  (or restart your terminal)"
}

# Run main installation
main
