const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
const path = require('path');
const fs = require('fs-extra');
const { spawn } = require('child_process');
const WebSocket = require('ws');
const chokidar = require('chokidar');
const pty = require('node-pty');

const app = express();
const PORT = process.env.PORT || 3000;
const DEBUG_WEBSOCKET = process.env.DEBUG_WEBSOCKET === 'true' || false;

// Middleware
app.use(helmet({
  contentSecurityPolicy: false, // Disable for Monaco Editor
}));
app.use(cors());
app.use(morgan('combined'));
app.use(express.json({ limit: '50mb' }));
app.use(express.urlencoded({ extended: true }));

// Serve static files
app.use(express.static(path.join(__dirname, 'dist')));
app.use('/monaco', express.static(path.join(__dirname, 'node_modules/monaco-editor')));

// Create HTTP server
const server = require('http').createServer(app);

// WebSocket server for real-time updates
const wss = new WebSocket.Server({ noServer: true });

// Store active connections and terminal sessions
const connections = new Set();
const terminalSessions = new Map();

wss.on('connection', (ws, request) => {
  connections.add(ws);
  console.log('Client connected to WebSocket');
  
  ws.on('message', (message) => {
    try {
      const data = JSON.parse(message);
      if (DEBUG_WEBSOCKET) {
        console.log('Received WebSocket message:', data.type);
      }
      
      if (data.type === 'terminal') {
        if (DEBUG_WEBSOCKET) {
          console.log('Handling terminal message:', data.action);
        }
        handleTerminalMessage(ws, data);
      }
    } catch (error) {
      console.error('Error parsing WebSocket message:', error);
    }
  });
  
  ws.on('close', () => {
    connections.delete(ws);
    // Clean up terminal session if it exists
    for (const [sessionId, session] of terminalSessions) {
      if (session.ws === ws) {
        session.pty.kill();
        terminalSessions.delete(sessionId);
        break;
      }
    }
    console.log('Client disconnected from WebSocket');
  });
});

// Broadcast to all connected clients
function broadcast(data) {
  const message = JSON.stringify(data);
  connections.forEach((ws) => {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(message);
    }
  });
}

// Terminal message handler
function handleTerminalMessage(ws, data) {
  const { action, sessionId, input, cols, rows } = data;
  
  switch (action) {
    case 'create':
      createTerminalSession(ws, sessionId, cols, rows);
      break;
    case 'check':
      checkTerminalSession(ws, sessionId);
      break;
    case 'input':
      sendInputToTerminal(sessionId, input);
      break;
    case 'resize':
      resizeTerminal(sessionId, cols, rows);
      break;
    case 'destroy':
      destroyTerminalSession(sessionId);
      break;
  }
}

// Check if terminal session exists
function checkTerminalSession(ws, sessionId) {
  const session = terminalSessions.get(sessionId);
  if (session && session.pty && !session.pty.killed) {
    // Session exists and is active
    ws.send(JSON.stringify({
      type: 'terminal',
      action: 'exists',
      sessionId: sessionId
    }));
    // Update WebSocket reference
    session.ws = ws;
  } else {
    // Session doesn't exist or is dead
    ws.send(JSON.stringify({
      type: 'terminal',
      action: 'not_found',
      sessionId: sessionId
    }));
    // Clean up dead session
    if (session) {
      terminalSessions.delete(sessionId);
    }
  }
}

// Create a new terminal session
function createTerminalSession(ws, sessionId, cols = 80, rows = 24) {
  try {
    // Check if session already exists
    const existingSession = terminalSessions.get(sessionId);
    if (existingSession && existingSession.pty && !existingSession.pty.killed) {
      // Update WebSocket reference for existing session
      existingSession.ws = ws;
      ws.send(JSON.stringify({
        type: 'terminal',
        action: 'created',
        sessionId: sessionId
      }));
      return;
    }

    // Clean up any dead session with same ID
    if (existingSession) {
      terminalSessions.delete(sessionId);
    }

    // Determine working directory based on current exercise or default
    const cwd = path.join(__dirname, '../exercises');
    
    const ptyProcess = pty.spawn(process.platform === 'win32' ? 'powershell.exe' : 'bash', [], {
      name: 'xterm-color',
      cols: cols,
      rows: rows,
      cwd: cwd,
      env: process.env
    });

    ptyProcess.onData((data) => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
          type: 'terminal',
          action: 'output',
          sessionId: sessionId,
          data: data
        }));
      }
    });

    ptyProcess.onExit(() => {
      terminalSessions.delete(sessionId);
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
          type: 'terminal',
          action: 'exit',
          sessionId: sessionId
        }));
      }
    });

    terminalSessions.set(sessionId, {
      pty: ptyProcess,
      ws: ws
    });

    ws.send(JSON.stringify({
      type: 'terminal',
      action: 'created',
      sessionId: sessionId
    }));

    if (DEBUG_WEBSOCKET) {
      console.log(`Terminal session ${sessionId} created`);
    }
  } catch (error) {
    console.error('Error creating terminal session:', error);
    ws.send(JSON.stringify({
      type: 'terminal',
      action: 'error',
      sessionId: sessionId,
      message: 'Failed to create terminal session'
    }));
  }
}

// Send input to terminal
function sendInputToTerminal(sessionId, input) {
  const session = terminalSessions.get(sessionId);
  if (session) {
    session.pty.write(input);
  }
}

// Resize terminal
function resizeTerminal(sessionId, cols, rows) {
  const session = terminalSessions.get(sessionId);
  if (session) {
    session.pty.resize(cols, rows);
  }
}

// Destroy terminal session
function destroyTerminalSession(sessionId) {
  const session = terminalSessions.get(sessionId);
  if (session) {
    session.pty.kill();
    terminalSessions.delete(sessionId);
    if (DEBUG_WEBSOCKET) {
      console.log(`Terminal session ${sessionId} destroyed`);
    }
  }
}

// Helper function to run cargo commands
function runCargoCommand(command, cwd, args = []) {
  return new Promise((resolve, reject) => {
    const cargo = spawn('cargo', [command, ...args], { 
      cwd,
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    cargo.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    cargo.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    cargo.on('close', (code) => {
      resolve({
        success: code === 0,
        code,
        stdout,
        stderr,
        output: stdout + stderr
      });
    });

    cargo.on('error', (error) => {
      reject(error);
    });
  });
}

// API Routes

// Get all exercises
app.get('/api/exercises', async (req, res) => {
  try {
    const exercisesDir = path.join(__dirname, '../exercises');
    const exercises = [];
    
    // Scan for exercise directories
    const chapters = await fs.readdir(exercisesDir);
    
    for (const chapter of chapters) {
      const chapterPath = path.join(exercisesDir, chapter);
      const stat = await fs.stat(chapterPath);
      
      if (stat.isDirectory()) {
        const exerciseDirs = await fs.readdir(chapterPath);
        
        for (const exerciseDir of exerciseDirs) {
          const exercisePath = path.join(chapterPath, exerciseDir);
          const metadataPath = path.join(exercisePath, 'metadata.json');
          
          if (await fs.pathExists(metadataPath)) {
            const metadata = await fs.readJson(metadataPath);
            exercises.push({
              ...metadata,
              path: `${chapter}/${exerciseDir}`
            });
          }
        }
      }
    }
    
    // Sort by chapter and exercise number
    exercises.sort((a, b) => {
      if (a.chapter !== b.chapter) {
        return a.chapter - b.chapter;
      }
      return a.exercise_number - b.exercise_number;
    });
    
    res.json(exercises);
  } catch (error) {
    console.error('Error loading exercises:', error);
    res.status(500).json({ error: 'Failed to load exercises' });
  }
});

// Get specific exercise
app.get('/api/exercises/:chapter/:exercise', async (req, res) => {
  try {
    const { chapter, exercise } = req.params;
    const exercisePath = path.join(__dirname, '../exercises', chapter, exercise);
    
    // Load metadata
    const metadata = await fs.readJson(path.join(exercisePath, 'metadata.json'));
    
    // Load main source file
    const mainPath = path.join(exercisePath, 'src/main.rs');
    const mainContent = await fs.readFile(mainPath, 'utf8');
    
    // Load README
    const readmePath = path.join(exercisePath, 'README.md');
    const readme = await fs.readFile(readmePath, 'utf8');
    
    // Load hints if available
    let hints = '';
    const hintsPath = path.join(exercisePath, 'hints.md');
    if (await fs.pathExists(hintsPath)) {
      hints = await fs.readFile(hintsPath, 'utf8');
    }
    
    res.json({
      metadata,
      mainContent,
      readme,
      hints,
      path: `${chapter}/${exercise}`
    });
  } catch (error) {
    console.error('Error loading exercise:', error);
    res.status(404).json({ error: 'Exercise not found' });
  }
});

// Save exercise code
app.put('/api/exercises/:chapter/:exercise/code', async (req, res) => {
  try {
    const { chapter, exercise } = req.params;
    const { code } = req.body;
    
    const exercisePath = path.join(__dirname, '../exercises', chapter, exercise);
    const mainPath = path.join(exercisePath, 'src/main.rs');
    
    await fs.writeFile(mainPath, code, 'utf8');
    
    // Broadcast file change to connected clients with exercise name
    let exerciseName = `${chapter}/${exercise}`;
    try {
      const metadataPath = path.join(exercisePath, 'metadata.json');
      if (await fs.pathExists(metadataPath)) {
        const metadata = await fs.readJson(metadataPath);
        exerciseName = metadata.title || exerciseName;
      }
    } catch (error) {
      // If metadata loading fails, use the path as fallback
    }
    
    broadcast({
      type: 'file_updated',
      exercise: exerciseName,
      file: 'src/main.rs'
    });
    
    res.json({ success: true });
  } catch (error) {
    console.error('Error saving code:', error);
    res.status(500).json({ error: 'Failed to save code' });
  }
});

// Run tests for an exercise
app.post('/api/exercises/:chapter/:exercise/test', async (req, res) => {
  try {
    const { chapter, exercise } = req.params;
    const exercisePath = path.join(__dirname, '../exercises', chapter, exercise);
    
    // Run cargo test
    const result = await runCargoCommand('test', exercisePath, ['--', '--nocapture']);
    
    res.json({
      success: result.success,
      output: result.output,
      stdout: result.stdout,
      stderr: result.stderr
    });
  } catch (error) {
    console.error('Error running tests:', error);
    res.status(500).json({ error: 'Failed to run tests' });
  }
});

// Run the exercise
app.post('/api/exercises/:chapter/:exercise/run', async (req, res) => {
  try {
    const { chapter, exercise } = req.params;
    const exercisePath = path.join(__dirname, '../exercises', chapter, exercise);
    
    // Run cargo run
    const result = await runCargoCommand('run', exercisePath);
    
    res.json({
      success: result.success,
      output: result.output,
      stdout: result.stdout,
      stderr: result.stderr
    });
  } catch (error) {
    console.error('Error running exercise:', error);
    res.status(500).json({ error: 'Failed to run exercise' });
  }
});

// Check code with clippy
app.post('/api/exercises/:chapter/:exercise/check', async (req, res) => {
  try {
    const { chapter, exercise } = req.params;
    const exercisePath = path.join(__dirname, '../exercises', chapter, exercise);
    
    // Run cargo clippy
    const result = await runCargoCommand('clippy', exercisePath, ['--', '-W', 'clippy::all']);
    
    res.json({
      success: result.success,
      output: result.output,
      stdout: result.stdout,
      stderr: result.stderr
    });
  } catch (error) {
    console.error('Error running clippy:', error);
    res.status(500).json({ error: 'Failed to run clippy' });
  }
});

// Helper function to count total exercises
async function countTotalExercises() {
  try {
    const exercisesPath = path.join(__dirname, '../exercises');
    let totalCount = 0;
    
    if (await fs.pathExists(exercisesPath)) {
      const chapters = await fs.readdir(exercisesPath);
      
      for (const chapter of chapters) {
        const chapterPath = path.join(exercisesPath, chapter);
        const stat = await fs.stat(chapterPath);
        
        if (stat.isDirectory() && chapter.startsWith('ch')) {
          const exercises = await fs.readdir(chapterPath);
          for (const exercise of exercises) {
            const exercisePath = path.join(chapterPath, exercise);
            const exerciseStat = await fs.stat(exercisePath);
            if (exerciseStat.isDirectory() && exercise.startsWith('ex')) {
              totalCount++;
            }
          }
        }
      }
    }
    
    return totalCount > 0 ? totalCount : 50; // Fallback to reasonable default
  } catch (error) {
    console.error('Error counting exercises:', error);
    return 50; // Safe fallback
  }
}

// Helper function to ensure progress file exists with proper structure
async function ensureProgressFile(progressPath) {
  try {
    // Ensure the progress directory exists
    await fs.ensureDir(path.dirname(progressPath));
    
    // Get actual exercise count
    const totalExercises = await countTotalExercises();
    
    // Check if progress file exists
    if (!(await fs.pathExists(progressPath))) {
      console.log('Creating new progress file:', progressPath);
      console.log(`Detected ${totalExercises} total exercises`);
      
      const defaultProgress = {
        user_id: 'default',
        created_at: new Date().toISOString(),
        overall_progress: 0.0,
        chapters_completed: 0,
        exercises_completed: 0,
        total_exercises: totalExercises,
        current_streak: 0,
        longest_streak: 0,
        total_time_minutes: 0,
        chapters: {},
        exercise_history: [],
        achievements: [],
        session_stats: {
          exercises_viewed: 0,
          exercises_completed: 0,
          hints_used: 0,
          time_spent: 0
        }
      };
      
      await fs.writeJson(progressPath, defaultProgress, { spaces: 2 });
      console.log('Progress file created successfully');
      return defaultProgress;
    }
    
    // File exists, load and return it
    const progress = await fs.readJson(progressPath);
    
    // Ensure all required properties exist (for backwards compatibility)
    if (!progress.session_stats) {
      progress.session_stats = {
        exercises_viewed: 0,
        exercises_completed: 0,
        hints_used: 0,
        time_spent: 0
      };
    }
    if (!progress.exercise_history) progress.exercise_history = [];
    if (!progress.achievements) progress.achievements = [];
    if (!progress.chapters) progress.chapters = {};
    
    // Update total exercises count if it's wrong or missing
    if (!progress.total_exercises || progress.total_exercises === 200) {
      progress.total_exercises = totalExercises;
      await fs.writeJson(progressPath, progress, { spaces: 2 });
      console.log(`Updated total exercises count to ${totalExercises}`);
    }
    
    return progress;
  } catch (error) {
    console.error('Error ensuring progress file:', error);
    throw error;
  }
}

// Get progress
app.get('/api/progress', async (req, res) => {
  try {
    const progressPath = path.join(__dirname, '../progress/user_progress.json');
    const progress = await ensureProgressFile(progressPath);
    res.json(progress);
  } catch (error) {
    console.error('Error loading progress:', error);
    res.status(500).json({ error: 'Failed to load progress' });
  }
});

// Update progress
app.post('/api/progress/complete', async (req, res) => {
  try {
    const { exercise_id, time_taken_minutes } = req.body;
    const progressPath = path.join(__dirname, '../progress/user_progress.json');
    
    // Load existing progress or create default
    const progress = await ensureProgressFile(progressPath);
    
    // Check if already completed to avoid duplicates
    const existingEntry = progress.exercise_history.find(entry => entry.exercise_id === exercise_id);
    const isAlreadyCompleted = existingEntry && existingEntry.completed_at;
    
    console.log(`Checking completion for ${exercise_id}:`);
    console.log(`Existing entry:`, existingEntry);
    console.log(`Is already completed:`, isAlreadyCompleted);
    
    if (isAlreadyCompleted) {
      console.log(`Exercise ${exercise_id} already completed`);
      return res.json({ success: true, message: 'Exercise already completed' });
    }
    
    // Update progress
    progress.exercises_completed += 1;
    progress.session_stats.exercises_completed += 1;
    progress.total_time_minutes += time_taken_minutes || 0;
    progress.overall_progress = progress.exercises_completed / progress.total_exercises;
    
    // Update or add to exercise history
    if (existingEntry) {
      // Update existing entry
      existingEntry.completed_at = new Date().toISOString();
      existingEntry.time_taken_minutes = time_taken_minutes || 0;
      existingEntry.status = 'completed';
      existingEntry.session_id = `session_${Date.now()}`;
    } else {
      // Add new entry
      progress.exercise_history.push({
        exercise_id: exercise_id,
        completed_at: new Date().toISOString(),
        time_taken_minutes: time_taken_minutes || 0,
        status: 'completed',
        session_id: `session_${Date.now()}`
      });
    }
    
    // Save updated progress
    await fs.ensureDir(path.dirname(progressPath));
    await fs.writeJson(progressPath, progress, { spaces: 2 });
    
    console.log(`Exercise completed: ${exercise_id} in ${time_taken_minutes || 0} minutes`);
    console.log(`Total exercises completed: ${progress.exercises_completed}/${progress.total_exercises}`);
    console.log(`Progress saved to: ${progressPath}`);
    
    res.json({ success: true, progress: progress });
  } catch (error) {
    console.error('Error updating progress:', error);
    res.status(500).json({ error: 'Failed to update progress' });
  }
});

// Track hint usage
app.post('/api/progress/hint', async (req, res) => {
  try {
    const { exercise_id, hint_level } = req.body;
    const progressPath = path.join(__dirname, '../progress/user_progress.json');
    
    // Load existing progress or create default
    const progress = await ensureProgressFile(progressPath);
    
    // Update hint usage stats
    progress.session_stats.hints_used += 1;
    
    // Add to exercise history if not already tracked for this exercise
    const existingEntry = progress.exercise_history.find(entry => entry.exercise_id === exercise_id);
    if (existingEntry) {
      if (!existingEntry.hints_used) existingEntry.hints_used = [];
      if (!existingEntry.hints_used.includes(hint_level)) {
        existingEntry.hints_used.push(hint_level);
      }
    } else {
      // Create new entry for this exercise
      progress.exercise_history.push({
        exercise_id: exercise_id,
        viewed_at: new Date().toISOString(),
        hints_used: [hint_level],
        status: 'in_progress'
      });
    }
    
    // Save updated progress
    await fs.writeJson(progressPath, progress, { spaces: 2 });
    
    console.log(`Hint used: ${exercise_id}, level ${hint_level}`);
    res.json({ success: true, progress: progress });
  } catch (error) {
    console.error('Error tracking hint usage:', error);
    res.status(500).json({ error: 'Failed to track hint usage' });
  }
});

// Track exercise view
app.post('/api/progress/view', async (req, res) => {
  try {
    const { exercise_id } = req.body;
    const progressPath = path.join(__dirname, '../progress/user_progress.json');
    
    // Load existing progress or create default
    const progress = await ensureProgressFile(progressPath);
    
    // Update view stats
    progress.session_stats.exercises_viewed += 1;
    
    // Check if already viewed
    const existingEntry = progress.exercise_history.find(entry => entry.exercise_id === exercise_id);
    if (!existingEntry) {
      // Create new entry for this exercise
      progress.exercise_history.push({
        exercise_id: exercise_id,
        viewed_at: new Date().toISOString(),
        status: 'viewed'
      });
    }
    
    // Save updated progress
    await fs.writeJson(progressPath, progress, { spaces: 2 });
    
    console.log(`Exercise viewed: ${exercise_id}`);
    res.json({ success: true, progress: progress });
  } catch (error) {
    console.error('Error tracking exercise view:', error);
    res.status(500).json({ error: 'Failed to track exercise view' });
  }
});

// Get Rust Book chapter content (proxy to avoid CORS issues)
app.get('/api/book/:chapter', async (req, res) => {
  try {
    const { chapter } = req.params;
    const bookUrl = `https://doc.rust-lang.org/book/ch${chapter}.html`;
    
    // In a real implementation, you might want to cache this
    // For now, just return the URL for client-side fetching
    res.json({ 
      url: bookUrl,
      chapter: chapter
    });
  } catch (error) {
    console.error('Error fetching book content:', error);
    res.status(500).json({ error: 'Failed to fetch book content' });
  }
});

// File watching for auto-reload
const exercisesPath = path.join(__dirname, '../exercises');
const watcher = chokidar.watch(exercisesPath, {
  ignored: /(^|[\/\\])\../, // ignore dotfiles
  persistent: true
});

watcher.on('change', async (filePath) => {
  const relativePath = path.relative(exercisesPath, filePath);
  
  // Extract exercise name from path (e.g., "ch01_getting_started/ex01_hello_world/src/main.rs")
  const pathParts = relativePath.split(path.sep);
  if (pathParts.length >= 2) {
    const chapterDir = pathParts[0];
    const exerciseDir = pathParts[1];
    
    try {
      // Try to load the exercise metadata to get the title
      const metadataPath = path.join(exercisesPath, chapterDir, exerciseDir, 'metadata.json');
      if (await fs.pathExists(metadataPath)) {
        const metadata = await fs.readJson(metadataPath);
        broadcast({
          type: 'file_changed',
          exercise: metadata.title || exerciseDir,
          file: relativePath
        });
        return;
      }
    } catch (error) {
      // If metadata loading fails, fall back to directory name
    }
    
    // Fallback: use exercise directory name
    broadcast({
      type: 'file_changed',
      exercise: exerciseDir.replace(/_/g, ' ').replace(/^ex\d+_/, ''),
      file: relativePath
    });
  } else {
    // Fallback: use the file path
    broadcast({
      type: 'file_changed',
      file: relativePath
    });
  }
});

// Serve React app for all non-API routes
app.get('*', (req, res) => {
  res.sendFile(path.join(__dirname, 'dist', 'index.html'));
});

// Handle WebSocket upgrade requests
server.on('upgrade', (request, socket, head) => {
  if (request.url === '/ws') {
    wss.handleUpgrade(request, socket, head, (ws) => {
      wss.emit('connection', ws, request);
    });
  } else {
    socket.destroy();
  }
});

// Initialize progress system on startup
async function initializeProgressSystem() {
  try {
    const progressPath = path.join(__dirname, '../progress/user_progress.json');
    await ensureProgressFile(progressPath);
    console.log('📊 Progress system initialized');
  } catch (error) {
    console.error('Failed to initialize progress system:', error);
  }
}

// Start server
server.listen(PORT, async () => {
  console.log(`🌐 Rust Learning Platform server running on http://localhost:${PORT}`);
  console.log(`📡 WebSocket available at ws://localhost:${PORT}/ws`);
  console.log(`🦀 Ready to serve Rust learning exercises!`);
  
  // Initialize progress system
  await initializeProgressSystem();
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('Shutting down gracefully...');
  wss.close();
  process.exit(0);
});