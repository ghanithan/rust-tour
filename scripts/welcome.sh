#!/bin/bash

# Colors for better readability
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color
BOLD='\033[1m'

clear

# ASCII Art Banner
echo -e "${BLUE}"
cat << 'EOF'
    ╦═╗╦ ╦╔═╗╔╦╗  ╔╦╗╔═╗╦ ╦╦═╗
    ╠╦╝║ ║╚═╗ ║    ║ ║ ║║ ║╠╦╝
    ╩╚═╚═╝╚═╝ ╩    ╩ ╚═╝╚═╝╩╚═
    
    🦀 Interactive Rust Learning Platform 🦀
EOF
echo -e "${NC}"

echo -e "${GREEN}Welcome to the Rust Tour Dev Container!${NC}"
echo -e "This environment is set up for learning Rust through interactive exercises."
echo ""

# Check if binary already exists
if [ -f "./bin/rust-tour" ]; then
    echo -e "${YELLOW}ℹ️  Rust Tour is already installed${NC}"
    
    # Check for updates
    CURRENT_VERSION=$(./bin/rust-tour --version 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown")
    LATEST_VERSION=$(curl -s https://api.github.com/repos/ghanithan/rust-tour/releases/latest | grep -o '"tag_name": "[^"]*"' | sed 's/"tag_name": "v*//;s/"//')
    
    if [ "$CURRENT_VERSION" != "$LATEST_VERSION" ] && [ "$LATEST_VERSION" != "" ]; then
        echo -e "${YELLOW}📦 Update available: $CURRENT_VERSION → $LATEST_VERSION${NC}"
    else
        echo -e "${GREEN}✅ You have the latest version: $CURRENT_VERSION${NC}"
    fi
    echo ""
fi

echo -e "${BOLD}What would you like to do?${NC}"
echo ""
echo "  1) 🚀 Start Rust Tour (recommended for learners)"
echo "  2) 🔄 Download/Update to latest release"
echo "  3) 🔧 Run development mode (for contributors)"
echo "  4) 📚 View available commands"
echo "  5) 🚪 Exit (explore on your own)"
echo ""

read -t 10 -p "Enter your choice (1-5): " choice
if [ $? -ne 0 ]; then
    echo -e "\nNo input received in 10 seconds. Defaulting to option 1."
    choice=1
fi

case $choice in
    1)
        echo ""
        if [ -f "./bin/rust-tour" ]; then
            # Check GLIBC compatibility
            if ldd ./bin/rust-tour 2>&1 | grep -q "GLIBC.*not found"; then
                echo -e "${YELLOW}⚠️  Binary incompatible with system GLIBC version${NC}"
                echo -e "${BLUE}Building from source instead (this may take a few minutes)...${NC}"
                echo ""
                exec ./scripts/run.sh dev
            else
                echo -e "${GREEN}Starting Rust Tour...${NC}"
                echo -e "${BLUE}🚀 Starting on port 3000...${NC}"
                echo -e "${YELLOW}💡 Tip: The browser should open automatically. If not, visit http://localhost:3000${NC}"
                echo ""
                exec ./bin/rust-tour --port 3000
            fi
        else
            echo -e "${YELLOW}Rust Tour not installed. Installing now...${NC}"
            
            # Download binary using run-release.sh logic
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
            echo -e "${BLUE}⬇️  Downloading latest release...${NC}"
            LATEST_RELEASE=$(curl -s https://api.github.com/repos/ghanithan/rust-tour/releases/latest)
            DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/ghanithan/rust-tour/releases/download/[^\"]*rust-tour-${TARGET}\.tar\.gz" | head -1)
            
            if [ -z "$DOWNLOAD_URL" ]; then
                # Try zip format for Windows
                DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/ghanithan/rust-tour/releases/download/[^\"]*rust-tour-${TARGET}\.zip" | head -1)
            fi
            
            if [ -z "$DOWNLOAD_URL" ]; then
                echo "❌ Could not find release artifact for $TARGET"
                exit 1
            fi
            
            VERSION=$(echo "$LATEST_RELEASE" | grep -o '"tag_name": "[^"]*"' | sed 's/"tag_name": "//;s/"//')
            echo -e "${GREEN}📌 Downloading version: $VERSION${NC}"
            
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
            
            # Check GLIBC compatibility
            if ldd ./bin/"$BINARY_NAME" 2>&1 | grep -q "GLIBC.*not found"; then
                echo ""
                echo -e "${YELLOW}⚠️  Downloaded binary is incompatible with your system's GLIBC version${NC}"
                echo -e "${BLUE}Building from source instead (this may take a few minutes)...${NC}"
                echo ""
                exec ./scripts/run.sh dev
            fi
            
            echo ""
            echo -e "${GREEN}✅ Installation complete! Starting Rust Tour...${NC}"
            echo -e "${BLUE}🚀 Starting on port 3000...${NC}"
            echo -e "${YELLOW}💡 Tip: The browser should open automatically. If not, visit http://localhost:3000${NC}"
            echo ""
            exec ./bin/rust-tour --port 3000
        fi
        ;;
    
    2)
        echo ""
        echo -e "${GREEN}Downloading/Updating Rust Tour...${NC}"
        exec ./scripts/run-release.sh
        ;;
    
    3)
        echo ""
        echo -e "${GREEN}Starting development mode...${NC}"
        echo -e "${YELLOW}This will compile from source and may take a few minutes...${NC}"
        echo ""
        exec ./scripts/run.sh dev
        ;;
    
    4)
        echo ""
        echo -e "${BOLD}Available Commands:${NC}"
        echo ""
        echo -e "${GREEN}For Learners:${NC}"
        echo "  ./scripts/welcome.sh         - Show this menu again"
        echo "  ./scripts/run-release.sh     - Download and run latest release"
        echo "  ./bin/rust-tour             - Run installed Rust Tour"
        echo ""
        echo -e "${GREEN}For Contributors:${NC}"
        echo "  ./scripts/run.sh dev        - Run in development mode"
        echo "  ./scripts/run.sh test       - Run all tests"
        echo "  ./scripts/setup.sh          - Initial project setup"
        echo "  cargo test                  - Run Rust tests"
        echo "  cd web && npm run dev       - Run frontend only"
        echo ""
        echo -e "${GREEN}Exercise Management:${NC}"
        echo "  cd exercises/               - Browse exercise files"
        echo "  ./scripts/check-exercise.sh - Validate exercise solutions"
        echo ""
        echo -e "${BLUE}Press Enter to return to menu...${NC}"
        read
        exec ./scripts/welcome.sh
        ;;
    
    5)
        echo ""
        echo -e "${GREEN}Happy exploring! 🦀${NC}"
        echo ""
        echo "Run ${BOLD}./scripts/welcome.sh${NC} anytime to see this menu again."
        echo ""
        ;;
    
    *)
        echo ""
        echo -e "${RED}Invalid choice. Please try again.${NC}"
        sleep 2
        exec ./scripts/welcome.sh
        ;;
esac