use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path as AxumPath, Query, State,
    },
    http::{header, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use clap::Parser;
use futures_util::{sink::SinkExt, stream::StreamExt};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use regex::Regex;
use reqwest;
use scraper::{Html as ScraperHtml, Selector};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env,
    io::{Read, Write},
    net::SocketAddr,
    path::PathBuf,
    process::Stdio,
    sync::Arc,
    thread,
    time::Duration,
};
use tokio::{
    fs,
    process::Command,
    sync::{broadcast, RwLock, Mutex},
    time::timeout,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use walkdir::WalkDir;

#[cfg(feature = "embed-assets")]
use rust_embed::RustEmbed;

#[cfg(feature = "download-exercises")]
use dialoguer::{Confirm, Input, Select};
#[cfg(feature = "download-exercises")]
use git2::Repository;
#[cfg(feature = "download-exercises")]
use tempfile::TempDir;
#[cfg(feature = "download-exercises")]
use open;

#[cfg(feature = "embed-assets")]
#[derive(RustEmbed)]
#[folder = "web-dist/"]
struct Assets;

#[cfg(not(feature = "embed-assets"))]
struct Assets;

/// Rust Tour - An interactive Rust learning platform
/// 
/// Rust Tour provides a structured pathway to learn Rust through progressive,
/// test-driven exercises aligned with "The Rust Programming Language" book.
/// 
/// When installed via cargo, exercises will be downloaded on first run to a
/// directory of your choice. Your progress is tracked locally and persists
/// between sessions.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port to run the server on
    #[arg(short, long, default_value = "3000", env = "PORT")]
    port: u16,
    
    /// Enable debug logging for WebSocket connections
    #[arg(long, env = "DEBUG_WEBSOCKET")]
    debug_websocket: bool,
    
    /// Custom path to exercises directory (for development)
    #[arg(long)]
    exercises_path: Option<PathBuf>,
}

// Application state
#[derive(Clone)]
struct AppState {
    connections: Arc<RwLock<HashSet<ConnectionId>>>,
    terminal_sessions: Arc<RwLock<HashMap<String, TerminalSession>>>,
    pty_handles: Arc<RwLock<HashMap<String, PtyHandle>>>,
    broadcast_tx: broadcast::Sender<BroadcastMessage>,
    debug_websocket: bool,
    exercises_path: PathBuf,
    progress_path: PathBuf,
}

type ConnectionId = Uuid;

#[derive(Debug, Clone)]
struct TerminalSession {
    session_id: String,
    connection_id: ConnectionId,
}

// Separate struct for actual PTY handles (not Clone/Send)
struct PtyHandle {
    writer: Arc<Mutex<Box<dyn std::io::Write + Send>>>,
    child: Arc<Mutex<Box<dyn portable_pty::Child + Send + Sync>>>,
    master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
}

#[derive(Debug, Clone, Serialize)]
struct BroadcastMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(flatten)]
    data: serde_json::Value,
}

// WebSocket message types
#[derive(Debug, Deserialize)]
struct WebSocketMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(flatten)]
    data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct TerminalMessage {
    action: String,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    input: Option<String>,
    cols: Option<u16>,
    rows: Option<u16>,
}

// API response types
#[derive(Debug, Serialize)]
struct CargoResult {
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
    output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExerciseMetadata {
    id: String,
    title: String,
    description: String,
    chapter: u32,
    exercise_number: u32,
    difficulty: String,
    estimated_time_minutes: u32,
    concepts: Vec<String>,
    prerequisites: Vec<String>,
    exercise_type: String,
    #[serde(default)]
    rust_book_refs: serde_json::Value,
    #[serde(default)]
    hints: serde_json::Value,
    #[serde(default)]
    testing: serde_json::Value,
    #[serde(default)]
    validation: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct ExerciseWithPath {
    #[serde(flatten)]
    metadata: ExerciseMetadata,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileContent {
    path: String,
    content: String,
    language: String,
    editable: bool,
}

#[derive(Debug, Serialize)]
struct ExerciseDetails {
    metadata: ExerciseMetadata,
    #[serde(rename = "mainContent")]
    main_content: String,
    readme: String,
    hints: String,
    path: String,
    files: Vec<FileContent>,  // New field for all files
}

#[derive(Debug, Serialize, Deserialize)]
struct ProgressData {
    user_id: String,
    #[serde(default = "default_created_at")]
    created_at: String,
    #[serde(default)]
    overall_progress: f64,
    #[serde(default)]
    chapters_completed: u32,
    #[serde(default)]
    exercises_completed: u32,
    #[serde(default = "default_total_exercises")]
    total_exercises: u32,
    #[serde(default)]
    current_streak: u32,
    #[serde(default)]
    longest_streak: u32,
    #[serde(default)]
    total_time_minutes: u32,
    #[serde(default = "default_chapters")]
    chapters: serde_json::Value,
    #[serde(default)]
    exercise_history: Vec<ExerciseHistoryEntry>,
    #[serde(default)]
    achievements: Vec<serde_json::Value>,
    #[serde(default)]
    session_stats: SessionStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExerciseHistoryEntry {
    exercise_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_taken_minutes: Option<u32>,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hints_used: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionStats {
    #[serde(default)]
    exercises_viewed: u32,
    #[serde(default)]
    exercises_completed: u32,
    #[serde(default)]
    hints_used: u32,
    #[serde(default)]
    time_spent: u32,
}

// Default functions for serde
fn default_created_at() -> String {
    chrono::Utc::now().to_rfc3339()
}

fn default_total_exercises() -> u32 {
    3 // Default based on current exercises
}

fn default_chapters() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

impl Default for SessionStats {
    fn default() -> Self {
        Self {
            exercises_viewed: 0,
            exercises_completed: 0,
            hints_used: 0,
            time_spent: 0,
        }
    }
}

// Helper trait for string case conversion
trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        self.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Serialize)]
struct ChapterInfo {
    chapter_number: u32,
    title: String,
    exercises_completed: u32,
    total_exercises: u32,
    completion_percentage: f64,
    time_spent_minutes: u32,
}

#[derive(Debug, Deserialize)]
struct SaveCodeRequest {
    code: String,
}

#[derive(Debug, Deserialize)]
struct BatchSaveRequest {
    files: Vec<FileSaveRequest>,
}

#[derive(Debug, Deserialize)]
struct FileSaveRequest {
    path: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct FileOperationRequest {
    path: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CompleteExerciseRequest {
    exercise_id: String,
    time_taken_minutes: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct HintRequest {
    exercise_id: String,
    hint_level: u32,
}

#[derive(Debug, Deserialize)]
struct ViewRequest {
    exercise_id: String,
}

#[derive(Debug, Serialize)]
struct BookContentResponse {
    url: String,
    chapter: String,
    content: Option<String>,
    title: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(flatten)]
    data: Option<T>,
    #[serde(flatten)]
    extra: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: Some(true),
            message: None,
            error: None,
            data: Some(data),
            extra: None,
        }
    }

    fn success_with_extra(data: T, extra: serde_json::Value) -> Self {
        Self {
            success: Some(true),
            message: None,
            error: None,
            data: Some(data),
            extra: Some(extra),
        }
    }

    fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: Some(false),
            message: None,
            error: Some(message),
            data: None,
            extra: None,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_tour=info,tower_http=debug".into()),
        )
        .init();

    let port = cli.port;
    let debug_websocket = cli.debug_websocket;

    // Set up paths
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let exercises_path = cli.exercises_path.unwrap_or_else(|| current_dir.join("exercises"));

    // Check if exercises exist, download if needed (only for published binaries)
    #[cfg(feature = "download-exercises")]
    let exercises_path = {
        ensure_exercises_available(exercises_path).await?
    };
    
    #[cfg(not(feature = "download-exercises"))]
    let exercises_path = exercises_path;

    println!("🎯 Final exercises_path: {}", exercises_path.display());

    // Progress file goes in the parent directory (alongside exercises/)
    let progress_path = exercises_path.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("progress")
        .join("user_progress.json");
        
    println!("📊 Progress file: {}", progress_path.display());

    // Create broadcast channel for WebSocket messages
    let (broadcast_tx, _) = broadcast::channel(100);

    // Initialize application state
    let state = AppState {
        connections: Arc::new(RwLock::new(HashSet::new())),
        terminal_sessions: Arc::new(RwLock::new(HashMap::new())),
        pty_handles: Arc::new(RwLock::new(HashMap::new())),
        broadcast_tx: broadcast_tx.clone(),
        debug_websocket,
        exercises_path: exercises_path.clone(),
        progress_path: progress_path.clone(),
    };

    // Initialize progress system
    initialize_progress_system(&state).await?;

    // Set up file watching
    setup_file_watcher(state.clone()).await?;

    // Build the application router
    let app = create_router(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("\n🚀 Rust Tour is running!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("  🌐 Web interface:    http://localhost:{}", port);
    println!("  📡 WebSocket:        ws://localhost:{}/ws", port);
    println!("  🩺 Health check:     http://localhost:{}/health", port);
    println!();
    println!("  📚 Exercises path:   {}", exercises_path.display());
    println!("  💾 Progress path:    {}", progress_path.display());
    println!();
    println!("  Press Ctrl+C to stop the server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    info!("Server started on port {}", port);
    
    // Open browser automatically when download-exercises feature is enabled
    #[cfg(feature = "download-exercises")]
    {
        let url = format!("http://localhost:{}", port);
        println!("\n🌐 Opening browser to {}...", url);
        if let Err(e) = open::that(&url) {
            warn!("Failed to open browser automatically: {}", e);
            println!("   ⚠️  Please open your browser manually to: {}", url);
        }
    }

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Health check handler
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "rust-tour",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check route
        .route("/health", get(health_check))
        
        // WebSocket route
        .route("/ws", get(websocket_handler))
        
        // API routes
        .route("/api/exercises", get(get_exercises))
        .route("/api/exercises/:chapter/:exercise", get(get_exercise))
        .route("/api/exercises/:chapter/:exercise/code", put(save_exercise_code))
        .route("/api/exercises/:chapter/:exercise/files", put(save_exercise_files))
        .route("/api/exercises/:chapter/:exercise/file", post(create_exercise_file))
        .route("/api/exercises/:chapter/:exercise/file", delete(delete_exercise_file))
        .route("/api/exercises/:chapter/:exercise/test", post(test_exercise))
        .route("/api/exercises/:chapter/:exercise/run", post(run_exercise))
        .route("/api/exercises/:chapter/:exercise/check", post(check_exercise))
        .route("/api/progress", get(get_progress))
        .route("/api/progress/complete", post(complete_exercise))
        .route("/api/progress/hint", post(track_hint_usage))
        .route("/api/progress/view", post(track_exercise_view))
        .route("/api/book/:chapter", get(get_book_chapter))
        .route("/api/book/fetch", get(get_book_by_url))
        
        // Static file routes
        .fallback(serve_static_files)
        
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024)) // 50MB limit
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any)
                        .allow_origin(Any),
                )
        )
        .with_state(state)
}

// WebSocket handlers
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: WebSocket, state: AppState) {
    let connection_id = Uuid::new_v4();
    
    // Add connection to state
    {
        let mut connections = state.connections.write().await;
        connections.insert(connection_id);
    }
    
    info!("Client connected to WebSocket: {}", connection_id);
    
    let mut broadcast_rx = state.broadcast_tx.subscribe();
    let (mut sender, mut receiver) = socket.split();
    
    // Spawn task to handle broadcast messages
    let _broadcast_state = state.clone();
    let broadcast_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = handle_websocket_message(text, &state, connection_id).await {
                    error!("Error handling WebSocket message: {}", e);
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    // Cleanup on disconnect
    {
        let mut connections = state.connections.write().await;
        connections.remove(&connection_id);
    }
    
    // Clean up terminal sessions for this connection
    cleanup_terminal_sessions(&state, connection_id).await;
    
    broadcast_task.abort();
    info!("Client disconnected from WebSocket: {}", connection_id);
}

async fn handle_websocket_message(
    text: String,
    state: &AppState,
    connection_id: ConnectionId,
) -> anyhow::Result<()> {
    let message: WebSocketMessage = serde_json::from_str(&text)?;
    
    if state.debug_websocket {
        debug!("Received WebSocket message: {}", message.msg_type);
    }
    
    match message.msg_type.as_str() {
        "terminal" => {
            let terminal_msg: TerminalMessage = serde_json::from_value(message.data)?;
            handle_terminal_message(state, connection_id, terminal_msg).await?;
        }
        "heartbeat" => {
            handle_heartbeat_message(state, connection_id, &message).await?;
        }
        "exercise_view" => {
            // Handle exercise view tracking
            if state.debug_websocket {
                debug!("Exercise view from connection {}", connection_id);
            }
        }
        "code_execution" => {
            // Handle code execution events
            if state.debug_websocket {
                debug!("Code execution from connection {}", connection_id);
            }
        }
        "progress_update" => {
            // Handle progress updates
            if state.debug_websocket {
                debug!("Progress update from connection {}", connection_id);
            }
        }
        _ => {
            warn!("Unknown WebSocket message type: {}", message.msg_type);
        }
    }
    
    Ok(())
}

async fn handle_heartbeat_message(
    state: &AppState,
    connection_id: ConnectionId,
    message: &WebSocketMessage,
) -> anyhow::Result<()> {
    if state.debug_websocket {
        debug!("Received heartbeat from connection {}", connection_id);
    }
    
    // Extract timestamp from heartbeat if available
    let timestamp = message.data.get("timestamp")
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
        );
    
    // Send heartbeat response (pong) back to client via broadcast
    let response = BroadcastMessage {
        msg_type: "heartbeat_response".to_string(),
        data: serde_json::json!({
            "timestamp": timestamp,
            "server_time": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        }),
    };
    
    // Send response via broadcast system
    let _ = state.broadcast_tx.send(response);
    
    Ok(())
}

async fn handle_terminal_message(
    state: &AppState,
    connection_id: ConnectionId,
    msg: TerminalMessage,
) -> anyhow::Result<()> {
    if state.debug_websocket {
        debug!("Handling terminal message: {}", msg.action);
    }
    
    match msg.action.as_str() {
        "create" => {
            let session_id = msg.session_id.unwrap_or_else(|| Uuid::new_v4().to_string());
            create_terminal_session(state, connection_id, session_id, msg.cols, msg.rows).await?;
        }
        "check" => {
            if let Some(session_id) = msg.session_id {
                check_terminal_session(state, connection_id, session_id).await?;
            }
        }
        "input" => {
            if let (Some(session_id), Some(input)) = (msg.session_id, msg.input) {
                send_input_to_terminal(state, session_id, input).await?;
            }
        }
        "resize" => {
            if let (Some(session_id), Some(cols), Some(rows)) = (msg.session_id, msg.cols, msg.rows) {
                resize_terminal(state, session_id, cols, rows).await?;
            }
        }
        "destroy" => {
            if let Some(session_id) = msg.session_id {
                destroy_terminal_session(state, session_id).await?;
            }
        }
        _ => {
            warn!("Unknown terminal action: {}", msg.action);
        }
    }
    
    Ok(())
}

// Terminal functions with full PTY integration
async fn create_terminal_session(
    state: &AppState,
    connection_id: ConnectionId,
    session_id: String,
    cols: Option<u16>,
    rows: Option<u16>,
) -> anyhow::Result<()> {
    // Check if session already exists
    {
        let sessions = state.terminal_sessions.read().await;
        if sessions.contains_key(&session_id) {
            // Update connection ID for existing session
            let mut sessions = state.terminal_sessions.write().await;
            if let Some(session) = sessions.get_mut(&session_id) {
                session.connection_id = connection_id;
            }
            send_terminal_response(state, &session_id, "created").await?;
            return Ok(());
        }
    }
    
    let cols = cols.unwrap_or(80);
    let rows = rows.unwrap_or(24);
    
    // Determine working directory and shell
    let cwd = state.exercises_path.clone();
    let shell = if cfg!(windows) {
        "powershell.exe"
    } else {
        "bash"
    };
    
    // Create PTY system
    let pty_system = native_pty_system();
    let pty_size = PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    };
    
    // Create PTY pair
    let pty_pair = pty_system.openpty(pty_size)?;
    
    // Spawn shell process
    let mut cmd = CommandBuilder::new(shell);
    cmd.cwd(&cwd);
    cmd.env("TERM", "xterm-color");
    
    let child = pty_pair.slave.spawn_command(cmd)?;
    
    // Get reader and writer
    let reader = pty_pair.master.try_clone_reader()?;
    let writer = pty_pair.master.take_writer()?;
    let master = pty_pair.master;
    
    // Create session
    let session = TerminalSession {
        session_id: session_id.clone(),
        connection_id,
    };
    
    let pty_handle = PtyHandle {
        writer: Arc::new(Mutex::new(writer)),
        child: Arc::new(Mutex::new(child)),
        master: Arc::new(Mutex::new(master)),
    };
    
    // Store session and handle
    {
        let mut sessions = state.terminal_sessions.write().await;
        sessions.insert(session_id.clone(), session);
    }
    
    {
        let mut handles = state.pty_handles.write().await;
        handles.insert(session_id.clone(), pty_handle);
    }
    
    // Spawn task to read PTY output and send to WebSocket
    let state_clone = state.clone();
    let session_id_clone = session_id.clone();
    tokio::spawn(async move {
        // Move reader to blocking thread for reading
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        
        thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0; 1024];
            
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        let data = buffer[..n].to_vec();
                        if tx.blocking_send(data).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        
        // Handle data in async context
        while let Some(data) = rx.recv().await {
            let data_str = String::from_utf8_lossy(&data).to_string();
            
            let message = BroadcastMessage {
                msg_type: "terminal".to_string(),
                data: serde_json::json!({
                    "action": "output",
                    "sessionId": session_id_clone,
                    "data": data_str
                }),
            };
            
            let _ = state_clone.broadcast_tx.send(message);
        }
        
        // Send exit message when PTY closes
        let exit_message = BroadcastMessage {
            msg_type: "terminal".to_string(),
            data: serde_json::json!({
                "action": "exit",
                "sessionId": session_id_clone
            }),
        };
        
        let _ = state_clone.broadcast_tx.send(exit_message);
        
        // Clean up session
        {
            let mut sessions = state_clone.terminal_sessions.write().await;
            sessions.remove(&session_id_clone);
        }
        {
            let mut handles = state_clone.pty_handles.write().await;
            handles.remove(&session_id_clone);
        }
    });
    
    send_terminal_response(state, &session_id, "created").await?;
    
    if state.debug_websocket {
        info!("Terminal session {} created with PTY", session_id);
    }
    
    Ok(())
}

async fn check_terminal_session(
    state: &AppState,
    connection_id: ConnectionId,
    session_id: String,
) -> anyhow::Result<()> {
    let sessions = state.terminal_sessions.read().await;
    let handles = state.pty_handles.read().await;
    
    if let (Some(_session), Some(_handle)) = (sessions.get(&session_id), handles.get(&session_id)) {
        send_terminal_response(state, &session_id, "exists").await?;
        
        // Update connection ID for existing session
        drop(sessions);
        let mut sessions = state.terminal_sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.connection_id = connection_id;
        }
    } else {
        send_terminal_response(state, &session_id, "not_found").await?;
    }
    Ok(())
}

async fn send_input_to_terminal(
    state: &AppState,
    session_id: String,
    input: String,
) -> anyhow::Result<()> {
    let handles = state.pty_handles.read().await;
    if let Some(handle) = handles.get(&session_id) {
        let writer = handle.writer.clone();
        let input_clone = input.clone();
        
        // Move writing to blocking task since PTY writer is not async
        tokio::task::spawn_blocking(move || {
            if let Ok(mut writer) = writer.try_lock() {
                let _ = writer.write_all(input_clone.as_bytes());
                let _ = writer.flush();
            }
        }).await?;
        
        if state.debug_websocket {
            debug!("Sent input to terminal {}: {}", session_id, input);
        }
    } else {
        warn!("Terminal session {} not found for input", session_id);
    }
    Ok(())
}

async fn resize_terminal(
    state: &AppState,
    session_id: String,
    cols: u16,
    rows: u16,
) -> anyhow::Result<()> {
    let handles = state.pty_handles.read().await;
    if let Some(handle) = handles.get(&session_id) {
        let master = handle.master.clone();
        
        // Move resizing to blocking task since PTY operations are not async
        tokio::task::spawn_blocking(move || {
            if let Ok(master) = master.try_lock() {
                let new_size = PtySize {
                    rows,
                    cols,
                    pixel_width: 0,
                    pixel_height: 0,
                };
                let _ = master.resize(new_size);
            }
        }).await?;
        
        if state.debug_websocket {
            debug!("Resized terminal {} to {}x{}", session_id, cols, rows);
        }
    } else {
        warn!("Terminal session {} not found for resize", session_id);
    }
    Ok(())
}

async fn destroy_terminal_session(
    state: &AppState,
    session_id: String,
) -> anyhow::Result<()> {
    // Remove PTY handle and kill process
    {
        let mut handles = state.pty_handles.write().await;
        if let Some(handle) = handles.remove(&session_id) {
            let child = handle.child.clone();
            tokio::task::spawn_blocking(move || {
                if let Ok(mut child) = child.try_lock() {
                    let _ = child.kill();
                }
            });
        }
    }
    
    // Remove session
    {
        let mut sessions = state.terminal_sessions.write().await;
        sessions.remove(&session_id);
    }
    
    if state.debug_websocket {
        info!("Terminal session {} destroyed", session_id);
    }
    
    Ok(())
}

async fn cleanup_terminal_sessions(
    state: &AppState,
    connection_id: ConnectionId,
) -> anyhow::Result<()> {
    let mut sessions_to_remove = Vec::new();
    
    {
        let sessions = state.terminal_sessions.read().await;
        for (session_id, session) in sessions.iter() {
            if session.connection_id == connection_id {
                sessions_to_remove.push(session_id.clone());
            }
        }
    }
    
    // Clean up each session
    for session_id in sessions_to_remove {
        destroy_terminal_session(state, session_id).await?;
    }
    
    Ok(())
}

async fn send_terminal_response(
    state: &AppState,
    session_id: &str,
    action: &str,
) -> anyhow::Result<()> {
    let response = BroadcastMessage {
        msg_type: "terminal".to_string(),
        data: serde_json::json!({
            "action": action,
            "sessionId": session_id
        }),
    };
    
    let _ = state.broadcast_tx.send(response);
    Ok(())
}

// API handlers
async fn get_exercises(State(state): State<AppState>) -> Result<Json<Vec<ExerciseWithPath>>, StatusCode> {
    match scan_exercises(&state.exercises_path).await {
        Ok(exercises) => Ok(Json(exercises)),
        Err(e) => {
            error!("Error loading exercises: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_exercise(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<ExerciseDetails>, StatusCode> {
    // The client sends chapter="ch01_getting_started" and exercise="ex01_hello_world"
    // We need to join them correctly to match the directory structure
    let exercise_dir_path = state.exercises_path.join(&chapter).join(&exercise);
    let exercise_id = format!("{}/{}", chapter, exercise);
    
    match load_exercise_details(&exercise_dir_path, &exercise_id).await {
        Ok(details) => Ok(Json(details)),
        Err(e) => {
            error!("Error loading exercise {}/{}: {}", chapter, exercise, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn save_exercise_code(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
    Json(request): Json<SaveCodeRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    let main_path = exercise_path.join("src").join("main.rs");
    
    match fs::write(&main_path, &request.code).await {
        Ok(_) => {
            // Broadcast file change
            let exercise_name = match load_exercise_title(&exercise_path).await {
                Ok(title) => title,
                Err(_) => format!("{}/{}", chapter, exercise),
            };
            
            let broadcast_msg = BroadcastMessage {
                msg_type: "file_updated".to_string(),
                data: serde_json::json!({
                    "exercise": exercise_name,
                    "file": "src/main.rs"
                }),
            };
            
            let _ = state.broadcast_tx.send(broadcast_msg);
            
            Ok(Json(ApiResponse::success(())))
        }
        Err(e) => {
            error!("Error saving code for {}/{}: {}", chapter, exercise, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn save_exercise_files(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
    Json(request): Json<BatchSaveRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    
    // Validate and save each file
    for file in &request.files {
        // Validate path to prevent directory traversal
        if file.path.contains("..") || file.path.starts_with('/') {
            error!("Invalid file path: {}", file.path);
            return Err(StatusCode::BAD_REQUEST);
        }
        
        // Only allow editing certain files
        if !is_editable_file(&file.path) {
            error!("File not editable: {}", file.path);
            return Err(StatusCode::FORBIDDEN);
        }
        
        let file_path = exercise_path.join(&file.path);
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                error!("Error creating directory for {}: {}", file.path, e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        
        // Write file
        if let Err(e) = fs::write(&file_path, &file.content).await {
            error!("Error saving file {}: {}", file.path, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    // Broadcast file changes
    let exercise_name = match load_exercise_title(&exercise_path).await {
        Ok(title) => title,
        Err(_) => format!("{}/{}", chapter, exercise),
    };
    
    for file in &request.files {
        let broadcast_msg = BroadcastMessage {
            msg_type: "file_updated".to_string(),
            data: serde_json::json!({
                "exercise": &exercise_name,
                "file": &file.path
            }),
        };
        let _ = state.broadcast_tx.send(broadcast_msg);
    }
    
    Ok(Json(ApiResponse::success(())))
}

async fn create_exercise_file(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
    Json(request): Json<FileOperationRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    
    // Validate path
    if request.path.contains("..") || request.path.starts_with('/') {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Only allow creating files in src/ directory
    if !request.path.starts_with("src/") || !request.path.ends_with(".rs") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let file_path = exercise_path.join(&request.path);
    
    // Check if file already exists
    if file_path.exists() {
        return Err(StatusCode::CONFLICT);
    }
    
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        if let Err(e) = fs::create_dir_all(parent).await {
            error!("Error creating directory: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    // Create file with default content
    let content = request.content.unwrap_or_else(|| "// New file\n".to_string());
    if let Err(e) = fs::write(&file_path, content).await {
        error!("Error creating file: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(Json(ApiResponse::success(())))
}

async fn delete_exercise_file(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
    Json(request): Json<FileOperationRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    
    // Validate path
    if request.path.contains("..") || request.path.starts_with('/') {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Don't allow deleting critical files
    if request.path == "src/main.rs" || request.path == "Cargo.toml" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Only allow deleting files in src/ directory
    if !request.path.starts_with("src/") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let file_path = exercise_path.join(&request.path);
    
    // Delete the file
    if let Err(e) = fs::remove_file(&file_path).await {
        error!("Error deleting file: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(Json(ApiResponse::success(())))
}

// Helper function to check if a file is editable
fn is_editable_file(path: &str) -> bool {
    // Allow editing Cargo.toml and any file in src/
    path == "Cargo.toml" || 
    (path.starts_with("src/") && (path.ends_with(".rs") || path.ends_with(".toml")))
}

async fn test_exercise(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<CargoResult>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    
    match run_cargo_command("test", &exercise_path, vec!["--", "--nocapture"]).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Error running tests for {}/{}: {}", chapter, exercise, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn run_exercise(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<CargoResult>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    
    match run_cargo_command("run", &exercise_path, vec![]).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Error running exercise {}/{}: {}", chapter, exercise, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn check_exercise(
    AxumPath((chapter, exercise)): AxumPath<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<CargoResult>, StatusCode> {
    let exercise_path = state.exercises_path.join(&chapter).join(&exercise);
    
    match run_cargo_command("clippy", &exercise_path, vec!["--", "-W", "clippy::all"]).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Error running clippy for {}/{}: {}", chapter, exercise, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_progress(State(state): State<AppState>) -> Result<Json<ProgressData>, StatusCode> {
    match ensure_progress_file(&state.progress_path, &state.exercises_path).await {
        Ok(progress) => Ok(Json(progress)),
        Err(e) => {
            error!("Error loading progress: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn complete_exercise(
    State(state): State<AppState>,
    Json(request): Json<CompleteExerciseRequest>,
) -> Result<Json<ApiResponse<ProgressData>>, StatusCode> {
    match update_exercise_completion(&state.progress_path, &state.exercises_path, &request).await {
        Ok(progress) => Ok(Json(ApiResponse::success_with_extra(
            progress,
            serde_json::json!({"message": "Exercise completed successfully"})
        ))),
        Err(e) => {
            error!("Error updating progress: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn track_hint_usage(
    State(state): State<AppState>,
    Json(request): Json<HintRequest>,
) -> Result<Json<ApiResponse<ProgressData>>, StatusCode> {
    match update_hint_usage(&state.progress_path, &state.exercises_path, &request).await {
        Ok(progress) => Ok(Json(ApiResponse::success(progress))),
        Err(e) => {
            error!("Error tracking hint usage: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn track_exercise_view(
    State(state): State<AppState>,
    Json(request): Json<ViewRequest>,
) -> Result<Json<ApiResponse<ProgressData>>, StatusCode> {
    match update_exercise_view(&state.progress_path, &state.exercises_path, &request).await {
        Ok(progress) => Ok(Json(ApiResponse::success(progress))),
        Err(e) => {
            error!("Error tracking exercise view: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn fetch_book_content(url: &str) -> anyhow::Result<(String, String)> {
    // Fetch the HTML content
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Rust-Tour/1.0")
        .send()
        .await?;
    
    let html_content = response.text().await?;
    let document = ScraperHtml::parse_document(&html_content);
    
    // Debug: Log some HTML structure to understand the page layout
    debug!("HTML title: {:?}", document.select(&Selector::parse("title").unwrap()).next().map(|el| el.text().collect::<String>()));
    debug!("Found body: {}", document.select(&Selector::parse("body").unwrap()).next().is_some());
    
    // Extract the main content - try multiple selectors for mdBook structure
    let selectors = [
        "main",
        ".content", 
        "#content",
        ".page-content",
        ".book-content", 
        ".chapter",
        "#page-wrapper",
        ".page",
        "section",
        "article",
        "[role='main']",
        ".light"  // mdBook often uses this class for content
    ];
    
    let mut content_element = None;
    for selector_str in &selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                content_element = Some(element);
                break;
            }
        }
    }
    
    let content_element = content_element.unwrap_or_else(|| {
        // Fallback: use body if no specific content container found
        warn!("No specific content container found, using body as fallback");
        document.select(&Selector::parse("body").unwrap()).next()
            .expect("Every HTML document should have a body")
    });
    
    // Extract title
    let title_selector = Selector::parse("h1").unwrap();
    let title = content_element
        .select(&title_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_else(|| "Rust Book Chapter".to_string());
    
    // Get the HTML content and clean it up
    let mut content_html = content_element.html();
    
    // Clean up common navigation and UI elements
    let cleanups = [
        (r"<nav[^>]*>.*?</nav>", ""),
        (r"<header[^>]*>.*?</header>", ""),
        (r"<footer[^>]*>.*?</footer>", ""),
        (r"<aside[^>]*>.*?</aside>", ""),
        (r#"<div[^>]*class="[^"]*nav[^"]*"[^>]*>.*?</div>"#, ""),
        (r#"<div[^>]*class="[^"]*sidebar[^"]*"[^>]*>.*?</div>"#, ""),
        (r#"<div[^>]*class="[^"]*menu[^"]*"[^>]*>.*?</div>"#, ""),
        (r#"<button[^>]*>.*?</button>"#, ""),  // Remove navigation buttons
        (r#"<script[^>]*>.*?</script>"#, ""),  // Remove scripts
        (r#"<style[^>]*>.*?</style>"#, ""),    // Remove inline styles
    ];
    
    for (pattern, replacement) in cleanups {
        if let Ok(regex) = Regex::new(pattern) {
            content_html = regex.replace_all(&content_html, replacement).to_string();
        }
    }
    
    // Convert relative links to absolute URLs
    content_html = content_html.replace("href=\"/", "href=\"https://doc.rust-lang.org/");
    content_html = content_html.replace("src=\"/", "src=\"https://doc.rust-lang.org/");
    content_html = content_html.replace("href=\"ch", "href=\"https://doc.rust-lang.org/book/ch");
    
    // Remove empty paragraphs and extra whitespace
    content_html = content_html.replace("<p></p>", "");
    content_html = content_html.replace("<p> </p>", "");
    
    // Clean up excessive whitespace
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    content_html = whitespace_regex.replace_all(&content_html, " ").to_string();
    
    Ok((content_html, title))
}

async fn get_book_chapter(
    AxumPath(chapter): AxumPath<String>,
) -> Result<Json<BookContentResponse>, StatusCode> {
    let book_url = format!("https://doc.rust-lang.org/book/ch{}.html", chapter);
    
    match fetch_book_content(&book_url).await {
        Ok((content, title)) => {
            Ok(Json(BookContentResponse {
                url: book_url,
                chapter,
                content: Some(content),
                title: Some(title),
                error: None,
            }))
        }
        Err(e) => {
            warn!("Failed to fetch book content for chapter {}: {}", chapter, e);
            // Fallback to URL-only response
            Ok(Json(BookContentResponse {
                url: book_url,
                chapter,
                content: None,
                title: None,
                error: Some(format!("Failed to fetch content: {}", e)),
            }))
        }
    }
}

async fn get_book_by_url(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<BookContentResponse>, StatusCode> {
    let url = params.get("url")
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    match fetch_book_content(url).await {
        Ok((content, title)) => {
            // Extract chapter identifier from URL for response
            let chapter = url.split('/').last()
                .and_then(|filename| filename.strip_suffix(".html"))
                .unwrap_or("unknown")
                .to_string();
            
            Ok(Json(BookContentResponse {
                url: url.clone(),
                chapter,
                content: Some(content),
                title: Some(title),
                error: None,
            }))
        }
        Err(e) => {
            warn!("Failed to fetch book content from URL {}: {}", url, e);
            // Fallback to URL-only response
            let chapter = url.split('/').last()
                .and_then(|filename| filename.strip_suffix(".html"))
                .unwrap_or("unknown")
                .to_string();
                
            Ok(Json(BookContentResponse {
                url: url.clone(),
                chapter,
                content: None,
                title: None,
                error: Some(format!("Failed to fetch content: {}", e)),
            }))
        }
    }
}

// Static file handlers
#[cfg(feature = "embed-assets")]
async fn serve_static_files(uri: axum::http::Uri) -> Result<Response, StatusCode> {
    let path = uri.path().trim_start_matches('/');
    
    // Try to serve the requested file
    if let Some(content) = Assets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return Ok((
            [(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap())],
            content.data,
        ).into_response());
    }
    
    // Fallback to index.html for client-side routing
    if let Some(content) = Assets::get("index.html") {
        return Ok((
            [(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))],
            content.data,
        ).into_response());
    }
    
    Err(StatusCode::NOT_FOUND)
}

#[cfg(not(feature = "embed-assets"))]
async fn serve_static_files(uri: axum::http::Uri) -> Result<Response, StatusCode> {
    use tokio::fs;
    use std::path::Path;
    
    let path = uri.path().trim_start_matches('/');
    let web_dist = Path::new("web/dist");
    
    // Security check: prevent directory traversal
    if path.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let file_path = if path.is_empty() { 
        web_dist.join("index.html") 
    } else { 
        web_dist.join(path) 
    };
    
    // Try to serve the requested file
    if file_path.exists() && file_path.is_file() {
        match fs::read(&file_path).await {
            Ok(content) => {
                let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
                return Ok((
                    [(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap())],
                    content,
                ).into_response());
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    
    // Fallback to index.html for client-side routing
    let index_path = web_dist.join("index.html");
    if index_path.exists() {
        match fs::read(&index_path).await {
            Ok(content) => {
                return Ok((
                    [(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))],
                    content,
                ).into_response());
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    
    Err(StatusCode::NOT_FOUND)
}



// Helper functions
async fn scan_exercises(exercises_path: &std::path::Path) -> anyhow::Result<Vec<ExerciseWithPath>> {
    let mut exercises = Vec::new();
    
    if !exercises_path.exists() {
        return Ok(exercises);
    }
    
    for chapter_entry in WalkDir::new(exercises_path).max_depth(1) {
        let chapter_entry = chapter_entry?;
        if !chapter_entry.file_type().is_dir() {
            continue;
        }
        
        let chapter_name = chapter_entry.file_name().to_string_lossy();
        if !chapter_name.starts_with("ch") {
            continue;
        }
        
        for exercise_entry in WalkDir::new(chapter_entry.path()).max_depth(1) {
            let exercise_entry = exercise_entry?;
            if !exercise_entry.file_type().is_dir() {
                continue;
            }
            
            let exercise_name = exercise_entry.file_name().to_string_lossy();
            if !exercise_name.starts_with("ex") {
                continue;
            }
            
            let metadata_path = exercise_entry.path().join("metadata.json");
            if metadata_path.exists() {
                match fs::read_to_string(&metadata_path).await {
                    Ok(content) => {
                        match serde_json::from_str::<ExerciseMetadata>(&content) {
                            Ok(metadata) => {
                                exercises.push(ExerciseWithPath {
                                    metadata,
                                    path: format!("{}/{}", chapter_name, exercise_name),
                                });
                            }
                            Err(e) => {
                                warn!("Error parsing metadata for {}/{}: {}", chapter_name, exercise_name, e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error reading metadata for {}/{}: {}", chapter_name, exercise_name, e);
                    }
                }
            }
        }
    }
    
    // Sort by chapter and exercise number
    exercises.sort_by(|a, b| {
        match a.metadata.chapter.cmp(&b.metadata.chapter) {
            std::cmp::Ordering::Equal => a.metadata.exercise_number.cmp(&b.metadata.exercise_number),
            other => other,
        }
    });
    
    Ok(exercises)
}

async fn load_exercise_details(
    exercise_path: &std::path::Path,
    path: &str,
) -> anyhow::Result<ExerciseDetails> {
    info!("Loading exercise details for: {}", path);
    info!("Exercise path: {}", exercise_path.display());
    
    // Load metadata
    let metadata_path = exercise_path.join("metadata.json");
    info!("Attempting to read metadata: {}", metadata_path.display());
    let metadata_content = fs::read_to_string(&metadata_path).await
        .map_err(|e| anyhow::anyhow!("Failed to read metadata.json at {}: {}", metadata_path.display(), e))?;
    let metadata: ExerciseMetadata = serde_json::from_str(&metadata_content)?;
    
    // Load main source file
    let main_path = exercise_path.join("src").join("main.rs");
    info!("Attempting to read main.rs: {}", main_path.display());
    let main_content = fs::read_to_string(&main_path).await
        .map_err(|e| anyhow::anyhow!("Failed to read src/main.rs at {}: {}", main_path.display(), e))?;
    
    // Load README
    let readme_path = exercise_path.join("README.md");
    info!("Attempting to read README: {}", readme_path.display());
    let readme = fs::read_to_string(&readme_path).await
        .map_err(|e| anyhow::anyhow!("Failed to read README.md at {}: {}", readme_path.display(), e))?;
    
    // Load hints if available
    let hints_path = exercise_path.join("hints.md");
    let hints = if hints_path.exists() {
        fs::read_to_string(&hints_path).await.unwrap_or_default()
    } else {
        String::new()
    };
    
    // Load all files for editing
    let mut files = Vec::new();
    
    // Add Cargo.toml
    let cargo_path = exercise_path.join("Cargo.toml");
    if cargo_path.exists() {
        if let Ok(content) = fs::read_to_string(&cargo_path).await {
            files.push(FileContent {
                path: "Cargo.toml".to_string(),
                content,
                language: "toml".to_string(),
                editable: true,
            });
        }
    }
    
    // Add main.rs (already loaded, but add to files array)
    files.push(FileContent {
        path: "src/main.rs".to_string(),
        content: main_content.clone(),
        language: "rust".to_string(),
        editable: true,
    });
    
    // Check for lib.rs
    let lib_path = exercise_path.join("src").join("lib.rs");
    if lib_path.exists() {
        if let Ok(content) = fs::read_to_string(&lib_path).await {
            files.push(FileContent {
                path: "src/lib.rs".to_string(),
                content,
                language: "rust".to_string(),
                editable: true,
            });
        }
    }
    
    // Scan for any other .rs files in src/
    let src_dir = exercise_path.join("src");
    if let Ok(mut entries) = fs::read_dir(&src_dir).await {
        while let Some(entry) = entries.next_entry().await.ok().flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".rs") && file_name != "main.rs" && file_name != "lib.rs" {
                if let Ok(content) = fs::read_to_string(entry.path()).await {
                    files.push(FileContent {
                        path: format!("src/{}", file_name),
                        content,
                        language: "rust".to_string(),
                        editable: true,
                    });
                }
            }
        }
    }
    
    // Sort files for consistent ordering
    files.sort_by(|a, b| a.path.cmp(&b.path));
    
    Ok(ExerciseDetails {
        metadata,
        main_content,
        readme,
        hints,
        path: path.to_string(),
        files,
    })
}

async fn load_exercise_title(exercise_path: &std::path::Path) -> anyhow::Result<String> {
    let metadata_path = exercise_path.join("metadata.json");
    let metadata_content = fs::read_to_string(&metadata_path).await?;
    let metadata: ExerciseMetadata = serde_json::from_str(&metadata_content)?;
    Ok(metadata.title)
}

async fn run_cargo_command(
    command: &str,
    cwd: &std::path::Path,
    args: Vec<&str>,
) -> anyhow::Result<CargoResult> {
    let mut cmd = Command::new("cargo");
    cmd.arg(command)
        .args(&args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    let output = timeout(Duration::from_secs(60), cmd.output()).await??;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined_output = format!("{}{}", stdout, stderr);
    
    Ok(CargoResult {
        success: output.status.success(),
        code: output.status.code(),
        stdout,
        stderr,
        output: combined_output,
    })
}

async fn discover_chapters(exercises_path: &std::path::Path) -> anyhow::Result<(HashMap<u32, ChapterInfo>, u32)> {
    let mut chapters = HashMap::new();
    let mut total_exercises = 0;
    
    if !exercises_path.exists() {
        return Ok((chapters, 0));
    }
    
    for chapter_entry in WalkDir::new(exercises_path).max_depth(1) {
        let chapter_entry = chapter_entry?;
        if !chapter_entry.file_type().is_dir() {
            continue;
        }
        
        let chapter_name = chapter_entry.file_name().to_string_lossy();
        if !chapter_name.starts_with("ch") {
            continue;
        }
        
        // Extract chapter number
        let chapter_number = chapter_name
            .strip_prefix("ch")
            .and_then(|s| s.split('_').next())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_else(|| {
                warn!("Could not parse chapter number from: {}", chapter_name);
                1
            });
        
        // Count exercises in this chapter and get chapter title
        let mut exercise_count = 0;
        let mut chapter_title = format!("Chapter {}", chapter_number);
        
        for exercise_entry in WalkDir::new(chapter_entry.path()).max_depth(1) {
            let exercise_entry = exercise_entry?;
            if !exercise_entry.file_type().is_dir() {
                continue;
            }
            
            let exercise_name = exercise_entry.file_name().to_string_lossy();
            if exercise_name.starts_with("ex") {
                exercise_count += 1;
                
                // Try to get chapter title from first exercise metadata
                if exercise_count == 1 {
                    let metadata_path = exercise_entry.path().join("metadata.json");
                    if let Ok(metadata_content) = fs::read_to_string(&metadata_path).await {
                        if let Ok(_metadata) = serde_json::from_str::<ExerciseMetadata>(&metadata_content) {
                            // Extract chapter title from chapter name, clean it up
                            chapter_title = chapter_name
                                .strip_prefix("ch")
                                .and_then(|s| s.split_once('_'))
                                .map(|(_, title)| title.replace('_', " ").to_title_case())
                                .unwrap_or_else(|| format!("Chapter {}", chapter_number));
                        }
                    }
                }
            }
        }
        
        if exercise_count > 0 {
            total_exercises += exercise_count;
            
            let chapter_info = ChapterInfo {
                chapter_number,
                title: chapter_title,
                exercises_completed: 0,
                total_exercises: exercise_count,
                completion_percentage: 0.0,
                time_spent_minutes: 0,
            };
            
            chapters.insert(chapter_number, chapter_info);
        }
    }
    
    Ok((chapters, total_exercises))
}

async fn count_total_exercises(exercises_path: &std::path::Path) -> anyhow::Result<u32> {
    let (_, total) = discover_chapters(exercises_path).await?;
    Ok(if total > 0 { total } else { 50 })
}

async fn ensure_progress_file(
    progress_path: &std::path::Path,
    exercises_path: &std::path::Path,
) -> anyhow::Result<ProgressData> {
    // Ensure the progress directory exists
    if let Some(parent) = progress_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    
    let (discovered_chapters, total_exercises) = discover_chapters(exercises_path).await?;
    
    if !progress_path.exists() {
        info!("Creating new progress file: {:?}", progress_path);
        info!("Detected {} total exercises across {} chapters", total_exercises, discovered_chapters.len());
        
        // Convert discovered chapters to JSON Value
        let chapters_json = serde_json::to_value(&discovered_chapters)?;
        
        let default_progress = ProgressData {
            user_id: "default".to_string(),
            created_at: Utc::now().to_rfc3339(),
            overall_progress: 0.0,
            chapters_completed: 0,
            exercises_completed: 0,
            total_exercises,
            current_streak: 0,
            longest_streak: 0,
            total_time_minutes: 0,
            chapters: chapters_json,
            exercise_history: Vec::new(),
            achievements: Vec::new(),
            session_stats: SessionStats {
                exercises_viewed: 0,
                exercises_completed: 0,
                hints_used: 0,
                time_spent: 0,
            },
        };
        
        let content = serde_json::to_string_pretty(&default_progress)?;
        fs::write(progress_path, content).await?;
        info!("Progress file created successfully");
        return Ok(default_progress);
    }
    
    // File exists, load and validate it
    let content = fs::read_to_string(progress_path).await?;
    let mut progress: ProgressData = serde_json::from_str(&content)?;
    
    // Ensure all required properties exist (for backwards compatibility)
    if progress.session_stats.exercises_viewed == 0 && progress.session_stats.exercises_completed == 0 && progress.session_stats.hints_used == 0 && progress.session_stats.time_spent == 0 {
        // Initialize default session stats if they're all zero (likely missing)
        progress.session_stats = SessionStats {
            exercises_viewed: 0,
            exercises_completed: 0,
            hints_used: 0,
            time_spent: 0,
        };
    }
    
    // Update total exercises count and chapters if needed
    let mut should_save = false;
    
    if progress.total_exercises == 0 || progress.total_exercises == 200 || progress.total_exercises != total_exercises {
        progress.total_exercises = total_exercises;
        should_save = true;
        info!("Updated total exercises count to {}", total_exercises);
    }
    
    // Update chapters if empty or if we have new chapters discovered
    if progress.chapters.as_object().map_or(true, |obj| obj.is_empty()) || discovered_chapters.len() > 0 {
        let chapters_json = serde_json::to_value(&discovered_chapters)?;
        progress.chapters = chapters_json;
        should_save = true;
        info!("Updated chapters structure with {} chapters", discovered_chapters.len());
    }
    
    if should_save {
        let content = serde_json::to_string_pretty(&progress)?;
        fs::write(progress_path, content).await?;
        info!("Progress file updated successfully");
    }
    
    Ok(progress)
}

async fn update_exercise_completion(
    progress_path: &std::path::Path,
    exercises_path: &std::path::Path,
    request: &CompleteExerciseRequest,
) -> anyhow::Result<ProgressData> {
    let mut progress = ensure_progress_file(progress_path, exercises_path).await?;
    
    // Check if already completed to avoid duplicates
    let existing_entry = progress.exercise_history.iter().find(|entry| entry.exercise_id == request.exercise_id);
    let is_already_completed = existing_entry.map_or(false, |entry| entry.completed_at.is_some());
    
    info!("Checking completion for {}: already completed: {}", request.exercise_id, is_already_completed);
    
    if is_already_completed {
        info!("Exercise {} already completed", request.exercise_id);
        return Ok(progress);
    }
    
    // Update progress
    progress.exercises_completed += 1;
    progress.session_stats.exercises_completed += 1;
    progress.total_time_minutes += request.time_taken_minutes.unwrap_or(0);
    progress.overall_progress = progress.exercises_completed as f64 / progress.total_exercises as f64;
    
    // Update or add to exercise history
    if let Some(entry) = progress.exercise_history.iter_mut().find(|entry| entry.exercise_id == request.exercise_id) {
        entry.completed_at = Some(Utc::now().to_rfc3339());
        entry.time_taken_minutes = request.time_taken_minutes;
        entry.status = "completed".to_string();
        entry.session_id = Some(format!("session_{}", Utc::now().timestamp_millis()));
    } else {
        progress.exercise_history.push(ExerciseHistoryEntry {
            exercise_id: request.exercise_id.clone(),
            viewed_at: None,
            completed_at: Some(Utc::now().to_rfc3339()),
            time_taken_minutes: request.time_taken_minutes,
            status: "completed".to_string(),
            session_id: Some(format!("session_{}", Utc::now().timestamp_millis())),
            hints_used: None,
        });
    }
    
    // Save updated progress
    let content = serde_json::to_string_pretty(&progress)?;
    fs::write(progress_path, content).await?;
    
    info!(
        "Exercise completed: {} in {} minutes",
        request.exercise_id,
        request.time_taken_minutes.unwrap_or(0)
    );
    info!(
        "Total exercises completed: {}/{}",
        progress.exercises_completed,
        progress.total_exercises
    );
    
    Ok(progress)
}

async fn update_hint_usage(
    progress_path: &std::path::Path,
    exercises_path: &std::path::Path,
    request: &HintRequest,
) -> anyhow::Result<ProgressData> {
    let mut progress = ensure_progress_file(progress_path, exercises_path).await?;
    
    // Update hint usage stats
    progress.session_stats.hints_used += 1;
    
    // Add to exercise history if not already tracked for this exercise
    if let Some(entry) = progress.exercise_history.iter_mut().find(|entry| entry.exercise_id == request.exercise_id) {
        if let Some(ref mut hints_used) = entry.hints_used {
            if !hints_used.contains(&request.hint_level) {
                hints_used.push(request.hint_level);
            }
        } else {
            entry.hints_used = Some(vec![request.hint_level]);
        }
    } else {
        progress.exercise_history.push(ExerciseHistoryEntry {
            exercise_id: request.exercise_id.clone(),
            viewed_at: Some(Utc::now().to_rfc3339()),
            completed_at: None,
            time_taken_minutes: None,
            status: "in_progress".to_string(),
            session_id: None,
            hints_used: Some(vec![request.hint_level]),
        });
    }
    
    // Save updated progress
    let content = serde_json::to_string_pretty(&progress)?;
    fs::write(progress_path, content).await?;
    
    info!("Hint used: {}, level {}", request.exercise_id, request.hint_level);
    Ok(progress)
}

async fn update_exercise_view(
    progress_path: &std::path::Path,
    exercises_path: &std::path::Path,
    request: &ViewRequest,
) -> anyhow::Result<ProgressData> {
    let mut progress = ensure_progress_file(progress_path, exercises_path).await?;
    
    // Update view stats
    progress.session_stats.exercises_viewed += 1;
    
    // Check if already viewed
    if !progress.exercise_history.iter().any(|entry| entry.exercise_id == request.exercise_id) {
        progress.exercise_history.push(ExerciseHistoryEntry {
            exercise_id: request.exercise_id.clone(),
            viewed_at: Some(Utc::now().to_rfc3339()),
            completed_at: None,
            time_taken_minutes: None,
            status: "viewed".to_string(),
            session_id: None,
            hints_used: None,
        });
    }
    
    // Save updated progress
    let content = serde_json::to_string_pretty(&progress)?;
    fs::write(progress_path, content).await?;
    
    info!("Exercise viewed: {}", request.exercise_id);
    Ok(progress)
}

async fn initialize_progress_system(state: &AppState) -> anyhow::Result<()> {
    match ensure_progress_file(&state.progress_path, &state.exercises_path).await {
        Ok(_) => {
            info!("📊 Progress system initialized");
            Ok(())
        }
        Err(e) => {
            error!("Failed to initialize progress system: {}", e);
            Err(e)
        }
    }
}

async fn setup_file_watcher(state: AppState) -> anyhow::Result<()> {
    let exercises_path = state.exercises_path.clone();
    let broadcast_tx = state.broadcast_tx.clone();
    
    tokio::spawn(async move {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        
        let mut watcher = match RecommendedWatcher::new(
            move |res| {
                if let Err(e) = tx.blocking_send(res) {
                    error!("Failed to send file watcher event: {}", e);
                }
            },
            notify::Config::default(),
        ) {
            Ok(watcher) => watcher,
            Err(e) => {
                error!("Failed to create file watcher: {}", e);
                return;
            }
        };
        
        if let Err(e) = watcher.watch(&exercises_path, RecursiveMode::Recursive) {
            error!("Failed to watch exercises directory: {}", e);
            return;
        }
        
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    for path in event.paths {
                        if let Ok(relative_path) = path.strip_prefix(&exercises_path) {
                            let path_parts: Vec<_> = relative_path.components().collect();
                            if path_parts.len() >= 2 {
                                let chapter_dir = path_parts[0].as_os_str().to_string_lossy();
                                let exercise_dir = path_parts[1].as_os_str().to_string_lossy();
                                
                                // Try to load exercise metadata to get title
                                let metadata_path = exercises_path.join(&*chapter_dir).join(&*exercise_dir).join("metadata.json");
                                let exercise_name = if metadata_path.exists() {
                                    match load_exercise_title(&exercises_path.join(&*chapter_dir).join(&*exercise_dir)).await {
                                        Ok(title) => title,
                                        Err(_) => exercise_dir.replace('_', " ").replacen("ex", "", 1),
                                    }
                                } else {
                                    exercise_dir.replace('_', " ").replacen("ex", "", 1)
                                };
                                
                                let broadcast_msg = BroadcastMessage {
                                    msg_type: "file_changed".to_string(),
                                    data: serde_json::json!({
                                        "exercise": exercise_name,
                                        "file": relative_path.to_string_lossy()
                                    }),
                                };
                                
                                let _ = broadcast_tx.send(broadcast_msg);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("File watcher error: {}", e);
                }
            }
        }
    });
    
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("\n\n🛑 Shutdown signal received");
            println!("   Saving progress and closing connections...");
            info!("Shutting down gracefully...");
        },
        _ = terminate => {
            println!("\n\n🛑 Shutdown signal received");
            println!("   Saving progress and closing connections...");
            info!("Shutting down gracefully...");
        },
    }
}

#[cfg(feature = "download-exercises")]
async fn ensure_exercises_available(exercises_path: PathBuf) -> anyhow::Result<PathBuf> {
    println!("🔍 Checking exercises availability...");
    println!("   Initial exercises_path: {}", exercises_path.display());
    
    // Check if exercises directory exists and has content
    if exercises_path.exists() && has_exercises(&exercises_path).await? {
        println!("   ✓ Found exercises at initial path");
        return Ok(exercises_path);
    }

    // Check if user has a config file with a custom exercises path
    if let Ok(base_path) = get_config_exercises_path().await {
        println!("   Config base_path: {}", base_path.display());
        let exercises_subdir = base_path.join("exercises");
        println!("   Checking exercises_subdir: {}", exercises_subdir.display());
        if exercises_subdir.exists() && has_exercises(&exercises_subdir).await? {
            println!("   ✓ Found exercises at config path");
            return Ok(exercises_subdir);
        }
    }

    // No exercises found, prompt user to download
    println!("\n🦀 Welcome to Rust Tour!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("This appears to be your first time running Rust Tour!");
    println!("We need to download the exercise files to get started.");
    println!();
    println!("📚 The exercises include:");
    println!("   • 200+ hands-on coding challenges");
    println!("   • Test-driven exercises aligned with The Rust Book");
    println!("   • Progressive difficulty from beginner to advanced");
    println!("   • Complete with hints and solutions");
    println!();
    println!("📦 Download size: ~5MB");
    println!("⏱️  Estimated time: 10-30 seconds");
    println!();

    let should_download = Confirm::new()
        .with_prompt("Download exercises from GitHub?")
        .default(true)
        .interact()?;

    if !should_download {
        println!("\n❌ Rust Tour requires exercises to run.");
        println!("   You can:");
        println!("   • Run this command again and choose to download");
        println!("   • Clone the repository manually from https://github.com/ghanithan/rust-tour");
        println!("   • Use --exercises-path to specify a custom location");
        anyhow::bail!("Exiting without exercises.");
    }

    // Get download directory from user
    println!("\n📍 Choose where to store your exercises and progress:");
    println!("   This will create a 'rust-tour-exercises' folder at your chosen location.");
    println!("   Your progress will be saved here between sessions.");
    println!();
    
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let default_path = home_dir.join("rust-tour-exercises");
    
    // Ask user how they want to select the directory
    let selection_method = Select::new()
        .with_prompt("How would you like to select the directory?")
        .items(&vec!["Browse for directory", "Enter path manually"])
        .default(0)
        .interact()?;
    
    let download_path = match selection_method {
        0 => {
            // Browse for directory
            println!("\n📁 Navigate to your desired parent directory:");
            println!("   The 'rust-tour-exercises' folder will be created inside your selection.");
            
            let start_path = home_dir.parent().unwrap_or(&home_dir).to_path_buf();
            match browse_for_directory(start_path).await? {
                Some(selected_path) => selected_path.join("rust-tour-exercises"),
                None => {
                    println!("\n❌ Directory selection cancelled.");
                    anyhow::bail!("Download cancelled. Please run again to select a directory.");
                }
            }
        }
        1 => {
            // Manual entry
            let path_input: String = Input::new()
                .with_prompt("Location")
                .default(default_path.to_string_lossy().to_string())
                .validate_with(|input: &String| {
                    let path = PathBuf::from(input);
                    if let Some(parent) = path.parent() {
                        if !parent.exists() {
                            return Err(format!("Parent directory '{}' does not exist", parent.display()));
                        }
                    }
                    Ok(())
                })
                .interact_text()?;
            
            PathBuf::from(path_input)
        }
        _ => unreachable!(),
    };

    // Confirm the full path
    println!("\n📂 Full path: {}", download_path.display());
    let confirm_path = Confirm::new()
        .with_prompt("Is this correct?")
        .default(true)
        .interact()?;
    
    if !confirm_path {
        anyhow::bail!("Download cancelled. Please run again with your preferred location.");
    }

    // Download exercises
    println!("\n🌐 Connecting to GitHub...");
    println!("📦 Downloading exercises from https://github.com/ghanithan/rust-tour...");
    download_exercises(&download_path).await?;
    
    println!("🔍 Download completed to: {}", download_path.display());
    println!("   Expected exercises at: {}", download_path.join("exercises").display());

    // Save config for future use (save the base directory)
    save_config_exercises_path(&download_path).await?;
    println!("💾 Saved config with base path: {}", download_path.display());

    println!("\n✅ Success! Rust Tour is ready to use.");
    println!("\n📂 Your Rust Tour directory structure:");
    println!("   {}/", download_path.display());
    println!("   ├── exercises/     # All learning exercises");
    println!("   └── progress/      # Your progress tracking");
    println!("\n🎯 Starting Rust Tour server...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Return the exercises subdirectory path
    Ok(download_path.join("exercises"))
}

#[cfg(feature = "download-exercises")]
async fn has_exercises(path: &std::path::Path) -> anyhow::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    // Check if there's at least one chapter directory with exercises
    let mut entries = tokio::fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let name = entry.file_name();
            if let Some(name_str) = name.to_str() {
                if name_str.starts_with("ch") && name_str.contains("_") {
                    // Found a chapter directory, check if it has exercises
                    let chapter_path = entry.path();
                    let mut chapter_entries = tokio::fs::read_dir(&chapter_path).await?;
                    while let Some(ex_entry) = chapter_entries.next_entry().await? {
                        if ex_entry.file_type().await?.is_dir() {
                            let ex_name = ex_entry.file_name();
                            if let Some(ex_name_str) = ex_name.to_str() {
                                if ex_name_str.starts_with("ex") {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(false)
}

#[cfg(feature = "download-exercises")]
async fn download_exercises(target_path: &std::path::Path) -> anyhow::Result<()> {

    // Create target directory
    tokio::fs::create_dir_all(target_path).await?;

    // Repository URL
    let repo_url = "https://github.com/ghanithan/rust-tour.git";
    
    // Clone the repository to a temporary directory
    let temp_dir = TempDir::new()?;
    let _repo = Repository::clone(repo_url, temp_dir.path())?;
    
    println!("   ✓ Repository cloned");
    println!("📂 Extracting exercise files...");
    
    // Copy exercises directory from temp to target/exercises
    let source_exercises = temp_dir.path().join("exercises");
    let target_exercises = target_path.join("exercises");
    
    if source_exercises.exists() {
        copy_dir_recursive(source_exercises, target_exercises.clone()).await?;
        println!("   ✓ Exercise files extracted");
        
        // Verify critical exercise files exist
        verify_exercises_integrity(&target_exercises).await?;
        println!("   ✓ Exercise integrity verified");
    } else {
        anyhow::bail!("No exercises directory found in repository");
    }

    Ok(())
}

#[cfg(feature = "download-exercises")]
async fn verify_exercises_integrity(exercises_path: &std::path::Path) -> anyhow::Result<()> {
    println!("   🔍 Verifying exercise integrity...");
    let mut missing_files = Vec::new();
    let mut total_exercises = 0;
    let mut valid_exercises = 0;
    
    // Check some known critical exercises
    let critical_exercises = [
        "ch01_getting_started/ex01_hello_world",
        "ch02_guessing_game/ex01_user_input", 
        "ch03_common_concepts/ex01_variables",
        "ch04_understanding_ownership/ex01_ownership_basics",
        "ch05_using_structs/ex01_struct_definition",
        "ch05_using_structs/ex06_ownership_structs", // The one that was failing
    ];
    
    for exercise_path in critical_exercises {
        total_exercises += 1;
        let exercise_dir = exercises_path.join(exercise_path);
        let src_main = exercise_dir.join("src/main.rs");
        let metadata = exercise_dir.join("metadata.json");
        
        if !exercise_dir.exists() {
            missing_files.push(format!("Missing exercise directory: {}", exercise_path));
            continue;
        }
        
        if !src_main.exists() {
            missing_files.push(format!("Missing src/main.rs: {}", exercise_path));
            continue;
        }
        
        if !metadata.exists() {
            missing_files.push(format!("Missing metadata.json: {}", exercise_path));
            continue;
        }
        
        valid_exercises += 1;
        println!("      ✓ {}", exercise_path);
    }
    
    println!("   📊 Verified {}/{} critical exercises", valid_exercises, total_exercises);
    
    if !missing_files.is_empty() {
        println!("   ⚠️  Found {} issues:", missing_files.len());
        for missing in &missing_files {
            println!("      ✗ {}", missing);
        }
        anyhow::bail!("Exercise integrity check failed: {} critical files missing", missing_files.len());
    }
    
    Ok(())
}

#[cfg(feature = "download-exercises")]
async fn copy_dir_recursive(src: std::path::PathBuf, dst: std::path::PathBuf) -> anyhow::Result<()> {
    println!("   📁 Creating directory: {}", dst.display());
    tokio::fs::create_dir_all(&dst).await?;
    
    let mut entries = tokio::fs::read_dir(&src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        let file_type = entry.file_type().await?;
        
        if file_type.is_dir() {
            println!("   📂 Copying directory: {} -> {}", src_path.display(), dst_path.display());
            Box::pin(copy_dir_recursive(src_path, dst_path)).await?;
        } else if file_type.is_file() {
            println!("   📄 Copying file: {} -> {}", src_path.display(), dst_path.display());
            match tokio::fs::copy(&src_path, &dst_path).await {
                Ok(bytes) => {
                    println!("      ✓ Copied {} bytes", bytes);
                },
                Err(e) => {
                    error!("      ✗ Failed to copy {}: {}", src_path.display(), e);
                    return Err(e.into());
                }
            }
        } else {
            println!("   ⚠️  Skipping non-regular file: {}", src_path.display());
        }
    }
    
    Ok(())
}

#[cfg(feature = "download-exercises")]
async fn get_config_exercises_path() -> anyhow::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|p| p.join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let config_file = config_dir.join("rust-tour").join("config.json");
    
    if config_file.exists() {
        let content = tokio::fs::read_to_string(&config_file).await?;
        let config: serde_json::Value = serde_json::from_str(&content)?;
        
        if let Some(path_str) = config.get("exercises_path").and_then(|v| v.as_str()) {
            return Ok(PathBuf::from(path_str));
        }
    }
    
    anyhow::bail!("No config found")
}

#[cfg(feature = "download-exercises")]
async fn save_config_exercises_path(exercises_path: &std::path::Path) -> anyhow::Result<()> {
    let config_dir = dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|p| p.join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let config_dir = config_dir.join("rust-tour");
    tokio::fs::create_dir_all(&config_dir).await?;
    
    let config_file = config_dir.join("config.json");
    let config = serde_json::json!({
        "exercises_path": exercises_path.to_string_lossy()
    });
    
    tokio::fs::write(&config_file, serde_json::to_string_pretty(&config)?).await?;
    Ok(())
}

#[cfg(feature = "download-exercises")]
async fn browse_for_directory(start_path: PathBuf) -> anyhow::Result<Option<PathBuf>> {
    let mut current_path = start_path;
    
    loop {
        // Read directory entries
        let mut entries = Vec::new();
        let mut dirs = Vec::new();
        
        match std::fs::read_dir(&current_path) {
            Ok(read_dir) => {
                for entry in read_dir {
                    if let Ok(entry) = entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                if let Some(name) = entry.file_name().to_str() {
                                    // Skip hidden directories
                                    if !name.starts_with('.') {
                                        dirs.push(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                dirs.sort();
            }
            Err(_) => {
                println!("⚠️  Cannot read directory: {}", current_path.display());
                return Ok(None);
            }
        }
        
        // Build menu items
        entries.push("[✓ Select This Directory]".to_string());
        if current_path.parent().is_some() {
            entries.push("[↑ Go Up]".to_string());
        }
        for dir in &dirs {
            entries.push(format!("📁 {}/", dir));
        }
        entries.push("[✗ Cancel]".to_string());
        
        // Show current path
        println!("\n📂 Current: {}", current_path.display());
        
        // Create selection dialog
        let selection = Select::new()
            .items(&entries)
            .default(0)
            .interact()?;
        
        match entries[selection].as_str() {
            "[✓ Select This Directory]" => {
                return Ok(Some(current_path));
            }
            "[↑ Go Up]" => {
                if let Some(parent) = current_path.parent() {
                    current_path = parent.to_path_buf();
                }
            }
            "[✗ Cancel]" => {
                return Ok(None);
            }
            dir_entry => {
                // Extract directory name (remove "📁 " prefix and "/" suffix)
                let dir_name = dir_entry.trim_start_matches("📁 ").trim_end_matches('/');
                current_path = current_path.join(dir_name);
            }
        }
    }
}