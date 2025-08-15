import './styles.css';
import { ExerciseManager } from './js/exercise-manager.js';
import { ProgressTracker } from './js/progress-tracker.js';
import { BookIntegration } from './js/book-integration.js';
import { WebSocketManager } from './js/websocket-manager.js';
import { UI } from './js/ui.js';
import { TerminalManager } from './js/terminal.js';

class RustTour {
  constructor() {
    this.exerciseManager = new ExerciseManager();
    this.progressTracker = new ProgressTracker();
    this.bookIntegration = new BookIntegration();
    this.websocket = new WebSocketManager();
    this.ui = new UI();
    
    // Calculate initial terminal font size based on current font size setting
    const currentFontSize = localStorage.getItem('fontSize') || 'medium';
    const fontSizeMap = { small: 12, medium: 13, large: 14 };
    const initialTerminalFontSize = fontSizeMap[currentFontSize];
    
    this.terminal = new TerminalManager(this.websocket, initialTerminalFontSize);
    
    this.currentExercise = null;
    this.exercises = [];
  }

  async init() {
    try {
      // Initialize components
      await this.exerciseManager.init();
      
      // Load exercises first
      this.exercises = await this.exerciseManager.loadExercises();
      
      // Initialize progress tracker with exercise count
      await this.progressTracker.init(this.exercises);
      
      this.websocket.init(this.handleWebSocketMessage.bind(this));
      
      // Initialize UI
      this.ui.init(this);
      this.ui.updateExerciseList(this.exercises);
      
      // Initialize terminal
      this.terminal.init('terminal');
      
      // Apply initial font size to terminal
      const currentFontSize = localStorage.getItem('fontSize') || 'medium';
      const terminalSizeMap = { small: 12, medium: 13, large: 14 };
      const terminalFontSize = terminalSizeMap[currentFontSize];
      this.terminal.updateFontSize(terminalFontSize);
      
      this.setupEventListeners();
      
      // Load default exercise or last opened
      const lastExercise = localStorage.getItem('lastExercise');
      if (lastExercise) {
        await this.loadExercise(lastExercise);
      } else if (this.exercises.length > 0) {
        await this.loadExercise(this.exercises[0].path);
      }
      
      // Hide loading screen and show app
      document.getElementById('loading').style.display = 'none';
      document.getElementById('app').classList.add('loaded');
      
      console.log('🦀 Rust Tour loaded successfully!');
    } catch (error) {
      console.error('Failed to initialize platform:', error);
      this.ui.showError('Failed to initialize Rust Tour. Please refresh and try again.');
    }
  }

  setupEventListeners() {
    // Exercise selection
    document.addEventListener('exercise-selected', (e) => {
      this.loadExercise(e.detail.path);
    });

    // Code execution
    document.addEventListener('run-code', () => {
      this.runCode();
    });

    document.addEventListener('test-code', async () => {
      await this.saveCode();
      await this.testCode();
    });

    document.addEventListener('check-code', async () => {
      await this.saveCode();
      await this.checkCode();
    });

    // Code saving
    document.addEventListener('save-code', () => {
      this.saveCode();
    });

    // Save and run (sequential operation)
    document.addEventListener('save-and-run-code', async () => {
      await this.saveCode();
      await this.runCode();
    });

    // Hints
    document.addEventListener('show-hint', (e) => {
      this.showHint(e.detail.level);
    });

    // Progress
    document.addEventListener('complete-exercise', () => {
      this.completeExercise();
    });

    // Auto-save on code change
    let saveTimeout;
    document.addEventListener('code-changed', () => {
      clearTimeout(saveTimeout);
      saveTimeout = setTimeout(() => {
        this.saveCode(true); // silent save
      }, 2000);
    });

    // Terminal controls
    document.addEventListener('click', (e) => {
      if (e.target.id === 'terminal-btn') {
        this.toggleTerminal();
      } else if (e.target.id === 'terminal-close-btn') {
        this.hideTerminal();
      }
    });

    // Terminal keyboard shortcuts
    document.addEventListener('keydown', (e) => {
      if (e.ctrlKey && e.key === '~') {
        e.preventDefault();
        this.toggleTerminal();
      }
    });
  }

  async loadExercise(exercisePath) {
    try {
      this.ui.setLoading(true);
      
      const exercise = await this.exerciseManager.loadExercise(exercisePath);
      this.currentExercise = exercise;
      
      // Update UI
      this.ui.updateExercise(exercise);
      await this.ui.updateBookPanel(exercise.metadata.rust_book_refs);
      
      // Update progress
      await this.progressTracker.trackExerciseViewed(exercise.metadata.id);
      
      // Save last exercise
      localStorage.setItem('lastExercise', exercisePath);
      
      // Navigate terminal to new exercise directory if terminal is open
      const terminalContainer = document.getElementById('terminal-container');
      if (terminalContainer && terminalContainer.style.display !== 'none') {
        this.terminal.navigateToExercise(exercise.path);
      }
      
      console.log(`Loaded exercise: ${exercise.metadata.title}`);
    } catch (error) {
      console.error('Failed to load exercise:', error);
      this.ui.showError('Failed to load exercise. Please try again.');
    } finally {
      this.ui.setLoading(false);
    }
  }

  async runCode() {
    if (!this.currentExercise) return;

    try {
      this.ui.setExecutionStatus('running', 'Running code...');
      
      const result = await this.exerciseManager.runExercise(this.currentExercise.path);
      
      if (result.success) {
        this.ui.setExecutionStatus('success', 'Code executed successfully!');
        this.ui.updateOutput(result.stdout, 'stdout');
      } else {
        this.ui.setExecutionStatus('error', 'Execution failed');
        this.ui.updateOutput(result.stderr, 'stderr');
      }
    } catch (error) {
      console.error('Failed to run code:', error);
      this.ui.setExecutionStatus('error', 'Failed to run code');
      this.ui.showError('Failed to run code. Please try again.');
    }
  }

  async testCode() {
    if (!this.currentExercise) return;

    try {
      this.ui.setExecutionStatus('running', 'Running tests...');
      
      const result = await this.exerciseManager.testExercise(this.currentExercise.path);
      
      if (result.success) {
        this.ui.setExecutionStatus('success', 'All tests passed! 🎉');
        this.ui.updateTestResults(result.stdout);
        
        // Check if this completes the exercise
        if (this.isExerciseComplete(result)) {
          this.ui.showCompletionCelebration();
          await this.completeExercise();
        }
      } else {
        this.ui.setExecutionStatus('warning', 'Some tests failed');
        this.ui.updateTestResults(result.output);
      }
    } catch (error) {
      console.error('Failed to run tests:', error);
      this.ui.setExecutionStatus('error', 'Failed to run tests');
      this.ui.showError('Failed to run tests. Please try again.');
    }
  }

  async checkCode() {
    if (!this.currentExercise) return;

    try {
      this.ui.setExecutionStatus('running', 'Checking code quality...');
      
      const result = await this.exerciseManager.checkExercise(this.currentExercise.path);
      
      if (result.success) {
        this.ui.setExecutionStatus('success', 'Code quality looks great!');
      } else {
        this.ui.setExecutionStatus('info', 'Code quality suggestions available');
      }
      
      this.ui.updateClippyResults(result.stderr);
    } catch (error) {
      console.error('Failed to check code:', error);
      this.ui.setExecutionStatus('error', 'Failed to check code');
      this.ui.showError('Failed to check code. Please try again.');
    }
  }

  async saveCode(silent = false) {
    if (!this.currentExercise) return;

    try {
      const code = this.ui.getEditorContent();
      await this.exerciseManager.saveCode(this.currentExercise.path, code);
      
      if (!silent) {
        this.ui.showSaveSuccess();
      }
    } catch (error) {
      console.error('Failed to save code:', error);
      if (!silent) {
        this.ui.showError('Failed to save code. Please try again.');
      }
    }
  }

  showHint(level) {
    if (!this.currentExercise) return;
    
    const hints = this.parseHints(this.currentExercise.hints);
    const hint = hints[level - 1];
    
    if (hint) {
      this.ui.showHint(hint, level);
      this.progressTracker.trackHintUsed(this.currentExercise.metadata.id, level);
    } else {
      this.ui.showError('No hint available for this level.');
    }
  }

  parseHints(hintsMarkdown) {
    const hints = [];
    const sections = hintsMarkdown.split(/## Level \d+:/);
    
    for (let i = 1; i < sections.length; i++) {
      hints.push({
        title: sections[i].split('\n')[0].trim(),
        content: sections[i].substring(sections[i].indexOf('\n') + 1).trim()
      });
    }
    
    return hints;
  }

  async completeExercise() {
    if (!this.currentExercise) return;
    
    const timeSpent = this.progressTracker.getTimeSpentOnCurrentExercise();
    
    await this.progressTracker.completeExercise(
      this.currentExercise.id || this.currentExercise.metadata.id,
      timeSpent
    );
    
    this.ui.showExerciseCompletion(this.currentExercise.metadata);
    
    // Show completion celebration but no dialog for next exercise
    // (dialog removed per user request, but celebration popup remains)
  }

  getNextExercise() {
    const currentIndex = this.exercises.findIndex(ex => 
      ex.path === this.currentExercise.path
    );
    
    if (currentIndex !== -1 && currentIndex < this.exercises.length - 1) {
      return this.exercises[currentIndex + 1];
    }
    
    return null;
  }

  isExerciseComplete(testResult) {
    // Exercise is complete when all functional tests pass
    return testResult.success;
  }

  handleWebSocketMessage(data) {
    switch (data.type) {
      case 'file_updated':
        if (data.exercise === this.currentExercise?.path) {
          // this.ui.showNotification('File updated externally');
        }
        break;
      case 'file_changed':
        // Handle file system changes
        break;
    }
  }

  // Terminal control methods
  toggleTerminal() {
    const terminalBtn = document.getElementById('terminal-btn');
    const terminalContainer = document.getElementById('terminal-container');
    
    if (terminalContainer.style.display === 'none') {
      this.showTerminal();
    } else {
      this.hideTerminal();
    }
  }

  showTerminal() {
    const terminalBtn = document.getElementById('terminal-btn');
    terminalBtn.classList.add('active');
    
    this.terminal.show();
    
    // Navigate to current exercise directory if one is loaded
    if (this.currentExercise) {
      this.terminal.navigateToExercise(this.currentExercise.path);
    }
  }

  hideTerminal() {
    const terminalBtn = document.getElementById('terminal-btn');
    terminalBtn.classList.remove('active');
    
    this.terminal.hide();
  }
}

// Initialize the platform when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  const platform = new RustTour();
  platform.init();
});