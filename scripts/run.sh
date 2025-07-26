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
    "production"|"prod")
        echo "🚀 Starting production server..."
        exec "$SCRIPT_DIR/start-production.sh" "${@:2}"
        ;;
    "simple"|"start")
        echo "🚀 Starting server (simple mode)..."
        cd "$(dirname "$SCRIPT_DIR")/web"
        exec node server.js
        ;;
    *)
        echo "Usage: $0 {platform|web|test|setup|production|simple} [args...]"
        echo ""
        echo "Available scripts:"
        echo "  simple      - Just start the server (for users)"
        echo "  platform    - Start the complete platform (development)"
        echo "  web         - Start web development server (same as platform)"
        echo "  test        - Run exercise tests"
        echo "  setup       - Set up project"
        echo "  production  - Start optimized production server (build + run)"
        exit 1
        ;;
esac