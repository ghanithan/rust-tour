#!/bin/bash

# Script runner for the Rust Learning Platform
# Usage: ./scripts/run.sh [script-name] [args...]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

case "$1" in
    "platform")
        echo "🚀 Starting Rust Learning Platform..."
        exec "$SCRIPT_DIR/start-platform.sh" "${@:2}"
        ;;
    "web")
        echo "🌐 Starting web development server..."
        exec "$SCRIPT_DIR/start-web.sh" "${@:2}"
        ;;
    "test")
        echo "🧪 Running exercise tests..."
        exec "$SCRIPT_DIR/test-exercises.sh" "${@:2}"
        ;;
    "setup")
        echo "⚙️ Setting up project..."
        exec "$SCRIPT_DIR/setup.sh" "${@:2}"
        ;;
    *)
        echo "Usage: $0 {platform|web|test|setup} [args...]"
        echo ""
        echo "Available scripts:"
        echo "  platform  - Start the complete platform"
        echo "  web       - Start web development server"
        echo "  test      - Run exercise tests"
        echo "  setup     - Set up project"
        exit 1
        ;;
esac