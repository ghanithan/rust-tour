#!/bin/bash

# Script runner for Rust Tour
# Usage: ./scripts/run.sh [script-name] [args...]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

case "$1" in
    "platform")
        echo "🚀 Starting Rust Tour..."
        exec "$SCRIPT_DIR/start-platform.sh" "${@:2}"
        ;;
    "web")
        echo "🌐 Starting web development server..."
        exec "$SCRIPT_DIR/start-web.sh" "${@:2}"
        ;;
    "server")
        echo "🦀 Starting Rust server only..."
        cd "$PROJECT_ROOT"
        exec cargo run --package rust-tour --no-default-features
        ;;
    "dev")
        echo "🚀 Starting development mode (Vite + Rust server)..."
        cd "$PROJECT_ROOT"
        # Start Rust server in background
        cargo run --package rust-tour --no-default-features &
        SERVER_PID=$!
        # Start Vite dev server
        cd web && npm run client &
        VITE_PID=$!
        # Wait for interrupt and cleanup
        trap "kill $SERVER_PID $VITE_PID 2>/dev/null" EXIT
        wait
        ;;
    "test")
        echo "🧪 Running exercise tests..."
        exec "$SCRIPT_DIR/test-exercises.sh" "${@:2}"
        ;;
    "setup")
        echo "⚙️ Setting up project..."
        exec "$SCRIPT_DIR/setup.sh" "${@:2}"
        ;;
    "production"|"prod")
        echo "🚀 Starting production server..."
        exec "$SCRIPT_DIR/start-production.sh" "${@:2}"
        ;;
    "simple"|"start")
        echo "🦀 Starting Rust server (simple mode)..."
        cd "$PROJECT_ROOT"
        # Build frontend if needed
        if [ ! -d "web/dist" ] || [ "web/src" -nt "web/dist" ]; then
            echo "📦 Building frontend..."
            cd web && npm run build && cd ..
        fi
        exec cargo run --package rust-tour --no-default-features
        ;;
    "publish")
        echo "📦 Building for publishing..."
        cd "$PROJECT_ROOT"
        echo "🌐 Building frontend..."
        cd web && npm run build && cd ..
        echo "🦀 Building Rust server with embedded assets..."
        cargo build --release --package rust-tour --features "embed-assets,download-exercises"
        echo "✅ Build complete! Binary at: target/release/rust-tour"
        ;;
    *)
        echo "Usage: $0 {platform|web|server|dev|test|setup|production|simple|publish} [args...]"
        echo ""
        echo "Available scripts:"
        echo "  simple      - Start Rust server (builds frontend if needed)"
        echo "  start       - Same as simple"
        echo "  server      - Start Rust server only (development mode)"
        echo "  dev         - Start Vite dev server + Rust API server"
        echo "  platform    - Start complete platform (legacy Node.js)"
        echo "  web         - Start web development server (legacy)"
        echo "  test        - Run exercise tests"
        echo "  setup       - Set up project"
        echo "  production  - Start optimized production server"
        echo "  publish     - Build for publishing with embedded assets"
        exit 1
        ;;
esac