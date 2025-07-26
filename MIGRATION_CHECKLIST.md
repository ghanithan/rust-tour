# Migration Checklist: Node.js → Rust Server

## Core Infrastructure
- [x] Express server → Axum server
- [x] HTTP server creation with upgrade support
- [x] WebSocket server with connection management
- [x] Graceful shutdown handling
- [x] Error handling and logging

## Middleware & Security
- [x] Helmet security headers → Tower middleware
- [x] CORS support
- [x] Morgan logging → tracing
- [x] JSON body parsing (50mb limit)
- [x] URL encoded body parsing → Axum extractors
- [x] Static file serving (`/dist`, `/monaco`)

## WebSocket Features
- [x] Connection set management
- [x] Terminal session map storage
- [x] Broadcast function to all clients
- [x] WebSocket message parsing and routing
- [x] Terminal message handling with actions:
  - [x] `create` - Create new terminal session
  - [x] `check` - Check if session exists
  - [x] `input` - Send input to terminal
  - [x] `resize` - Resize terminal
  - [x] `destroy` - Destroy terminal session

## Terminal Management
- [x] `createTerminalSession()` - PTY process creation
- [x] `checkTerminalSession()` - Session validation
- [x] `sendInputToTerminal()` - Input forwarding
- [x] `resizeTerminal()` - Terminal resizing
- [x] `destroyTerminalSession()` - Cleanup
- [x] Cross-platform shell detection (bash/powershell)
- [x] Terminal data streaming
- [x] Session cleanup on disconnect

## API Endpoints
- [x] `GET /api/exercises` - List all exercises with metadata
- [x] `GET /api/exercises/:chapter/:exercise` - Get specific exercise
- [x] `PUT /api/exercises/:chapter/:exercise/code` - Save exercise code
- [x] `POST /api/exercises/:chapter/:exercise/test` - Run cargo test
- [x] `POST /api/exercises/:chapter/:exercise/run` - Run cargo run
- [x] `POST /api/exercises/:chapter/:exercise/check` - Run cargo clippy
- [x] `GET /api/progress` - Get user progress
- [x] `POST /api/progress/complete` - Mark exercise complete
- [x] `POST /api/progress/hint` - Track hint usage
- [x] `POST /api/progress/view` - Track exercise view
- [x] `GET /api/book/:chapter` - Get book chapter info
- [x] `GET *` - Serve React app (catch-all)

## Helper Functions
- [x] `runCargoCommand()` - Execute cargo with args, return result
- [x] `countTotalExercises()` - Scan and count exercise directories
- [x] `ensureProgressFile()` - Create/validate progress.json structure
- [x] `initializeProgressSystem()` - Startup progress initialization

## File Operations
- [x] Exercise directory scanning (chapters/exercises)
- [x] Metadata.json loading and parsing
- [x] Source file reading (main.rs, README.md, hints.md)
- [x] Code file writing (src/main.rs)
- [x] Progress file management (user_progress.json)
- [x] File watching with chokidar → notify

## Progress System Features
- [x] Default progress structure creation
- [x] Exercise completion tracking (avoid duplicates)
- [x] Hint usage tracking
- [x] Exercise view tracking
- [x] Session statistics
- [x] Time tracking
- [x] Exercise history management
- [x] Backwards compatibility for progress files

## File Watching & Broadcasting
- [x] Watch exercises directory for changes
- [x] Parse file paths to extract exercise info
- [x] Load exercise metadata for titles
- [x] Broadcast file change events to all connected clients
- [x] Handle metadata loading failures gracefully

## Environment & Configuration
- [x] PORT environment variable (default 3000)
- [x] DEBUG_WEBSOCKET environment variable
- [x] Working directory management
- [x] Process environment passing to terminals

## Error Handling
- [x] Cargo command execution errors
- [x] File system operation errors
- [x] JSON parsing errors
- [x] WebSocket connection errors
- [x] Terminal session errors
- [x] Progress file operation errors

## Logging & Debugging
- [x] Server startup messages
- [x] WebSocket connection/disconnection logs
- [x] Terminal session creation/destruction logs
- [x] Exercise completion logs
- [x] Error logging with context
- [x] Debug mode for WebSocket messages

## Special Behaviors
- [x] Exercise title extraction from metadata vs fallback to dir name
- [x] File path relative to exercises directory
- [x] Cross-platform path handling
- [x] Session ID generation and management
- [x] Memory cleanup on client disconnect
- [x] Automatic terminal session reuse
- [x] Progress file total exercises count updates