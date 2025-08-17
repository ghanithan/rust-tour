#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if binary already exists
if [ -f "./bin/rust-tour" ]; then
    CURRENT_VERSION=$(./bin/rust-tour --version 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown")
    echo -e "${YELLOW}ℹ️  Found existing installation (version: $CURRENT_VERSION)${NC}"
    echo "Checking for updates..."
fi

echo -e "${BLUE}🚀 Fetching latest Rust Tour release...${NC}"

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
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

echo -e "${GREEN}📦 Detected platform: $TARGET${NC}"

# Create directory for binary
mkdir -p ./bin

# Download latest release
echo -e "${BLUE}⬇️  Fetching latest release info...${NC}"

# Use gh CLI if available to avoid rate limits
if command -v gh &> /dev/null && gh auth status &> /dev/null; then
    echo -e "${GREEN}Using GitHub CLI for authenticated API access${NC}"
    LATEST_RELEASE=$(gh api repos/ghanithan/rust-tour/releases/latest)
else
    LATEST_RELEASE=$(curl -s https://api.github.com/repos/ghanithan/rust-tour/releases/latest)
    # Check for rate limit error
    if echo "$LATEST_RELEASE" | grep -q "API rate limit exceeded"; then
        echo -e "${RED}❌ GitHub API rate limit exceeded${NC}"
        echo -e "${YELLOW}Try one of these alternatives:${NC}"
        echo "  1. Wait an hour and try again"
        echo "  2. Use Docker: docker run -p 3000:3000 ghcr.io/ghanithan/rust-tour:latest"
        echo "  3. Build from source: ./scripts/run.sh dev"
        echo "  4. Install GitHub CLI and authenticate: gh auth login"
        exit 1
    fi
fi

DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/ghanithan/rust-tour/releases/download/[^\"]*rust-tour-${TARGET}\.tar\.gz" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    # Try zip format for Windows
    DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/ghanithan/rust-tour/releases/download/[^\"]*rust-tour-${TARGET}\.zip" | head -1)
fi

if [ -z "$DOWNLOAD_URL" ]; then
    echo "❌ Could not find release artifact for $TARGET"
    echo "Available releases:"
    echo "$LATEST_RELEASE" | grep -o "rust-tour-[^\"]*\.\(tar\.gz\|zip\)" | sort -u
    exit 1
fi

VERSION=$(echo "$LATEST_RELEASE" | grep -o '"tag_name": "[^"]*"' | sed 's/"tag_name": "//;s/"//')
VERSION_NUM=$(echo "$VERSION" | sed 's/^v//')

# Check if update is needed
if [ -f "./bin/rust-tour" ] && [ "$CURRENT_VERSION" = "$VERSION_NUM" ]; then
    echo -e "${GREEN}✅ You already have the latest version ($VERSION)${NC}"
    
    # Check GLIBC compatibility
    if ldd ./bin/"$BINARY_NAME" 2>&1 | grep -q "GLIBC.*not found"; then
        echo ""
        echo -e "${YELLOW}⚠️  Binary is incompatible with your system's GLIBC version${NC}"
        echo -e "${BLUE}Building from source instead...${NC}"
        echo ""
        exec ./scripts/run.sh dev
    fi
    
    echo ""
    echo -e "${BLUE}🚀 Starting Rust Tour on port 3000...${NC}"
    echo -e "${BLUE}📂 Working directory: $(pwd)${NC}"
    echo ""
    exec ./bin/"$BINARY_NAME" --port 3000
fi

echo -e "${GREEN}📌 Latest version: $VERSION${NC}"
if [ -f "./bin/rust-tour" ]; then
    echo -e "${YELLOW}📦 Updating from $CURRENT_VERSION to $VERSION_NUM${NC}"
fi
echo -e "${BLUE}📥 Downloading from: $DOWNLOAD_URL${NC}"

# Download and extract
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

# Check GLIBC compatibility before running
if ldd ./bin/"$BINARY_NAME" 2>&1 | grep -q "GLIBC.*not found"; then
    echo ""
    echo -e "${YELLOW}⚠️  Downloaded binary is incompatible with your system's GLIBC version${NC}"
    echo -e "${YELLOW}Your system has: $(ldd --version | head -n1 | grep -o '[0-9]\+\.[0-9]\+')${NC}"
    echo -e "${YELLOW}Binary requires: GLIBC 2.35 or newer${NC}"
    echo ""
    echo -e "${BLUE}Falling back to building from source (this may take a few minutes)...${NC}"
    echo ""
    exec ./scripts/run.sh dev
fi

echo ""
echo -e "${BLUE}🚀 Starting Rust Tour on port 3000...${NC}"
echo -e "${BLUE}📂 Working directory: $(pwd)${NC}"
echo -e "${YELLOW}💡 Tip: The browser should open automatically. If not, visit http://localhost:3000${NC}"
echo ""

# Run the binary from the project directory
exec ./bin/"$BINARY_NAME" --port 3000