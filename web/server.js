const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
const path = require('path');
const fs = require('fs-extra');
const { spawn } = require('child_process');
const WebSocket = require('ws');
const chokidar = require('chokidar');

const app = express();
const PORT = process.env.PORT || 3000;

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

// WebSocket server for real-time updates
const wss = new WebSocket.Server({ port: 8080 });

// Store active connections
const connections = new Set();

wss.on('connection', (ws) => {
  connections.add(ws);
  console.log('Client connected to WebSocket');
  
  ws.on('close', () => {
    connections.delete(ws);
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
    const hintsPath = path.join(exercisePath, 'src/hints.md');
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
    
    // Broadcast file change to connected clients
    broadcast({
      type: 'file_updated',
      exercise: `${chapter}/${exercise}`,
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

// Get progress
app.get('/api/progress', async (req, res) => {
  try {
    const progressPath = path.join(__dirname, '../progress/user_progress.json');
    
    if (await fs.pathExists(progressPath)) {
      const progress = await fs.readJson(progressPath);
      res.json(progress);
    } else {
      // Return default progress
      res.json({
        user_id: 'default',
        created_at: new Date().toISOString(),
        overall_progress: 0.0,
        chapters_completed: 0,
        exercises_completed: 0,
        total_exercises: 200,
        chapters: {},
        exercise_history: [],
        achievements: []
      });
    }
  } catch (error) {
    console.error('Error loading progress:', error);
    res.status(500).json({ error: 'Failed to load progress' });
  }
});

// Update progress
app.post('/api/progress/complete', async (req, res) => {
  try {
    const { exercise_id, time_taken_minutes } = req.body;
    
    // This would integrate with the Rust progress system
    // For now, just acknowledge the completion
    console.log(`Exercise completed: ${exercise_id} in ${time_taken_minutes} minutes`);
    
    res.json({ success: true });
  } catch (error) {
    console.error('Error updating progress:', error);
    res.status(500).json({ error: 'Failed to update progress' });
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

watcher.on('change', (filePath) => {
  const relativePath = path.relative(exercisesPath, filePath);
  broadcast({
    type: 'file_changed',
    file: relativePath
  });
});

// Serve React app for all non-API routes
app.get('*', (req, res) => {
  res.sendFile(path.join(__dirname, 'dist', 'index.html'));
});

// Start server
app.listen(PORT, () => {
  console.log(`🌐 Rust Learning Platform server running on http://localhost:${PORT}`);
  console.log(`📡 WebSocket server running on ws://localhost:8080`);
  console.log(`🦀 Ready to serve Rust learning exercises!`);
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('Shutting down gracefully...');
  wss.close();
  process.exit(0);
});