#!/bin/bash
set -e

echo "🦀 Setting up Rust Learning Platform..."

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

# Check if we're in Codespaces
if [ -n "$CODESPACES" ]; then
    print_status "Detected GitHub Codespaces environment"
    export IN_CODESPACES=true
else
    print_status "Local development environment detected"
    export IN_CODESPACES=false
fi

# Update Rust toolchain
print_status "Updating Rust toolchain..."
rustup update stable
rustup component add clippy rustfmt

# Install cargo tools
print_status "Installing cargo tools..."
cargo install cargo-watch cargo-edit || print_warning "Some cargo tools may already be installed"

# Setup Node.js dependencies for web UI
if [ -d "web" ]; then
    print_status "Installing Node.js dependencies..."
    cd web
    npm install
    cd ..
else
    print_status "Creating web directory structure..."
    mkdir -p web/{src,public,dist}
fi

# Create initial progress tracking
print_status "Initializing progress tracking..."
mkdir -p progress
if [ ! -f "progress/user_progress.json" ]; then
    cat > progress/user_progress.json << 'EOF'
{
  "user_id": "default",
  "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "overall_progress": 0.0,
  "chapters_completed": 0,
  "exercises_completed": 0,
  "total_time_minutes": 0,
  "current_streak": 0,
  "longest_streak": 0,
  "chapters": {},
  "preferences": {
    "difficulty_preference": "adaptive",
    "hint_level": "progressive",
    "theme": "rust"
  }
}
EOF
fi

# Build the exercise framework
print_status "Building exercise framework..."
if [ -d "exercise-framework" ]; then
    cd exercise-framework
    cargo build --release
    cd ..
fi

# Build CLI tool if it exists
if [ -d "cli" ]; then
    print_status "Building CLI tool..."
    cd cli
    cargo build --release
    cd ..
    
    # Add to PATH if in Codespaces
    if [ "$IN_CODESPACES" = true ]; then
        echo 'export PATH="$PATH:'$(pwd)'/target/release"' >> ~/.bashrc
    fi
fi

# Set up git hooks for progress tracking
print_status "Setting up git hooks..."
mkdir -p .git/hooks
cat > .git/hooks/post-commit << 'EOF'
#!/bin/bash
# Auto-sync progress after commits
if [ -f "./scripts/sync-progress.sh" ]; then
    ./scripts/sync-progress.sh
fi
EOF
chmod +x .git/hooks/post-commit

# Create necessary directories
print_status "Creating directory structure..."
mkdir -p {docs,logs,tmp}

# Set permissions
chmod +x scripts/*.sh 2>/dev/null || true

print_success "Setup completed successfully! 🎉"
print_status ""
print_status "Next steps:"
print_status "1. Start web UI: ./scripts/start-web.sh"
print_status "2. Use CLI tool: cargo run --bin rust-learn-cli"
print_status "3. Begin learning: ./scripts/start-learning.sh"
print_status ""

if [ "$IN_CODESPACES" = true ]; then
    print_status "🌟 Codespaces detected! The web UI will auto-forward to port 3000"
    print_status "💡 Try: cargo run --bin rust-learn-cli status"
else
    print_status "🏠 Local setup complete! Run 'source ~/.bashrc' to update PATH"
fi

print_success "Happy learning! 🦀📚"