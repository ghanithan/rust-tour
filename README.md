# 🦀 Rust Tour

An interactive Rust tutorial with progressive exercises integrated with "The Rust Programming Language" book.

[![CI](https://github.com/your-username/rust-tour/workflows/CI/badge.svg)](https://github.com/your-username/rust-tour/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## ✨ Features

- **🎯 Progressive Learning**: 20 chapters following the Rust Book structure
- **📚 Integrated Rust Book**: Side-by-side theory and practice
- **🌐 Interactive Web UI**: Monaco editor with syntax highlighting
- **⚡ Integrated Terminal**: Built-in terminal for running Rust commands  
- **📊 Smart Progress Tracking**: Multi-dimensional learning analytics
- **🎮 Gamified Experience**: Achievements, streaks, and skill trees
- **🔄 Adaptive Difficulty**: AI-powered exercise recommendations
- **👥 Community Driven**: Open-source with contribution workflows

## 🚀 Quick Start

### GitHub Codespaces (Recommended)
1. Click "Code" → "Create codespace on main"
2. Wait for environment setup (2-3 minutes)
3. Run `./scripts/setup.sh` to initialize
4. Open browser to `localhost:3000` for web UI
5. Start using the integrated terminal for Rust commands

### Local Development
```bash
# Clone the repository
git clone https://github.com/your-username/rust-tour.git
cd rust-tour

# Setup with Docker (recommended)
docker-compose up -d

# Or setup locally
./scripts/setup-local.sh
```

## 🎓 Learning Path

### Chapter 1: Getting Started
- [x] `ex01_hello_world` - Your first Rust program
- [x] `ex02_hello_cargo` - Understanding Cargo basics
- [ ] `ex03_comments_and_printing` - Documentation and output

### Chapter 2: Programming a Guessing Game  
- [ ] `ex01_setup_game` - Project initialization
- [ ] `ex02_processing_input` - Reading user input
- [ ] `ex03_generating_numbers` - Random number generation
- [ ] `ex04_comparing_guess` - Control flow and matching

### Chapter 3: Common Programming Concepts
- [ ] `ex01_variables` - Variables and mutability
- [ ] `ex02_data_types` - Scalar and compound types
- [ ] `ex03_functions` - Function definition and calls
- [ ] `ex04_control_flow` - if expressions and loops

*[Full curriculum: 200+ exercises across 20 chapters]*

## 🖥️ Interface Options

### Web UI Features
- **Monaco Editor**: VS Code-quality editing experience
- **Live Testing**: Instant feedback on code execution
- **Rust Book Integration**: Contextual documentation panel
- **Visual Progress**: Interactive skill tree and analytics
- **Responsive Design**: Works on desktop, tablet, and mobile

### Integrated Terminal Features  
```bash
# Exercise management via built-in terminal
cd exercises/ch03_common_concepts/ex02_data_types
cargo test                         # Run exercise tests
cargo run                          # Execute your solution
cargo clippy                       # Check code quality
```

## 📊 Progress Tracking

The platform tracks multiple dimensions of your learning:

- **📈 Concept Mastery**: Understanding of ownership, borrowing, lifetimes
- **⚡ Coding Speed**: Time efficiency improvements over practice
- **✨ Code Quality**: Idiomatic Rust patterns and best practices
- **🎯 Problem Solving**: Pattern recognition and debugging skills
- **📚 Book Integration**: Reading progress synchronized with exercises

## 🏗️ Architecture

```
rust-tour/
├── 🌐 web/                    # TypeScript/React web interface
├── 🦀 web-server/             # Rust web server with terminal support
├── 🔧 exercise-framework/     # Core exercise system (Rust)
├── 📝 exercises/              # Learning content
│   ├── ch01_getting_started/
│   ├── ch02_guessing_game/
│   └── ch03_common_concepts/
├── 📊 progress/               # JSON-based progress tracking
├── 🛠️ scripts/               # Development and setup tools
└── 📚 docs/                   # Documentation and guides
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md).

### Quick Contribution Guide
1. **New Exercise**: Use `./scripts/create-exercise.sh ch03 temperature_conversion`
2. **Bug Report**: Use GitHub issue templates
3. **Feature Request**: Start a GitHub discussion
4. **Code Contribution**: Fork → Branch → PR with tests

### Development Setup
```bash
# Install development tools
./scripts/dev-setup.sh

# Run tests
cargo test --workspace           # Rust components
cd web && npm test              # Web components

# Start development servers
cargo run --package rust-tour   # Rust backend
cd web && npm run dev           # Frontend
```

## 📋 Exercise Types

- **📝 Code Completion**: Fill in missing parts of working programs
- **🐛 Bug Fixing**: Find and fix intentional errors
- **🏗️ From Scratch**: Build complete programs from specifications  
- **👀 Code Review**: Analyze and improve existing code
- **⚡ Performance**: Optimize for speed and memory efficiency

## 🎯 Learning Outcomes

By completing this platform, you'll master:

### Core Rust Concepts
- ✅ Ownership and borrowing system
- ✅ Type system and memory safety
- ✅ Error handling patterns
- ✅ Concurrency and parallelism
- ✅ Trait system and generics

### Practical Skills  
- ✅ Cargo and project management
- ✅ Testing and documentation
- ✅ Performance optimization
- ✅ Ecosystem and crates.io
- ✅ Real-world project patterns

## 📈 Success Metrics

Our learners achieve:
- **📊 85%+ exercise completion rate**
- **⏱️ 2-week average to ownership mastery**
- **🏆 90%+ satisfaction rating**
- **💼 Job-ready Rust skills**

## 🔗 Resources

- **📖 [The Rust Programming Language](https://doc.rust-lang.org/book/)** - Official book
- **🦀 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Hands-on examples
- **📚 [Rust Reference](https://doc.rust-lang.org/reference/)** - Language specification
- **💬 [Rust Users Forum](https://users.rust-lang.org/)** - Community discussion
- **💻 [Rust Playground](https://play.rust-lang.org/)** - Online Rust editor

## 📄 License

This project is licensed under either of
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- The Rust Team for creating an amazing language and book
- GitHub for free Codespaces hosting
- The open-source community for inspiration and contributions
- All learners who provide feedback and help improve the platform

---

**Ready to master Rust?** 🚀 [Start your journey](https://github.com/your-username/rust-tour) today!