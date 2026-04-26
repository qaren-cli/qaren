#!/usr/bin/env bash

# Qaren Secure Installer
# https://qaren.me
# -------------------------------------------------------------------------
# Usage: curl -sSfL https://qaren.me/install | sh
# -------------------------------------------------------------------------

set -euo pipefail

# --- Configuration ---
REPO="qaren-cli/qaren"
BINARY_NAME="qaren"
GITHUB_URL="https://github.com/$REPO"

# --- Colors for Output ---
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() { echo -e "${BLUE}[INFO]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }

# --- Identity & Platform Detection ---
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64)  ARCH="amd64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) error "Unsupported architecture: $ARCH" ;;
esac

# --- Map to Release Asset Names ---
case "$OS" in
    linux)
        # Check for libc vs musl (detect Fedora/Alpine)
        if ldd /bin/ls | grep -q "musl"; then
            PLATFORM="fedora-$ARCH" # Using our musl build for maximum compatibility
        else
            PLATFORM="linux-$ARCH"
        fi
        EXTENSION="tar.gz"
        ;;
    darwin)
        if [ "$ARCH" == "amd64" ]; then
            PLATFORM="macos-intel"
        else
            PLATFORM="macos-silicon"
        fi
        EXTENSION="tar.gz"
        ;;
    *) error "Unsupported OS: $OS" ;;
esac

ASSET_NAME="qaren-${PLATFORM}.${EXTENSION}"

# --- Version Resolution ---
log "Fetching latest release version..."
LATEST_TAG=$(curl -sSf "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    error "Could not resolve latest version from GitHub API."
fi

# --- Idempotency Check ---
if command -v qaren >/dev/null 2>&1; then
    CURRENT_VER="v$(qaren --version | awk '{print $2}')"
    if [ "$CURRENT_VER" == "$LATEST_TAG" ]; then
        success "Qaren $CURRENT_VER is already installed and up to date."
        exit 0
    fi
    log "Updating Qaren from $CURRENT_VER to $LATEST_TAG..."
fi

# --- Installation Path ---
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    # Fallback to local user bin if /usr/local/bin is not writable
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    
    # Ensure it's in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log "Adding $INSTALL_DIR to your PATH via .bashrc/.zshrc..."
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
        [ -f "$HOME/.zshrc" ] && echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc"
    fi
fi

# --- Download & Extract ---
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

log "Downloading $ASSET_NAME ($LATEST_TAG)..."
DOWNLOAD_URL="$GITHUB_URL/releases/download/$LATEST_TAG/$ASSET_NAME"

curl -sSfL "$DOWNLOAD_URL" -o "$TEMP_DIR/$ASSET_NAME"

log "Installing to $INSTALL_DIR..."
if [[ "$EXTENSION" == "tar.gz" ]]; then
    tar -xzf "$TEMP_DIR/$ASSET_NAME" -C "$TEMP_DIR"
else
    unzip -q "$TEMP_DIR/$ASSET_NAME" -d "$TEMP_DIR"
fi

# Move binary and set permissions
mv "$TEMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

success "Qaren $LATEST_TAG installed successfully at $INSTALL_DIR/$BINARY_NAME"
log "Run 'qaren --help' to get started."
