# Docker Guide for Rust Tour

This guide covers building, testing, and deploying Rust Tour using Docker.

## Quick Start

### Build the Image
```bash
# Simple build
./scripts/build-docker.sh

# Build with specific tag
./scripts/build-docker.sh v1.0.0

# Build with buildx (multi-platform support)
./scripts/build-docker.sh latest --buildx
```

### Test the Image
```bash
# Test the built image
./scripts/test-docker.sh

# Test specific image
./scripts/test-docker.sh rust-tour:v1.0.0
```

### Run the Container
```bash
# Quick start
docker run -d -p 3000:3000 rust-tour

# With progress persistence
docker run -d -p 3000:3000 -v $(pwd)/progress:/app/progress rust-tour

# With custom name and restart policy
docker run -d \
  --name rust-tour \
  --restart unless-stopped \
  -p 3000:3000 \
  -v $(pwd)/progress:/app/progress \
  rust-tour
```

## Building Docker Images

### Prerequisites

Before building, ensure you have:
- Docker installed and running
- All source files in place
- Required dependencies

### Build Script Options

The `./scripts/build-docker.sh` script supports several options:

```bash
# Basic build
./scripts/build-docker.sh

# With custom tag
./scripts/build-docker.sh v1.0.0

# Multi-platform build
./scripts/build-docker.sh latest --multi-platform

# Using buildx
./scripts/build-docker.sh latest --buildx
```

### Manual Build Commands

```bash
# Standard Docker build
docker build -t rust-tour .

# Build with specific tag
docker build -t rust-tour:v1.0.0 .

# Multi-platform build with buildx
docker buildx build --platform linux/amd64,linux/arm64 -t rust-tour --load .

# Build and push to registry
docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/ghanithan/rust-tour --push .
```

## Dockerfile Architecture

The Dockerfile uses a multi-stage build:

### Stage 1: Web Builder (`node:18-alpine`)
- Installs Node.js dependencies
- Builds the web frontend with Vite
- Creates optimized production assets

### Stage 2: Rust Builder (`rust:1.75-alpine`)
- Installs Rust toolchain and build dependencies
- Copies and builds Rust components
- Creates optimized release binary with embedded assets
- Includes exercise content and scripts

### Stage 3: Runtime (`alpine:3.18`)
- Minimal Alpine Linux base
- Non-root user for security
- Health checks configured
- Only runtime dependencies

## Configuration

### Environment Variables

```bash
# Server configuration
RUST_LOG=info                    # Logging level
RUST_TOUR_HOST=0.0.0.0          # Bind address
RUST_TOUR_PORT=3000             # Port number

# Development
RUST_BACKTRACE=1                # Enable backtraces
```

### Volume Mounts

```bash
# Progress persistence
-v $(pwd)/progress:/app/progress

# Exercise development (read-only)
-v $(pwd)/exercises:/app/exercises:ro

# Log access
-v $(pwd)/logs:/app/logs
```

### Port Mapping

```bash
# Standard mapping
-p 3000:3000

# Custom port
-p 8080:3000

# Bind to specific interface
-p 127.0.0.1:3000:3000
```

## Testing Docker Images

### Automated Testing

Use the test script for comprehensive validation:

```bash
./scripts/test-docker.sh
```

This script tests:
- Container startup
- Health endpoints
- API functionality
- Resource usage
- Log analysis

### Manual Testing

```bash
# Start container in foreground
docker run -it --rm -p 3000:3000 rust-tour

# Execute commands in running container
docker exec -it rust-tour sh

# View logs
docker logs rust-tour -f

# Check health
curl http://localhost:3000/health
```

### Performance Testing

```bash
# Check resource usage
docker stats rust-tour

# Memory limit test
docker run -d --memory=256m -p 3000:3000 rust-tour

# CPU limit test
docker run -d --cpus=0.5 -p 3000:3000 rust-tour
```

## Production Deployment

### Docker Compose (Recommended)

```yaml
# docker-compose.yml
version: '3.8'
services:
  rust-tour:
    image: ghcr.io/ghanithan/rust-tour:latest
    ports:
      - "3000:3000"
    volumes:
      - ./progress:/app/progress
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### Docker Swarm

```bash
# Deploy to swarm
docker service create \
  --name rust-tour \
  --publish 3000:3000 \
  --mount type=volume,source=rust-tour-progress,target=/app/progress \
  --replicas 3 \
  ghcr.io/ghanithan/rust-tour:latest
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-tour
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-tour
  template:
    metadata:
      labels:
        app: rust-tour
    spec:
      containers:
      - name: rust-tour
        image: ghcr.io/ghanithan/rust-tour:latest
        ports:
        - containerPort: 3000
        volumeMounts:
        - name: progress
          mountPath: /app/progress
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
      volumes:
      - name: progress
        persistentVolumeClaim:
          claimName: rust-tour-progress
```

## Registry Management

### GitHub Container Registry (GHCR)

```bash
# Login to GHCR
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Tag for GHCR
docker tag rust-tour ghcr.io/ghanithan/rust-tour:latest

# Push to GHCR
docker push ghcr.io/ghanithan/rust-tour:latest

# Pull from GHCR
docker pull ghcr.io/ghanithan/rust-tour:latest
```

### Image Tagging Strategy

```bash
# Production tags (with embedded assets)
ghcr.io/ghanithan/rust-tour:v1.0.0     # Specific version
ghcr.io/ghanithan/rust-tour:1.0        # Minor version
ghcr.io/ghanithan/rust-tour:1          # Major version
ghcr.io/ghanithan/rust-tour:latest     # Latest stable

# Development tags (without embedded assets)
ghcr.io/ghanithan/rust-tour:main       # Main branch
ghcr.io/ghanithan/rust-tour:dev        # Development
ghcr.io/ghanithan/rust-tour:pr-123     # Pull request builds
```

**Note on Build Differences:**
- **Production images** (version tags, `latest`): Built with `Dockerfile.prod`, includes embedded web assets for single-binary deployment
- **Development images** (`main`, PR builds): Built with standard `Dockerfile`, uses separate web files for easier debugging and faster builds

## Troubleshooting

### Common Issues

#### Build Failures

**Problem**: `Cargo.lock not found`
```bash
# Solution: Check .dockerignore
grep -n Cargo.lock .dockerignore
# Remove Cargo.lock from .dockerignore if present
```

**Problem**: `solutions/ directory not found`
```bash
# Solution: Create directory or make copy optional
mkdir -p solutions/
# Or update Dockerfile to handle missing directory
```

#### Runtime Issues

**Problem**: Container starts but app doesn't respond
```bash
# Check logs
docker logs rust-tour

# Check if port is bound
docker port rust-tour

# Test health endpoint
docker exec rust-tour wget -q --spider http://localhost:3000/health
```

**Problem**: Permission denied for volumes
```bash
# Fix volume permissions
sudo chown -R 1001:1001 ./progress
# Or run with user mapping
docker run --user $(id -u):$(id -g) -v $(pwd)/progress:/app/progress rust-tour
```

### Debug Commands

```bash
# Enter container shell
docker exec -it rust-tour sh

# Check running processes
docker exec rust-tour ps aux

# Check file permissions
docker exec rust-tour ls -la /app

# Check disk usage
docker exec rust-tour df -h

# Check network connectivity
docker exec rust-tour wget -q --spider http://localhost:3000/health
```

### Performance Optimization

#### Build Optimization

```bash
# Use build cache
export DOCKER_BUILDKIT=1

# Multi-stage build optimization
docker build --target rust-builder -t rust-tour-builder .

# BuildKit cache mounts
docker buildx build --cache-from type=local,src=.buildcache .
```

#### Runtime Optimization

```bash
# Limit resources
docker run --memory=512m --cpus=1.0 rust-tour

# Use read-only filesystem
docker run --read-only --tmpfs /tmp rust-tour

# Security options
docker run --security-opt=no-new-privileges --cap-drop=ALL rust-tour
```

## Best Practices

### Security

1. **Non-root user**: Images run as non-root by default
2. **Minimal base**: Alpine Linux for smaller attack surface
3. **No secrets in images**: Use environment variables or secrets management
4. **Regular updates**: Keep base images and dependencies updated

### Performance

1. **Multi-stage builds**: Minimize final image size
2. **Layer caching**: Optimize Dockerfile for better caching
3. **Resource limits**: Set appropriate CPU and memory limits
4. **Health checks**: Implement proper health monitoring

### Maintenance

1. **Image scanning**: Regular security vulnerability scans
2. **Automated builds**: CI/CD pipeline for consistent builds
3. **Version tagging**: Semantic versioning for releases
4. **Cleanup**: Regular removal of old images and containers

This comprehensive Docker setup ensures reliable, secure, and efficient deployment of Rust Tour across different environments.