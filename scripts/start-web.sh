#!/bin/bash
set -e

echo "🌐 Starting Rust Learning Platform Web Interface"
echo "==============================================="

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

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed. Please install Node.js 18+ to continue."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    print_error "npm is not installed. Please install npm to continue."
    exit 1
fi

# Check Node.js version
NODE_VERSION=$(node --version | sed 's/v//')
REQUIRED_VERSION="18.0.0"

if ! command -v sort &> /dev/null; then
    print_warning "Cannot verify Node.js version. Proceeding anyway..."
else
    if [[ "$(printf '%s\n' "$REQUIRED_VERSION" "$NODE_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]]; then
        print_error "Node.js version $NODE_VERSION is too old. Please install Node.js 18 or later."
        exit 1
    fi
fi

print_success "Node.js version $NODE_VERSION detected"

# Navigate to web directory
cd "$(dirname "$0")/../web"

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    print_status "Installing Node.js dependencies..."
    npm install
    if [ $? -ne 0 ]; then
        print_error "Failed to install dependencies"
        exit 1
    fi
    print_success "Dependencies installed successfully"
else
    print_status "Dependencies already installed"
fi

# Check if exercises exist
if [ ! -d "../exercises" ]; then
    print_warning "No exercises found. Creating sample exercise structure..."
    mkdir -p ../exercises/ch01_getting_started
    print_warning "Please run the setup script to initialize exercises"
fi

# Function to cleanup background processes
cleanup() {
    print_status "Shutting down servers..."
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
    if [ ! -z "$CLIENT_PID" ]; then
        kill $CLIENT_PID 2>/dev/null || true
    fi
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

print_status "Starting development servers..."
print_status ""
print_status "🔧 API Server will run on http://localhost:3000"
print_status "🌐 Web UI will run on http://localhost:8000"
print_status "📡 WebSocket server will run on ws://localhost:8080"
print_status ""
print_status "Press Ctrl+C to stop all servers"
print_status ""

# Start the API server in background
print_status "Starting API server..."
node server.js &
SERVER_PID=$!

# Wait a moment for server to start
sleep 2

# Check if server is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    print_error "Failed to start API server"
    exit 1
fi

print_success "API server started (PID: $SERVER_PID)"

# Start the frontend development server in background
print_status "Starting frontend development server..."
npm run client &
CLIENT_PID=$!

# Wait a moment for client to start
sleep 3

# Check if client is running
if ! kill -0 $CLIENT_PID 2>/dev/null; then
    print_error "Failed to start frontend server"
    cleanup
    exit 1
fi

print_success "Frontend server started (PID: $CLIENT_PID)"
print_success ""
print_success "🎉 Rust Learning Platform is running!"
print_success ""
print_success "📖 Open your browser and navigate to:"
print_success "   http://localhost:8000"
print_success ""
print_success "📡 API endpoints available at:"
print_success "   http://localhost:3000/api"
print_success ""

# Keep script running and handle signals
while true; do
    # Check if processes are still running
    if ! kill -0 $SERVER_PID 2>/dev/null; then
        print_error "API server stopped unexpectedly"
        cleanup
    fi
    
    if ! kill -0 $CLIENT_PID 2>/dev/null; then
        print_error "Frontend server stopped unexpectedly"
        cleanup
    fi
    
    sleep 5
done