#!/bin/bash
set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🦀 Starting Rust Learning Platform"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check dependencies
echo -e "${BLUE}Checking dependencies...${NC}"

if ! command_exists cargo; then
    echo -e "${RED}Error: Cargo not found. Please install Rust first.${NC}"
    exit 1
fi

if ! command_exists node; then
    echo -e "${RED}Error: Node.js not found. Please install Node.js first.${NC}"
    exit 1
fi

if ! command_exists npm; then
    echo -e "${RED}Error: npm not found. Please install npm first.${NC}"
    exit 1
fi

# Install npm dependencies if needed
echo -e "${BLUE}Installing dependencies...${NC}"
cd "$PROJECT_ROOT/web"
if [ ! -d "node_modules" ]; then
    echo "Installing npm packages..."
    npm install
fi

# Build the Rust exercise framework
echo -e "${BLUE}Building exercise framework...${NC}"
cd "$PROJECT_ROOT"
cargo build --manifest-path exercise-framework/Cargo.toml

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}Shutting down servers...${NC}"
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
    if [ ! -z "$CLIENT_PID" ]; then
        kill $CLIENT_PID 2>/dev/null || true
    fi
    exit 0
}

trap cleanup EXIT INT TERM

# Start the Express server
echo -e "${BLUE}Starting API server on port 3000...${NC}"
cd "$PROJECT_ROOT/web"
node server.js &
SERVER_PID=$!

# Wait for server to start
sleep 2

# Start the Vite dev server
echo -e "${BLUE}Starting web UI on port 8000...${NC}"
npm run client &
CLIENT_PID=$!

# Wait a moment for everything to start
sleep 3

echo -e "\n${GREEN}✨ Rust Learning Platform is running!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "🌐 Web UI:    ${BLUE}http://localhost:8000${NC}"
echo -e "📡 API:       ${BLUE}http://localhost:3000${NC}"
echo -e "📚 Exercises: ${BLUE}http://localhost:8000/#exercises${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "\nPress ${YELLOW}Ctrl+C${NC} to stop the platform"

# Keep the script running and wait for termination
wait