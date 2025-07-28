#!/bin/bash

# Test script for directory browser functionality
echo "🧪 Testing Rust Tour with directory browser..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "This test will:"
echo "1. Remove any existing test directory"
echo "2. Run rust-tour with a non-existent exercises path"
echo "3. Allow you to test the directory browser"
echo ""
echo "Press Ctrl+C at any time to cancel"
echo ""

# Create a test directory structure
TEST_DIR="/tmp/rust-tour-test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/option1/nested"
mkdir -p "$TEST_DIR/option2/deep/nested"
mkdir -p "$TEST_DIR/option3"

echo "📁 Created test directory structure at: $TEST_DIR"
echo ""

# Run rust-tour with non-existent exercises path to trigger download flow
EXERCISES_PATH="/tmp/nonexistent-exercises" cargo run --package rust-tour --features "download-exercises" -- --port 8888

echo ""
echo "✅ Test completed!"