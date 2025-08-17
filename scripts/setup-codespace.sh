#!/bin/bash

# Colors for better readability
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color
BOLD='\033[1m'

echo -e "${BLUE}🚀 Setting up Rust Tour...${NC}"

# Always download/overwrite the binary for fresh setup
echo -e "${YELLOW}📦 Downloading latest Rust Tour binary...${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map to rust target triple
case "$OS" in
    linux)
        TARGET="x86_64-unknown-linux-gnu"
        BINARY_NAME="rust-tour"
        ;;
    darwin)
        if [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
        BINARY_NAME="rust-tour"
        ;;
    mingw*|msys*|cygwin*)
        TARGET="x86_64-pc-windows-msvc"
        BINARY_NAME="rust-tour.exe"
        ;;
    *)
        echo -e "${RED}❌ Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}📦 Detected platform: $TARGET${NC}"

# Create directory for binary
mkdir -p ./bin

# Download latest release
echo -e "${BLUE}⬇️  Downloading latest release...${NC}"

# Use gh CLI if available to avoid rate limits (should be available in Codespaces)
if command -v gh &> /dev/null && gh auth status &> /dev/null; then
    LATEST_RELEASE=$(gh api repos/ghanithan/rust-tour/releases/latest)
else
    LATEST_RELEASE=$(curl -s https://api.github.com/repos/ghanithan/rust-tour/releases/latest)
    # Check for rate limit error
    if echo "$LATEST_RELEASE" | grep -q "API rate limit exceeded"; then
        echo -e "${YELLOW}⚠️  GitHub API rate limit exceeded. Will build from source if needed.${NC}"
        # Don't exit, let the setup continue
    fi
fi

DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/ghanithan/rust-tour/releases/download/[^\"]*rust-tour-${TARGET}\.tar\.gz" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    # Try zip format for Windows
    DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/ghanithan/rust-tour/releases/download/[^\"]*rust-tour-${TARGET}\.zip" | head -1)
fi

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}❌ Could not find release artifact for $TARGET${NC}"
    exit 1
fi

VERSION=$(echo "$LATEST_RELEASE" | grep -o '"tag_name": "[^"]*"' | sed 's/"tag_name": "//;s/"//')
echo -e "${GREEN}📌 Downloading version: $VERSION${NC}"

# Download and extract (always overwrite)
cd ./bin
curl -L -o rust-tour-release.archive "$DOWNLOAD_URL"

if [[ "$DOWNLOAD_URL" == *.zip ]]; then
    unzip -o rust-tour-release.archive
else
    tar -xzf rust-tour-release.archive
fi

rm rust-tour-release.archive
chmod +x "$BINARY_NAME"
cd ..

echo -e "${GREEN}✅ Download complete!${NC}"

# Check GLIBC compatibility
if ldd ./bin/"$BINARY_NAME" 2>&1 | grep -q "GLIBC.*not found"; then
    echo ""
    echo -e "${YELLOW}⚠️  Downloaded binary is incompatible with your system's GLIBC version${NC}"
    echo -e "${BLUE}Will fall back to development mode when you start Rust Tour${NC}"
    echo ""
fi

echo ""
echo -e "${GREEN}✅ Rust Tour setup complete!${NC}"
echo -e "${BOLD}Run: ${GREEN}./scripts/welcome.sh${NC} ${BOLD}to start the interactive menu${NC}"
echo ""

# Create setup completion status file for VS Code extension
touch .setup-complete
echo "Setup completed at $(date)" > .setup-complete

# Update terminal title to show completion
echo -e "\033]0;✅ Rust Tour Setup Complete!\007"

# Try to send notification to user's terminal (if available)
if [ -n "$TERM_PROGRAM" ] && [ "$TERM_PROGRAM" = "vscode" ]; then
    # Create a more prominent completion message for VS Code terminal
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC} ${GREEN}🎉 RUST TOUR SETUP COMPLETE! 🎉${NC}                ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}                                                ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC} ${BOLD}Ready to start learning Rust!${NC}                  ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}                                                ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC} ${YELLOW}Run this command to begin:${NC}                    ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC} ${GREEN}${BOLD}./scripts/welcome.sh${NC}                          ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════╝${NC}"
    echo ""
    
    # Try to get user's attention with bell
    echo -e "\a"
fi