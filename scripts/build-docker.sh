#!/bin/bash

# Docker build script for Rust Tour
set -e

echo "🐳 Building Rust Tour Docker Image"
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

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed or not available in PATH"
    echo "Please install Docker first: https://docs.docker.com/get-docker/"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Dockerfile" ]; then
    print_error "Dockerfile not found. Please run this script from the project root directory."
    exit 1
fi

print_status "Checking build context..."

# Verify required files exist
REQUIRED_FILES=("Cargo.toml" "Cargo.lock" "Dockerfile" ".dockerignore")
for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        print_status "✓ $file exists"
    else
        print_error "✗ $file is missing"
        exit 1
    fi
done

# Check required directories
REQUIRED_DIRS=("exercises" "web" "web-server" "scripts")
for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        print_status "✓ $dir/ directory exists"
    else
        print_error "✗ $dir/ directory is missing"
        exit 1
    fi
done

# Set image name and tag
IMAGE_NAME="rust-tour"
TAG="${1:-latest}"
FULL_IMAGE_NAME="$IMAGE_NAME:$TAG"

print_status "Building Docker image: $FULL_IMAGE_NAME"

# Build options
BUILD_ARGS=""
PLATFORM=""

# Check if we should build for multiple platforms
if [ "$2" = "--multi-platform" ]; then
    PLATFORM="--platform linux/amd64,linux/arm64"
    print_status "Building for multiple platforms: linux/amd64,linux/arm64"
fi

# Check if we should use buildx
if [ "$2" = "--buildx" ] || [ "$3" = "--buildx" ]; then
    print_status "Using Docker Buildx for build"
    
    # Check if buildx is available
    if ! docker buildx version &> /dev/null; then
        print_error "Docker Buildx is not available"
        exit 1
    fi
    
    # Create a builder if it doesn't exist
    if ! docker buildx inspect rust-tour-builder &> /dev/null; then
        print_status "Creating buildx builder instance..."
        docker buildx create --name rust-tour-builder --use
    else
        docker buildx use rust-tour-builder
    fi
    
    # Build with buildx
    docker buildx build $PLATFORM --load -t "$FULL_IMAGE_NAME" .
else
    # Regular docker build
    print_status "Using regular Docker build"
    docker build -t "$FULL_IMAGE_NAME" .
fi

# Check if build was successful
if [ $? -eq 0 ]; then
    print_success "Docker image built successfully!"
    
    # Show image information
    print_status "Image details:"
    docker images "$IMAGE_NAME" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"
    
    # Test the image
    print_status "Testing the built image..."
    
    # Run a quick test
    if docker run --rm "$FULL_IMAGE_NAME" --version &> /dev/null; then
        print_success "Image test passed - binary executes correctly"
    else
        print_warning "Image test warning - version check failed (this might be expected)"
    fi
    
    echo ""
    print_success "🎉 Build completed successfully!"
    echo ""
    echo "To run the container:"
    echo "  docker run -d -p 3000:3000 -v \$(pwd)/progress:/app/progress $FULL_IMAGE_NAME"
    echo ""
    echo "To run with interactive mode:"
    echo "  docker run -it --rm -p 3000:3000 $FULL_IMAGE_NAME"
    echo ""
    echo "To push to registry:"
    echo "  docker tag $FULL_IMAGE_NAME your-registry/$FULL_IMAGE_NAME"
    echo "  docker push your-registry/$FULL_IMAGE_NAME"
    
else
    print_error "Docker build failed!"
    echo ""
    echo "Common solutions:"
    echo "1. Make sure all required files are present"
    echo "2. Check .dockerignore isn't excluding required files"
    echo "3. Ensure Docker daemon is running"
    echo "4. Try cleaning Docker cache: docker builder prune"
    echo "5. For buildx issues, try: docker buildx use default"
    exit 1
fi