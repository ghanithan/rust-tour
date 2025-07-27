#!/bin/bash

# Docker test script for Rust Tour
set -e

echo "🧪 Testing Rust Tour Docker Image"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Configuration
IMAGE_NAME="${1:-rust-tour:latest}"
CONTAINER_NAME="rust-tour-test"
TEST_PORT="3001"  # Use different port to avoid conflicts

print_status "Testing Docker image: $IMAGE_NAME"

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed or not available"
    exit 1
fi

# Check if image exists
if ! docker images "$IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "$IMAGE_NAME"; then
    print_error "Docker image $IMAGE_NAME not found"
    echo "Please build the image first: ./scripts/build-docker.sh"
    exit 1
fi

# Cleanup any existing test container
print_status "Cleaning up any existing test containers..."
docker stop "$CONTAINER_NAME" 2>/dev/null || true
docker rm "$CONTAINER_NAME" 2>/dev/null || true

# Create progress directory for testing
mkdir -p ./test-progress

print_status "Starting container for testing..."

# Start the container
docker run -d \
    --name "$CONTAINER_NAME" \
    -p "$TEST_PORT:3000" \
    -v "$(pwd)/test-progress:/app/progress" \
    "$IMAGE_NAME"

if [ $? -ne 0 ]; then
    print_error "Failed to start container"
    exit 1
fi

print_success "Container started successfully"

# Wait for the application to start
print_status "Waiting for application to start..."
TIMEOUT=60
COUNTER=0

while [ $COUNTER -lt $TIMEOUT ]; do
    if docker exec "$CONTAINER_NAME" wget -q --spider http://localhost:3000/health 2>/dev/null; then
        print_success "Application is responding to health checks"
        break
    fi
    
    if [ $COUNTER -eq 30 ]; then
        print_warning "Application taking longer than expected to start..."
    fi
    
    sleep 1
    COUNTER=$((COUNTER + 1))
done

if [ $COUNTER -eq $TIMEOUT ]; then
    print_error "Application failed to start within $TIMEOUT seconds"
    
    print_status "Container logs:"
    docker logs "$CONTAINER_NAME"
    
    # Cleanup
    docker stop "$CONTAINER_NAME"
    docker rm "$CONTAINER_NAME"
    exit 1
fi

# Test basic endpoints
print_status "Testing endpoints..."

# Test health endpoint
if curl -f -s "http://localhost:$TEST_PORT/health" > /dev/null; then
    print_success "✓ Health endpoint responding"
else
    print_warning "✗ Health endpoint not responding"
fi

# Test main page
if curl -f -s "http://localhost:$TEST_PORT/" > /dev/null; then
    print_success "✓ Main page responding"
else
    print_warning "✗ Main page not responding"
fi

# Test if exercises are available
if curl -f -s "http://localhost:$TEST_PORT/api/exercises" > /dev/null; then
    print_success "✓ Exercises API responding"
else
    print_warning "✗ Exercises API not responding"
fi

# Check container resource usage
print_status "Container resource usage:"
docker stats "$CONTAINER_NAME" --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}"

# Check container logs for errors
print_status "Checking container logs for errors..."
if docker logs "$CONTAINER_NAME" 2>&1 | grep -i error | head -5; then
    print_warning "Found some error messages in logs (this might be normal)"
else
    print_success "No error messages found in logs"
fi

# Show sample logs
print_status "Sample container logs (last 10 lines):"
docker logs "$CONTAINER_NAME" --tail 10

print_success "🎉 Docker image test completed!"

echo ""
echo "Container is running at: http://localhost:$TEST_PORT"
echo ""
echo "To stop the test container:"
echo "  docker stop $CONTAINER_NAME && docker rm $CONTAINER_NAME"
echo ""
echo "To keep the container running for manual testing, press Ctrl+C now."
echo "Otherwise, it will be automatically cleaned up in 10 seconds..."

# Wait and cleanup
sleep 10

print_status "Cleaning up test container..."
docker stop "$CONTAINER_NAME"
docker rm "$CONTAINER_NAME"

# Cleanup test directory
rm -rf ./test-progress

print_success "Test cleanup completed!"

print_status "To run the image in production mode:"
echo "  docker run -d -p 3000:3000 -v \$(pwd)/progress:/app/progress --name rust-tour $IMAGE_NAME"