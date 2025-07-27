# Installation & Setup Guide

This comprehensive guide covers all installation methods for Rust Tour, troubleshooting common issues, and optimizing your learning environment.

## Prerequisites

### System Requirements
- **Operating System**: Linux, macOS, or Windows (WSL2 recommended for Windows)
- **RAM**: 4GB minimum, 8GB recommended
- **Disk Space**: 2GB free space
- **Network**: Internet connection for initial setup and updates

### Required Software (Method Dependent)
- **Git**: Required for all local installations
- **Rust Toolchain**: Required for local development (1.75.0 or later)
- **Node.js**: Required for development contributions (18.0 or later)
- **Docker**: Optional, for containerized deployment

## Installation Methods

### Method 1: GitHub Codespaces (Recommended for Beginners)

**Advantages**: No local setup, pre-configured environment, accessible from any device

#### With Progress Tracking (Recommended)
1. **Fork the repository**
   - Go to https://github.com/ghanithan/rust-tour
   - Click "Fork" in the top-right corner
   - Choose your GitHub account as the destination

2. **Launch Codespace**
   - Navigate to your forked repository
   - Click "Code" → "Codespaces" → "Create codespace on main"
   - Wait 2-3 minutes for environment initialization

3. **Start Rust Tour**
   ```bash
   ./scripts/run.sh start
   ```

4. **Access the application**
   - Codespaces will automatically forward port 3000
   - Click the notification or go to the "Ports" tab
   - Open the forwarded URL in your browser

#### Without Progress Tracking
- Follow the same steps but use the original repository instead of forking
- Your progress won't be saved between sessions

### Method 2: Cargo Installation (Simplest Local Setup)

**Advantages**: Single command installation, offline capability, lightweight

```bash
# Install Rust Tour
cargo install rust-tour

# Start learning
rust-tour
```

The application will be available at `http://localhost:3000`

**Note**: This provides a standalone binary with embedded content, perfect for offline learning.

### Method 3: Docker Installation

**Advantages**: Isolated environment, consistent across systems, easy cleanup

1. **Clone and setup**
   ```bash
   git clone https://github.com/ghanithan/rust-tour.git
   cd rust-tour
   ```

2. **Run with Docker Compose**
   ```bash
   docker-compose up -d
   ```

3. **Access the application**
   - Open `http://localhost:3000` in your browser

4. **Stop the service**
   ```bash
   docker-compose down
   ```

### Method 4: Local Repository (End Users)

**Advantages**: Full control, fastest performance, works offline

1. **Prerequisites check**
   ```bash
   git --version  # Should show git version
   ```

2. **Clone and start**
   ```bash
   git clone https://github.com/ghanithan/rust-tour.git
   cd rust-tour
   ./scripts/run.sh start
   ```

3. **Access the application**
   - Open `http://localhost:3000` in your browser

### Method 5: Development Setup (Contributors)

**Advantages**: Hot reloading, debugging capabilities, contribution-ready

1. **Prerequisites installation**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   
   # Install Node.js (via package manager or nvm)
   # Ubuntu/Debian:
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   
   # macOS (with Homebrew):
   brew install node
   ```

2. **Clone and setup**
   ```bash
   git clone https://github.com/ghanithan/rust-tour.git
   cd rust-tour
   ./scripts/run.sh setup
   ```

3. **Start development servers**
   ```bash
   ./scripts/run.sh dev
   ```

   This starts both the Rust API server and Vite development server with hot reloading.

## Configuration

### Environment Variables

Create a `.env` file in the project root for custom configuration:

```bash
# Server Configuration
RUST_TOUR_PORT=3000
RUST_TOUR_HOST=localhost

# Development Settings
RUST_LOG=info
RUST_BACKTRACE=1

# Progress Tracking
PROGRESS_FILE=progress/user_progress.json
```

### Browser Configuration

For the best experience, configure your browser:

1. **Enable JavaScript**: Required for the interactive interface
2. **Allow local storage**: For saving progress and preferences
3. **Disable popup blockers**: For the integrated terminal
4. **Modern browser**: Chrome 90+, Firefox 88+, Safari 14+, or Edge 90+

## Verification

After installation, verify everything works:

```bash
# Check if the server starts
curl http://localhost:3000/health

# Run a simple exercise test
cd exercises/ch01_getting_started/ex01_hello_world
cargo test
```

## Next Steps

1. **Complete your profile**: Set your learning preferences in the web interface
2. **Start with Chapter 1**: Begin with basic concepts and build up
3. **Join the community**: Introduce yourself in [GitHub Discussions](https://github.com/ghanithan/rust-tour/discussions)
4. **Bookmark resources**: Save the [Exercise Guide](Exercise-Guide) and [FAQ](FAQ-Troubleshooting) for quick reference

## Need Help?

- **Installation Issues**: See [FAQ & Troubleshooting](FAQ-Troubleshooting)
- **Platform Questions**: Visit [FAQ & Troubleshooting](FAQ-Troubleshooting)
- **Community Support**: Ask in [GitHub Discussions](https://github.com/ghanithan/rust-tour/discussions)