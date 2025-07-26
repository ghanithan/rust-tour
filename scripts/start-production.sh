#!/bin/bash
set -e

echo "🚀 Starting Rust Learning Platform (Production Mode)"
echo "=================================================="

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

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check dependencies
print_status "Checking dependencies..."

if ! command_exists cargo; then
    print_error "Cargo not found. Please install Rust first."
    exit 1
fi

if ! command_exists node; then
    print_error "Node.js not found. Please install Node.js first."
    exit 1
fi

if ! command_exists npm; then
    print_error "npm not found. Please install npm first."
    exit 1
fi

print_success "All dependencies found"

# Navigate to web directory
cd "$PROJECT_ROOT/web"

# Install npm dependencies if needed
print_status "Installing dependencies..."
if [ ! -d "node_modules" ]; then
    print_status "Installing npm packages..."
    npm install
    if [ $? -ne 0 ]; then
        print_error "Failed to install dependencies"
        exit 1
    fi
    print_success "Dependencies installed"
else
    print_status "Dependencies already installed"
fi

# Build the Rust exercise framework
print_status "Building exercise framework..."
cd "$PROJECT_ROOT"
if [ -d "exercise-framework" ]; then
    cargo build --manifest-path exercise-framework/Cargo.toml
    if [ $? -ne 0 ]; then
        print_warning "Exercise framework build failed, continuing anyway..."
    else
        print_success "Exercise framework built"
    fi
else
    print_warning "Exercise framework not found, skipping..."
fi

# Check if frontend is built
print_status "Checking frontend build..."
cd "$PROJECT_ROOT/web"
if [ ! -d "dist" ] || [ ! -f "dist/index.html" ]; then
    print_warning "Frontend not built. Building now..."
    npm run build
    if [ $? -ne 0 ]; then
        print_error "Frontend build failed"
        exit 1
    fi
    print_success "Frontend built successfully"
else
    print_success "Frontend already built"
fi

# Function to cleanup on exit
cleanup() {
    print_status "Shutting down server..."
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
    exit 0
}

trap cleanup EXIT INT TERM

# Start the production server
print_status "Starting production server..."
print_status ""
print_status "🚀 Production mode features:"
print_status "  • Optimized and minified assets"
print_status "  • Single server process"
print_status "  • Better performance"
print_status "  • No hot reloading (restart required for changes)"
print_status ""
print_status "🌐 Server will run on http://localhost:3000"
print_status "📡 WebSocket server included"
print_status ""
print_status "Press Ctrl+C to stop the server"
print_status ""

# Set production environment
export NODE_ENV=production

# Start the server
node server.js &
SERVER_PID=$!

# Wait for server to start
sleep 2

# Check if server is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    print_error "Failed to start production server"
    exit 1
fi

print_success "Production server started successfully!"
print_success ""
print_success "🎉 Rust Learning Platform is running in production mode!"
print_success ""
print_success "📖 Open your browser and navigate to:"
print_success "   http://localhost:3000"
print_success ""
print_success "📡 API endpoints available at:"
print_success "   http://localhost:3000/api"
print_success ""

# Keep script running and monitor server
while true; do
    # Check if server process is still running
    if ! kill -0 $SERVER_PID 2>/dev/null; then
        print_error "Server stopped unexpectedly"
        exit 1
    fi
    
    sleep 5
done