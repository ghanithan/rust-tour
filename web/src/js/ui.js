import * as monaco from 'monaco-editor';
import { marked } from 'marked';

// Configure Monaco Environment for web workers
self.MonacoEnvironment = {
  getWorkerUrl: function (moduleId, label) {
    if (label === 'json') {
      return './monaco-editor/esm/vs/language/json/json.worker.js';
    }
    if (label === 'css' || label === 'scss' || label === 'less') {
      return './monaco-editor/esm/vs/language/css/css.worker.js';
    }
    if (label === 'html' || label === 'handlebars' || label === 'razor') {
      return './monaco-editor/esm/vs/language/html/html.worker.js';
    }
    if (label === 'typescript' || label === 'javascript') {
      return './monaco-editor/esm/vs/language/typescript/ts.worker.js';
    }
    return './monaco-editor/esm/vs/editor/editor.worker.js';
  }
};

export class UI {
  constructor() {
    this.editor = null;
    this.currentExercise = null;
    this.platform = null;
    this.notifications = [];
    this.sidebarOpen = false;
    this.rightPanelOpen = false;
    this.exerciseHierarchy = null; // Will store the hierarchical structure
    this.currentTheme = localStorage.getItem('theme') || 'dark';
  }

  async init(platform) {
    this.platform = platform;
    this.setupLayout();
    this.initializeTheme();
    await this.initializeEditor();
    this.setupEventHandlers();
    this.setupOutputResize();
    console.log('UI initialized');
  }

  setupLayout() {
    const app = document.getElementById('app');
    app.innerHTML = `
      <div class="main-layout">
        <!-- Header -->
        <header class="header">
          <button class="hamburger-menu" id="hamburger-menu" title="Toggle Exercise Menu">
            <span class="hamburger-icon"><i class="fas fa-bars"></i></span>
          </button>
          <div class="logo">
            <img src="/images/ferris-official.svg" alt="Ferris" class="logo-icon ferris-logo">
            <span>Rust Learning Platform</span>
          </div>
          <div class="header-controls">
            <button class="theme-toggle" id="theme-toggle" title="Toggle Theme">
              <span class="theme-icon"><i class="fas fa-moon"></i></span>
            </button>
            <div class="progress-indicator">
              <span id="progress-text">0% Complete</span>
            </div>
          </div>
        </header>

        <!-- Sidebar Backdrop -->
        <div class="sidebar-backdrop" id="sidebar-backdrop"></div>

        <!-- Sidebar - Exercise Navigation -->
        <aside class="sidebar" id="sidebar">
          <div class="sidebar-header">
            <span><i class="fas fa-terminal"></i> Exercises</span>
          </div>
          <div class="exercise-list" id="exercise-list">
          </div>
        </aside>

        <!-- Editor Area -->
        <main class="editor-container">
          <div class="editor-header">
            <div class="editor-title" id="editor-title">Select an exercise to begin</div>
            <div class="editor-actions">
              <button class="btn btn-primary" id="save-run-btn" title="Save and Run (Ctrl+Enter)">
                <i class="fas fa-save"></i> <i class="fas fa-play"></i> Save and Run
              </button>
              <button class="btn btn-success" id="test-btn" title="Test (Ctrl+T)">
                <i class="fas fa-vial"></i> Test
              </button>
              <button class="btn btn-secondary" id="check-btn" title="Check (Ctrl+L)">
                <i class="fas fa-search"></i> Check
              </button>
              <button class="btn btn-secondary" id="terminal-btn" title="Terminal (Ctrl+~)">
                <i class="fas fa-terminal"></i> Terminal
              </button>
            </div>
          </div>
          <div id="editor"></div>
          <div class="status-bar">
            <div class="status-message" id="status-message">
              <span class="status-icon"></span>
              <span class="status-text">Ready</span>
            </div>
          </div>
        </main>

        <!-- Output Panel -->
        <section class="output-container">
          <div class="output-header">
            <div class="output-tab active" data-tab="output"><i class="fas fa-terminal"></i> Output</div>
            <div class="output-tab" data-tab="tests"><i class="fas fa-flask"></i> Tests</div>
            <div class="output-tab" data-tab="clippy"><i class="fas fa-search"></i> Clippy</div>
          </div>
          <div class="output-content">
            <div class="output-panel active" id="output-panel">
              <div class="output-placeholder">Run your code to see output here</div>
            </div>
            <div class="output-panel" id="tests-panel">
              <div class="output-placeholder">Run tests to see results here</div>
            </div>
            <div class="output-panel" id="clippy-panel">
              <div class="output-placeholder">Check code quality to see suggestions here</div>
            </div>
          </div>
        </section>

        <!-- Terminal Container -->
        <section class="terminal-container" id="terminal-container" style="display: none;">
          <div class="terminal-resize-handle" id="terminal-resize-handle"></div>
          <div class="terminal-header" id="terminal-header">
            <span class="terminal-title"><i class="fas fa-terminal"></i> Terminal</span>
            <div class="terminal-controls">
              <button class="btn btn-small" id="terminal-minimize-btn" title="Minimize Terminal"><i class="fas fa-minus"></i></button>
              <button class="btn btn-small" id="terminal-maximize-btn" title="Maximize Terminal" style="display: none;"><i class="fas fa-expand"></i></button>
              <button class="btn btn-small" id="terminal-close-btn" title="Close Terminal"><i class="fas fa-times"></i></button>
            </div>
          </div>
          <div class="terminal-content" id="terminal"></div>
        </section>

        <!-- Horizontal Resize Handle -->
        <div class="horizontal-resize-handle" id="horizontal-resize-handle"></div>

        <!-- Right Panel Trigger for Responsive -->
        <button class="right-panel-trigger" id="right-panel-trigger" style="display: none;">
          <i class="fas fa-chevron-left trigger-icon"></i>
          <span class="trigger-text">LEARN</span>
        </button>
        
        <!-- Right Panel Backdrop -->
        <div class="right-panel-backdrop" id="right-panel-backdrop"></div>

        <!-- Right Panel - Book Integration & Hints -->
        <aside class="right-panel" id="right-panel">
          <div class="panel-header">
            <span class="panel-title"><i class="fas fa-book-open"></i> Learning Resources</span>
            <div class="panel-nav-buttons">
              <button class="panel-nav-btn" id="prev-chapter-btn" title="Previous Exercise" disabled>
                ‹
              </button>
              <span class="exercise-counter" id="exercise-counter">-/-</span>
              <button class="panel-nav-btn" id="next-chapter-btn" title="Next Exercise" disabled>
                ›
              </button>
            </div>
          </div>
          <div class="panel-tabs">
            <div class="panel-tab active" data-tab="info"><i class="fas fa-info-circle"></i> Info</div>
            <div class="panel-tab" data-tab="hints"><i class="fas fa-lightbulb"></i> Hints</div>
            <div class="panel-tab" data-tab="book"><i class="fas fa-book"></i> Book</div>
          </div>
          <div class="panel-content">
            <div class="panel-section active" id="info-section">
              <div id="exercise-info">
                Select an exercise to see details
              </div>
            </div>
            <div class="panel-section" id="hints-section">
              <div class="hints-section">
                <div class="hint-level">
                  <button class="hint-button" data-level="1">
                    <i class="fas fa-lightbulb"></i> Level 1: Conceptual Hint
                  </button>
                  <div class="hint-content" id="hint-1"></div>
                </div>
                <div class="hint-level">
                  <button class="hint-button" data-level="2" disabled>
                    <i class="fas fa-crosshairs"></i> Level 2: Strategic Hint
                  </button>
                  <div class="hint-content" id="hint-2"></div>
                </div>
                <div class="hint-level">
                  <button class="hint-button" data-level="3" disabled>
                    <i class="fas fa-wrench"></i> Level 3: Implementation Hint
                  </button>
                  <div class="hint-content" id="hint-3"></div>
                </div>
              </div>
            </div>
            <div class="panel-section" id="book-section">
              <div class="book-links" id="book-links">
                Select an exercise to see book references
              </div>
            </div>
          </div>
        </aside>
      </div>
    `;
  }

  async initializeEditor() {
    // Configure Monaco Editor for Rust
    monaco.languages.register({ id: 'rust' });
    
    // Set up Rust syntax highlighting (basic)
    monaco.languages.setMonarchTokensProvider('rust', {
      tokenizer: {
        root: [
          [/\b(?:fn|let|mut|const|static|struct|enum|impl|trait|mod|pub|use|extern|crate|self|super|as|where|for|in|while|loop|if|else|match|break|continue|return|yield|async|await|move|ref|unsafe|dyn|abstract|become|box|do|final|macro|override|priv|typeof|unsized|virtual|try|catch)\b/, 'keyword'],
          [/\b(?:bool|u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize|f32|f64|char|str|String|Vec|Option|Result|Box|Rc|Arc|RefCell|Mutex|HashMap|HashSet)\b/, 'type'],
          [/\b(?:true|false|None|Some|Ok|Err|self|Self)\b/, 'constant'],
          [/"([^"\\\\]|\\\\.)*"/, 'string'],
          [/'([^'\\\\]|\\\\.)*'/, 'string.char'],
          [/\/\/.*$/, 'comment'],
          [/\/\*[\s\S]*?\*\//, 'comment'],
          [/\b\d+\b/, 'number'],
          [/[{}()\[\]]/, 'delimiter'],
          [/[<>=!]+/, 'operator'],
          [/[+\-*/%&|^~]+/, 'operator'],
        ]
      }
    });

    // Initialize editor
    this.editor = monaco.editor.create(document.getElementById('editor'), {
      value: '// Select an exercise to begin coding\nfn main() {\n    println!("Hello, Rust!");\n}',
      language: 'rust',
      theme: 'vs-dark',
      fontSize: 14,
      fontFamily: 'Monaco, Menlo, Ubuntu Mono, monospace',
      minimap: { enabled: true },
      scrollBeyondLastLine: false,
      automaticLayout: true,
      formatOnPaste: true,
      formatOnType: true,
      wordWrap: 'on',
      lineNumbers: 'on',
      rulers: [80, 120],
      renderWhitespace: 'boundary',
      bracketPairColorization: { enabled: true }
    });

    // Add editor event listeners
    this.editor.onDidChangeModelContent(() => {
      document.dispatchEvent(new CustomEvent('code-changed'));
    });

    // Add keyboard shortcuts
    this.editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => {
      document.dispatchEvent(new CustomEvent('save-and-run-code'));
    });

    this.editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyT, () => {
      document.dispatchEvent(new CustomEvent('test-code'));
    });

    this.editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyL, () => {
      document.dispatchEvent(new CustomEvent('check-code'));
    });

    this.editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
      document.dispatchEvent(new CustomEvent('save-code'));
    });
  }

  setupEventHandlers() {
    // Hamburger menu handler
    document.getElementById('hamburger-menu').addEventListener('click', () => {
      this.toggleSidebar();
    });

    // Backdrop handler
    document.getElementById('sidebar-backdrop').addEventListener('click', () => {
      this.closeSidebar();
    });

    // Right panel trigger handler
    document.getElementById('right-panel-trigger').addEventListener('click', () => {
      this.toggleRightPanel();
    });

    // Right panel backdrop handler
    document.getElementById('right-panel-backdrop').addEventListener('click', () => {
      this.closeRightPanel();
    });

    // Theme toggle handler
    document.getElementById('theme-toggle').addEventListener('click', () => {
      this.toggleTheme();
    });

    // ESC key handler
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        if (this.sidebarOpen) {
          this.closeSidebar();
        }
        if (this.rightPanelOpen) {
          this.closeRightPanel();
        }
      }
    });

    // Window resize handler for responsive behavior
    let resizeTimer;
    window.addEventListener('resize', () => {
      // Disable transitions during resize
      document.body.classList.add('disable-transitions');
      
      this.handleResize();
      
      // Re-enable transitions after resize is complete
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(() => {
        document.body.classList.remove('disable-transitions');
      }, 300);
    });

    // Initial resize check
    this.handleResize();

    // Horizontal resize handler
    this.setupHorizontalResize();

    // Chapter navigation handlers
    const prevBtn = document.getElementById('prev-chapter-btn');
    const nextBtn = document.getElementById('next-chapter-btn');
    
    if (prevBtn && nextBtn) {
      prevBtn.addEventListener('click', () => {
        console.log('Previous chapter button clicked');
        this.navigateToChapter('previous');
      });

      nextBtn.addEventListener('click', () => {
        console.log('Next chapter button clicked');
        this.navigateToChapter('next');
      });
    } else {
      console.error('Navigation buttons not found:', { prevBtn, nextBtn });
    }

    // Button handlers
    document.getElementById('save-run-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('save-and-run-code'));
    });

    document.getElementById('test-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('test-code'));
    });

    document.getElementById('check-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('check-code'));
    });

    // Save button removed - merged with run button

    // Output tab switching
    document.querySelectorAll('.output-tab').forEach(tab => {
      tab.addEventListener('click', () => {
        this.switchOutputTab(tab.dataset.tab);
      });
    });

    // Panel tab switching
    document.querySelectorAll('.panel-tab').forEach(tab => {
      tab.addEventListener('click', () => {
        this.switchPanelTab(tab.dataset.tab);
      });
    });

    // Hint buttons
    document.querySelectorAll('.hint-button').forEach(button => {
      button.addEventListener('click', () => {
        const level = parseInt(button.dataset.level);
        document.dispatchEvent(new CustomEvent('show-hint', { 
          detail: { level } 
        }));
      });
    });
    
    // Terminal close button
    document.getElementById('terminal-close-btn').addEventListener('click', (e) => {
      e.stopPropagation();
      document.dispatchEvent(new CustomEvent('toggle-terminal'));
    });

    // Handle all links to open in new tabs
    document.addEventListener('click', (e) => {
      if (e.target && e.target.tagName === 'A' && e.target.href) {
        // Check if it's an external link or if it doesn't have target="_blank" already
        if (e.target.href.startsWith('http') && !e.target.target) {
          e.preventDefault();
          window.open(e.target.href, '_blank', 'noopener,noreferrer');
        }
      }
    });
  }

  updateExerciseList(exercises) {
    const listContainer = document.getElementById('exercise-list');
    listContainer.innerHTML = '';

    // Rebuild the exercise hierarchy when exercises are updated
    this.exerciseHierarchy = this.buildExerciseHierarchy();

    // Group exercises by chapter
    const chapters = {};
    exercises.forEach(exercise => {
      const chapterNum = exercise.metadata ? exercise.metadata.chapter : exercise.chapter;
      if (!chapters[chapterNum]) {
        chapters[chapterNum] = [];
      }
      chapters[chapterNum].push(exercise);
    });

    // Render chapters
    Object.keys(chapters).sort((a, b) => parseInt(a) - parseInt(b)).forEach(chapterNum => {
      const chapterDiv = document.createElement('div');
      chapterDiv.className = 'chapter-group';
      
      const chapterTitle = document.createElement('div');
      chapterTitle.className = 'chapter-title';
      chapterTitle.textContent = `Chapter ${chapterNum}`;
      chapterDiv.appendChild(chapterTitle);

      chapters[chapterNum].forEach(exercise => {
        const exerciseDiv = document.createElement('div');
        exerciseDiv.className = 'exercise-item';
        exerciseDiv.dataset.path = exercise.path;
        
        // Check if exercise is completed
        const isCompleted = this.platform && this.platform.progressTracker ? 
          this.platform.progressTracker.isExerciseCompleted(exercise.id) : false;
        
        const statusIcon = isCompleted ? '<i class="fas fa-check-circle text-success"></i>' : '<i class="fas fa-circle text-muted"></i>';
        const completedClass = isCompleted ? ' completed' : '';
        
        exerciseDiv.className = `exercise-item${completedClass}`;
        exerciseDiv.innerHTML = `
          <div class="exercise-status">
            <span class="status-icon">${statusIcon}</span>
          </div>
          <div class="exercise-info">
            <div class="exercise-title">${exercise.title}</div>
            <div class="exercise-meta">
              <span>${exercise.difficulty}</span>
              <span>${exercise.estimated_time_minutes}min</span>
              ${isCompleted ? '<span class="completed-badge">Completed</span>' : ''}
            </div>
          </div>
        `;

        exerciseDiv.addEventListener('click', () => {
          document.dispatchEvent(new CustomEvent('exercise-selected', {
            detail: { path: exercise.path }
          }));
          this.closeSidebar();
        });

        chapterDiv.appendChild(exerciseDiv);
      });

      listContainer.appendChild(chapterDiv);
    });
  }

  updateExercise(exercise) {
    this.currentExercise = exercise;
    
    // Update editor title
    document.getElementById('editor-title').textContent = exercise.metadata.title;
    
    // Update editor content
    this.editor.setValue(exercise.mainContent);
    
    // Update exercise info (now includes instructions)
    this.updateExerciseInfo(exercise);
    
    // Update active exercise in sidebar
    document.querySelectorAll('.exercise-item').forEach(item => {
      item.classList.remove('active');
      if (item.dataset.path === exercise.path) {
        item.classList.add('active');
      }
    });

    // Reset hints
    this.resetHints();
    
    // Clear output panels
    this.clearOutputPanels();
    
    // Update chapter navigation
    this.updateChapterNavigation();
    
    // Switch to info tab by default
    this.switchPanelTab('info');
  }


  updateExerciseInfo(exercise) {
    const infoContainer = document.getElementById('exercise-info');
    
    // Render instructions if available
    let instructionsHtml = '';
    if (exercise.readme) {
      instructionsHtml = `
        <div style="margin-top: 20px; padding-top: 20px; border-top: 1px solid var(--border-primary);">
          <h3 style="margin-bottom: 15px; color: var(--rust-orange);"><i class="fas fa-file-alt"></i> Instructions</h3>
          <div class="markdown-content">
            ${this.renderMarkdown(exercise.readme)}
          </div>
        </div>
      `;
    }
    
    infoContainer.innerHTML = `
      <div style="padding: 0; margin: 0;">
        <h1 style="margin: 0 0 12px 0; color: var(--rust-orange); font-size: 24px; font-weight: 700;">${exercise.metadata.title}</h1>
        <p style="margin: 0 0 16px 0; line-height: 1.6; font-size: 14px; color: var(--text-primary);">${exercise.metadata.description}</p>
        
        <div style="margin: 0 0 10px 0; color: var(--text-primary);">
          <strong>Difficulty:</strong> 
          <span class="difficulty-${exercise.metadata.difficulty}">${exercise.metadata.difficulty}</span>
        </div>
        
        <div style="margin: 0 0 10px 0; color: var(--text-primary);">
          <strong>Estimated Time:</strong> ${exercise.metadata.estimated_time_minutes} minutes
        </div>
        
        <div style="margin: 0 0 15px 0; color: var(--text-primary);">
          <strong>Concepts:</strong><br>
          ${exercise.metadata.concepts.map(concept => `<span class="concept-tag">${concept}</span>`).join(' ')}
        </div>
        
        ${instructionsHtml}
      </div>
      
      <style>
        #info-section {
          padding: 0.75rem !important;
        }
        
        .difficulty-beginner { color: var(--status-success); }
        .difficulty-intermediate { color: var(--status-warning); }
        .difficulty-advanced { color: var(--status-error); }
        
        .concept-tag {
          display: inline-block;
          background: var(--bg-accent);
          padding: 2px 6px;
          border-radius: 3px;
          font-size: 11px;
          margin: 2px 2px 2px 0;
        }
      </style>
    `;
  }

  updateBookPanel(bookRefs) {
    const bookContainer = document.getElementById('book-links');
    
    if (!bookRefs || !bookRefs.specific_sections || bookRefs.specific_sections.length === 0) {
      bookContainer.innerHTML = '<p style="padding: 15px; color: var(--text-muted);">No book references available for this exercise.</p>';
      return;
    }

    bookContainer.innerHTML = bookRefs.specific_sections.map(section => `
      <a href="${section.url}" target="_blank" class="book-link">
        <h3 class="book-link-title"><i class="fas fa-external-link-alt"></i> ${section.title}</h3>
        <h4 class="book-link-chapter">Chapter ${section.chapter} • ${section.relevance}</h4>
      </a>
    `).join('');
  }

  switchOutputTab(tabName) {
    document.querySelectorAll('.output-tab').forEach(tab => {
      tab.classList.remove('active');
    });
    document.querySelectorAll('.output-panel').forEach(panel => {
      panel.classList.remove('active');
    });

    document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
    document.getElementById(`${tabName}-panel`).classList.add('active');
  }

  switchPanelTab(tabName) {
    document.querySelectorAll('.panel-tab').forEach(tab => {
      tab.classList.remove('active');
    });
    document.querySelectorAll('.panel-section').forEach(section => {
      section.classList.remove('active');
    });

    document.querySelector(`[data-tab="${tabName}"].panel-tab`).classList.add('active');
    document.getElementById(`${tabName}-section`).classList.add('active');
  }

  updateOutput(content, type = 'stdout') {
    const outputPanel = document.getElementById('output-panel');
    const className = type === 'stderr' ? 'output-stderr' : 'output-stdout';
    
    outputPanel.innerHTML = `<div class="${className}">${this.escapeHtml(content)}</div>`;
    
    // Switch to output tab
    this.switchOutputTab('output');
    
    // Auto-scroll to bottom
    this.scrollOutputToBottom();
  }

  updateTestResults(content) {
    const testsPanel = document.getElementById('tests-panel');
    testsPanel.innerHTML = `<div class="output-stdout">${this.escapeHtml(content)}</div>`;
    
    // Switch to tests tab
    this.switchOutputTab('tests');
    
    // Auto-scroll to bottom
    this.scrollOutputToBottom();
  }

  updateClippyResults(content) {
    const clippyPanel = document.getElementById('clippy-panel');
    clippyPanel.innerHTML = `<div class="output-stdout">${this.escapeHtml(content)}</div>`;
    
    // Switch to clippy tab if there are suggestions
    if (content.trim()) {
      this.switchOutputTab('clippy');
      // Auto-scroll to bottom
      this.scrollOutputToBottom();
    }
  }

  setExecutionStatus(status, message) {
    const statusMessage = document.getElementById('status-message');
    const statusIcon = statusMessage.querySelector('.status-icon');
    const statusText = statusMessage.querySelector('.status-text');
    
    statusIcon.className = `status-icon ${status}`;
    statusText.textContent = message;
  }

  showHint(hint, level) {
    const hintButton = document.querySelector(`[data-level="${level}"]`);
    const hintContent = document.getElementById(`hint-${level}`);
    
    // Toggle visibility
    if (hintContent.classList.contains('visible')) {
      // Close the hint
      hintContent.classList.remove('visible');
    } else {
      // Open the hint
      hintButton.classList.add('used');
      hintContent.innerHTML = this.formatHintContent(hint.content);
      hintContent.classList.add('visible');
      
      // Enable next hint button
      if (level < 3) {
        document.querySelector(`[data-level="${level + 1}"]`).disabled = false;
      }
      
      // Switch to hints tab
      this.switchPanelTab('hints');
    }
  }

  formatHintContent(content) {
    // Convert markdown-like content to HTML
    return content
      .replace(/```rust\n([\s\S]*?)\n```/g, '<pre><code>$1</code></pre>')
      .replace(/`([^`]+)`/g, '<code>$1</code>')
      .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
      .replace(/\n/g, '<br>');
  }

  resetHints() {
    document.querySelectorAll('.hint-button').forEach((button, index) => {
      button.classList.remove('used');
      button.disabled = index > 0;
    });
    
    document.querySelectorAll('.hint-content').forEach(content => {
      content.classList.remove('visible');
      content.innerHTML = '';
    });
  }

  clearOutputPanels() {
    document.getElementById('output-panel').innerHTML = '<div class="output-placeholder">Run your code to see output here</div>';
    document.getElementById('tests-panel').innerHTML = '<div class="output-placeholder">Run tests to see results here</div>';
    document.getElementById('clippy-panel').innerHTML = '<div class="output-placeholder">Check code quality to see suggestions here</div>';
  }

  getEditorContent() {
    return this.editor.getValue();
  }

  setLoading(loading) {
    const buttons = document.querySelectorAll('.btn');
    buttons.forEach(btn => {
      btn.disabled = loading;
    });
    
    if (loading) {
      this.setExecutionStatus('info', 'Loading...');
    } else {
      this.setExecutionStatus('success', 'Ready');
    }
  }

  showNotification(message, type = 'info') {
    const notification = document.createElement('div');
    notification.className = `notification ${type}`;
    notification.textContent = message;
    
    document.body.appendChild(notification);
    
    // Show notification
    setTimeout(() => notification.classList.add('visible'), 100);
    
    // Hide after 1 second for less intrusion
    setTimeout(() => {
      notification.classList.remove('visible');
      setTimeout(() => document.body.removeChild(notification), 300);
    }, 1000);
  }

  showError(message) {
    // this.showNotification(message, 'error');
    console.error('[UI Error]:', message); // Keep for debugging
  }

  showSaveSuccess() {
    // this.showNotification('Code saved successfully', 'success');
  }

  showCompletionCelebration() {
    // this.showNotification('🎉 Congratulations! Exercise completed!', 'success');
    // Refresh exercise list to show completion status
    if (this.platform && this.platform.exercises) {
      this.updateExerciseList(this.platform.exercises);
    }
  }

  showExerciseCompletion(exerciseMetadata) {
    // Get next exercise info
    const nextExercise = this.platform ? this.platform.getNextExercise() : null;
    
    // Create celebration backdrop
    const backdrop = document.createElement('div');
    backdrop.className = 'celebration-backdrop';
    backdrop.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      background: rgba(0, 0, 0, 0.7);
      z-index: 1000;
      opacity: 0;
      transition: opacity 0.3s ease;
    `;
    
    // Create completion celebration modal
    const modal = document.createElement('div');
    modal.className = 'completion-celebration';
    modal.innerHTML = `
      <div class="celebration-content">
        <div class="celebration-header">
          <div class="celebration-ferris">
            <div class="ferris-container">
              <img src="/images/ferris-happy.png" 
                   alt="Ferris the Rustacean celebrating" 
                   class="ferris-image"
                   title="Ferris the Rustacean by Karen Rustad Tölva • CC0" />
              <div class="ferris-sparkles">
                <span class="sparkle sparkle-1">✨</span>
                <span class="sparkle sparkle-2">⭐</span>
                <span class="sparkle sparkle-3">✨</span>
                <span class="sparkle sparkle-4">⭐</span>
                <span class="sparkle sparkle-5">✨</span>
                <span class="sparkle sparkle-6">⭐</span>
              </div>
            </div>
          </div>
          <div class="celebration-text">
            <h2><i class="fas fa-trophy"></i> Excellent Work!</h2>
            <p>You've successfully completed</p>
            <h3>"${exerciseMetadata.title}"</h3>
          </div>
        </div>
        <div class="celebration-actions">
          ${nextExercise ? `
            <button class="btn btn-rust-primary next-exercise-btn" data-exercise-path="${nextExercise.path}">
              <i class="fas fa-arrow-right"></i> Next Exercise
            </button>
          ` : `
            <button class="btn btn-rust-primary continue-learning-btn">
              <i class="fas fa-graduation-cap"></i> Continue Learning
            </button>
          `}
        </div>
      </div>
    `;
    
    // Style the modal with Rust theme
    modal.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%) scale(0.7);
      background: linear-gradient(135deg, #2d5016 0%, #3d6b1f 100%);
      border: 3px solid #62c132;
      border-radius: 16px;
      box-shadow: 0 20px 60px rgba(98, 193, 50, 0.3), 0 0 0 1px rgba(98, 193, 50, 0.1);
      z-index: 1001;
      max-width: 500px;
      width: 90vw;
      opacity: 0;
      transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
      overflow: hidden;
    `;
    
    // Add comprehensive styles
    const styles = document.createElement('style');
    styles.textContent = `
      @keyframes celebrationPulse {
        0%, 100% { transform: scale(1); }
        50% { transform: scale(1.05); }
      }
      
      @keyframes celebrationGlow {
        0%, 100% { box-shadow: 0 0 20px rgba(98, 193, 50, 0.3); }
        50% { box-shadow: 0 0 40px rgba(98, 193, 50, 0.6); }
      }
      
      @keyframes celebrationFloat {
        0%, 100% { transform: translateY(0px); }
        50% { transform: translateY(-10px); }
      }
      
      @keyframes ferrisBounce {
        0%, 100% { transform: rotate(-5deg) scale(1); }
        25% { transform: rotate(5deg) scale(1.1); }
        50% { transform: rotate(-3deg) scale(1.05); }
        75% { transform: rotate(3deg) scale(1.1); }
      }
      
      @keyframes sparkleFloat {
        0% { 
          transform: translateY(0px) rotate(0deg);
          opacity: 0;
        }
        10% { opacity: 1; }
        90% { opacity: 1; }
        100% { 
          transform: translateY(-50px) rotate(360deg);
          opacity: 0;
        }
      }
      
      @keyframes sparkleFloat2 {
        0% { 
          transform: translateY(0px) rotate(0deg) scale(0.8);
          opacity: 0;
        }
        15% { opacity: 1; }
        85% { opacity: 1; }
        100% { 
          transform: translateY(-60px) rotate(-360deg) scale(1.2);
          opacity: 0;
        }
      }
      
      .celebration-content {
        padding: 32px;
        text-align: center;
        color: white;
      }
      
      .celebration-header {
        margin-bottom: 32px;
      }
      
      .celebration-ferris {
        margin-bottom: 24px;
        animation: celebrationFloat 3s ease-in-out infinite;
        position: relative;
      }
      
      .ferris-container {
        position: relative;
        display: inline-block;
        width: 120px;
        height: 120px;
      }
      
      .ferris-image {
        width: 120px;
        height: 120px;
        animation: ferrisBounce 2.5s ease-in-out infinite;
        filter: drop-shadow(0 0 20px rgba(98, 193, 50, 0.4));
        border-radius: 10px;
        background: radial-gradient(circle, rgba(98, 193, 50, 0.1) 0%, transparent 70%);
      }
      
      
      .ferris-sparkles {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
      }
      
      .sparkle {
        position: absolute;
        font-size: 20px;
        animation-duration: 2s;
        animation-iteration-count: infinite;
        animation-timing-function: ease-in-out;
      }
      
      .sparkle-1 {
        top: 10%;
        left: 20%;
        animation-name: sparkleFloat;
        animation-delay: 0s;
      }
      
      .sparkle-2 {
        top: 20%;
        right: 15%;
        animation-name: sparkleFloat2;
        animation-delay: 0.3s;
      }
      
      .sparkle-3 {
        bottom: 20%;
        left: 10%;
        animation-name: sparkleFloat;
        animation-delay: 0.6s;
      }
      
      .sparkle-4 {
        bottom: 10%;
        right: 20%;
        animation-name: sparkleFloat2;
        animation-delay: 0.9s;
      }
      
      .sparkle-5 {
        top: 50%;
        left: 5%;
        animation-name: sparkleFloat;
        animation-delay: 1.2s;
      }
      
      .sparkle-6 {
        top: 40%;
        right: 5%;
        animation-name: sparkleFloat2;
        animation-delay: 1.5s;
      }
      
      .celebration-text h2 {
        margin: 0 0 8px 0;
        font-size: 28px;
        font-weight: 700;
        color: #62c132;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12px;
      }
      
      .celebration-text h2 i {
        color: #f1c40f;
        animation: celebrationGlow 2s ease-in-out infinite;
      }
      
      .celebration-text p {
        margin: 0 0 8px 0;
        font-size: 16px;
        color: #e8f4e8;
        opacity: 0.9;
      }
      
      .celebration-text h3 {
        margin: 0;
        font-size: 20px;
        color: white;
        font-weight: 600;
        background: rgba(255, 255, 255, 0.1);
        padding: 8px 16px;
        border-radius: 8px;
        display: inline-block;
      }
      
      .celebration-actions {
        display: flex;
        justify-content: center;
        gap: 16px;
      }
      
      .btn-rust-primary {
        background: linear-gradient(135deg, #62c132 0%, #4a9625 100%);
        color: white;
        border: 2px solid #62c132;
        padding: 12px 24px;
        font-size: 16px;
        font-weight: 600;
        border-radius: 8px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 10px;
        transition: all 0.3s ease;
        text-decoration: none;
        box-shadow: 0 4px 12px rgba(98, 193, 50, 0.3);
      }
      
      .btn-rust-primary:hover {
        background: linear-gradient(135deg, #4a9625 0%, #3d7a1f 100%);
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(98, 193, 50, 0.4);
      }
      
      .btn-rust-primary:active {
        transform: translateY(0px);
      }
      
      .celebration-backdrop {
        backdrop-filter: blur(4px);
      }
    `;
    document.head.appendChild(styles);
    
    // Add elements to DOM
    document.body.appendChild(backdrop);
    document.body.appendChild(modal);
    
    // Animate in
    setTimeout(() => {
      backdrop.style.opacity = '1';
      modal.style.opacity = '1';
      modal.style.transform = 'translate(-50%, -50%) scale(1)';
    }, 50);
    
    // Add event listeners
    const nextBtn = modal.querySelector('.next-exercise-btn');
    const continueBtn = modal.querySelector('.continue-learning-btn');
    
    const closeModal = () => {
      modal.style.opacity = '0';
      modal.style.transform = 'translate(-50%, -50%) scale(0.7)';
      backdrop.style.opacity = '0';
      setTimeout(() => {
        if (document.body.contains(modal)) document.body.removeChild(modal);
        if (document.body.contains(backdrop)) document.body.removeChild(backdrop);
        if (document.head.contains(styles)) document.head.removeChild(styles);
      }, 400);
    };
    
    if (nextBtn) {
      nextBtn.addEventListener('click', () => {
        const exercisePath = nextBtn.dataset.exercisePath;
        document.dispatchEvent(new CustomEvent('exercise-selected', {
          detail: { path: exercisePath }
        }));
        closeModal();
      });
    }
    
    if (continueBtn) {
      continueBtn.addEventListener('click', () => {
        // Find the next exercise manually if getNextExercise didn't find one
        const allExercises = this.platform ? this.platform.exercises : [];
        const currentIndex = allExercises.findIndex(ex => 
          ex.path === this.platform.currentExercise?.path
        );
        
        if (currentIndex !== -1 && currentIndex < allExercises.length - 1) {
          // There is a next exercise, navigate to it
          const nextExercisePath = allExercises[currentIndex + 1].path;
          document.dispatchEvent(new CustomEvent('exercise-selected', {
            detail: { path: nextExercisePath }
          }));
        }
        closeModal();
      });
    }
    
    // Close on backdrop click
    backdrop.addEventListener('click', closeModal);
    
    // Auto-close after 15 seconds
    setTimeout(closeModal, 15000);
    
    // Refresh exercise list to show completion status
    if (this.platform && this.platform.exercises) {
      this.updateExerciseList(this.platform.exercises);
    }
  }
  
  dismissNotification(notification) {
    notification.style.opacity = '0';
    notification.style.transform = 'translate(-50%, -50%) scale(0.8)';
    setTimeout(() => {
      if (document.body.contains(notification)) {
        document.body.removeChild(notification);
      }
    }, 300);
  }

  suggestNextExercise(nextExercise) {
    setTimeout(() => {
      const shouldContinue = confirm(`Great job! Would you like to move on to the next exercise: "${nextExercise.title}"?`);
      if (shouldContinue) {
        document.dispatchEvent(new CustomEvent('exercise-selected', {
          detail: { path: nextExercise.path }
        }));
      }
    }, 1000);
  }

  escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  scrollOutputToBottom() {
    const outputContent = document.querySelector('.output-content');
    if (outputContent) {
      // Delay to ensure content is rendered
      setTimeout(() => {
        outputContent.scrollTop = outputContent.scrollHeight;
      }, 0);
    }
  }


  renderMarkdown(markdown) {
    // Configure marked options
    marked.setOptions({
      breaks: true,
      gfm: true,
      highlight: function(code, language) {
        // Basic code highlighting (could be enhanced with syntax highlighting library)
        return code;
      }
    });
    
    // Convert markdown to HTML - the global click handler will handle opening in new tabs
    return marked.parse(markdown);
  }

  toggleSidebar() {
    this.sidebarOpen = !this.sidebarOpen;
    const sidebar = document.querySelector('.sidebar');
    const backdrop = document.getElementById('sidebar-backdrop');
    
    if (this.sidebarOpen) {
      sidebar.classList.add('open');
      backdrop.classList.add('visible');
    } else {
      sidebar.classList.remove('open');
      backdrop.classList.remove('visible');
    }
  }

  closeSidebar() {
    this.sidebarOpen = false;
    const sidebar = document.querySelector('.sidebar');
    const backdrop = document.getElementById('sidebar-backdrop');
    sidebar.classList.remove('open');
    backdrop.classList.remove('visible');
  }

  toggleRightPanel() {
    this.rightPanelOpen = !this.rightPanelOpen;
    const rightPanel = document.getElementById('right-panel');
    const backdrop = document.getElementById('right-panel-backdrop');
    const trigger = document.getElementById('right-panel-trigger');
    
    if (this.rightPanelOpen) {
      rightPanel.classList.add('open');
      backdrop.classList.add('visible');
      trigger.classList.add('open');
    } else {
      rightPanel.classList.remove('open');
      backdrop.classList.remove('visible');
      trigger.classList.remove('open');
    }
  }

  closeRightPanel() {
    this.rightPanelOpen = false;
    const rightPanel = document.getElementById('right-panel');
    const backdrop = document.getElementById('right-panel-backdrop');
    const trigger = document.getElementById('right-panel-trigger');
    rightPanel.classList.remove('open');
    backdrop.classList.remove('visible');
    trigger.classList.remove('open');
  }

  handleResize() {
    const trigger = document.getElementById('right-panel-trigger');
    const rightPanel = document.getElementById('right-panel');
    const resizeHandle = document.getElementById('horizontal-resize-handle');
    const mainLayout = document.querySelector('.main-layout');
    
    // Show/hide trigger based on screen size (64rem = 1024px)
    if (window.innerWidth <= 1024) {
      trigger.style.display = 'flex';
      resizeHandle.style.display = 'none';
      // Reset to responsive grid layout - remove inline styles to let CSS take over
      mainLayout.style.removeProperty('grid-template-columns');
      // Close panel on resize to smaller screen
      if (this.rightPanelOpen) {
        this.closeRightPanel();
      }
    } else {
      trigger.style.display = 'none';
      resizeHandle.style.display = 'flex';
      // Restore saved desktop layout using ratios
      const savedLeftRatio = localStorage.getItem('layout-left-ratio') || '1';
      const savedRightRatio = localStorage.getItem('layout-right-ratio') || '1';
      mainLayout.style.gridTemplateColumns = `${savedLeftRatio}fr 0.25rem ${savedRightRatio}fr`;
      // Ensure panel is closed when switching back to desktop
      this.closeRightPanel();
    }
  }

  setupHorizontalResize() {
    const resizeHandle = document.getElementById('horizontal-resize-handle');
    const mainLayout = document.querySelector('.main-layout');
    let isResizing = false;
    let startX = 0;
    let startLeftWidth = 0;
    
    // Only apply saved layout if we're on desktop (> 1024px)
    if (window.innerWidth > 1024) {
      const savedLeftRatio = localStorage.getItem('layout-left-ratio') || '1';
      const savedRightRatio = localStorage.getItem('layout-right-ratio') || '1';
      mainLayout.style.gridTemplateColumns = `${savedLeftRatio}fr 0.25rem ${savedRightRatio}fr`;
    }

    // Double-click to reset to default 50/50 split
    resizeHandle.addEventListener('dblclick', () => {
      mainLayout.style.gridTemplateColumns = '1fr 0.25rem 1fr';
      localStorage.setItem('layout-left-ratio', '1');
      localStorage.setItem('layout-right-ratio', '1');
    });

    resizeHandle.addEventListener('mousedown', (e) => {
      isResizing = true;
      startX = e.clientX;
      
      // Get current column widths
      const computedStyle = window.getComputedStyle(mainLayout);
      const columns = computedStyle.gridTemplateColumns.split(' ');
      startLeftWidth = parseFloat(columns[0]);
      
      document.body.classList.add('resizing-horizontal');
      e.preventDefault();
    });

    document.addEventListener('mousemove', (e) => {
      if (!isResizing) return;

      const deltaX = e.clientX - startX;
      const containerWidth = mainLayout.offsetWidth;
      const deltaPercent = (deltaX / containerWidth) * 100;
      
      // Calculate new widths with constraints
      let newLeftPercent = (startLeftWidth / containerWidth) * 100 + deltaPercent;
      let newRightPercent = 100 - newLeftPercent;
      
      // Constrain to reasonable limits (20% to 80%)
      newLeftPercent = Math.max(20, Math.min(80, newLeftPercent));
      newRightPercent = 100 - newLeftPercent;
      
      // Convert to fr ratios for flexible scaling
      const leftRatio = newLeftPercent / 50; // Normalize to 1 as baseline (50% = 1fr)
      const rightRatio = newRightPercent / 50;
      
      // Apply new layout using fr units
      mainLayout.style.gridTemplateColumns = `${leftRatio}fr 0.25rem ${rightRatio}fr`;
    });

    document.addEventListener('mouseup', () => {
      if (isResizing) {
        isResizing = false;
        document.body.classList.remove('resizing-horizontal');
        
        // Save layout preferences as ratios
        // Instead of trying to parse computed style, calculate ratios directly from current percentage
        const containerWidth = mainLayout.offsetWidth;
        const leftElement = document.querySelector('.editor-container');
        const leftWidth = leftElement.offsetWidth;
        const leftPercent = (leftWidth / containerWidth) * 100;
        const rightPercent = 100 - leftPercent;
        
        // Convert to fr ratios (normalize to 1 as baseline for 50%)
        const leftRatio = leftPercent / 50;
        const rightRatio = rightPercent / 50;
        
        localStorage.setItem('layout-left-ratio', leftRatio.toFixed(3));
        localStorage.setItem('layout-right-ratio', rightRatio.toFixed(3));
      }
    });
  }

  buildExerciseHierarchy() {
    if (!this.platform || !this.platform.exercises) {
      return null;
    }

    // Create flat array of all exercises in order
    const allExercises = [...this.platform.exercises].sort((a, b) => {
      const chapterA = a.metadata ? a.metadata.chapter : a.chapter;
      const chapterB = b.metadata ? b.metadata.chapter : b.chapter;
      const exerciseA = a.metadata ? a.metadata.exercise_number : 1;
      const exerciseB = b.metadata ? b.metadata.exercise_number : 1;
      
      if (chapterA !== chapterB) {
        return chapterA - chapterB;
      }
      return exerciseA - exerciseB;
    });

    console.log('Exercise hierarchy built:', allExercises.map(ex => ({
      path: ex.path,
      chapter: ex.metadata?.chapter,
      exercise: ex.metadata?.exercise_number
    })));

    return allExercises;
  }

  findCurrentExerciseIndex() {
    if (!this.exerciseHierarchy || !this.currentExercise) {
      return -1;
    }
    
    return this.exerciseHierarchy.findIndex(ex => ex.path === this.currentExercise.path);
  }

  getExercisePositionInChapter() {
    if (!this.currentExercise || !this.platform || !this.platform.exercises) {
      return { current: 0, total: 0 };
    }

    const currentChapter = this.currentExercise.metadata ? this.currentExercise.metadata.chapter : this.currentExercise.chapter;
    const currentExerciseNum = this.currentExercise.metadata ? this.currentExercise.metadata.exercise_number : 1;

    console.log('Counter calc - Current chapter:', currentChapter, 'Current exercise num:', currentExerciseNum);

    // Get all exercises in the current chapter
    const chapterExercises = this.platform.exercises.filter(ex => {
      const exerciseChapter = ex.metadata ? ex.metadata.chapter : ex.chapter;
      return exerciseChapter === currentChapter;
    }).sort((a, b) => {
      const exerciseA = a.metadata ? a.metadata.exercise_number : 1;
      const exerciseB = b.metadata ? b.metadata.exercise_number : 1;
      return exerciseA - exerciseB;
    });

    console.log('Chapter exercises:', chapterExercises.map(ex => ({
      path: ex.path,
      exercise_number: ex.metadata?.exercise_number
    })));

    // Find position by matching the actual current exercise path instead of just exercise number
    const currentPosition = chapterExercises.findIndex(ex => ex.path === this.currentExercise.path) + 1;

    console.log('Position calculation:', {
      currentPosition,
      total: chapterExercises.length,
      currentExercisePath: this.currentExercise.path
    });

    return {
      current: currentPosition,
      total: chapterExercises.length
    };
  }

  navigateToChapter(direction) {
    // Build hierarchy if not exists
    if (!this.exerciseHierarchy) {
      this.exerciseHierarchy = this.buildExerciseHierarchy();
    }
    
    if (!this.exerciseHierarchy || !this.currentExercise) {
      console.log('Navigation failed: missing data');
      return;
    }

    const currentIndex = this.findCurrentExerciseIndex();
    console.log('Current exercise:', this.currentExercise.path, 'index:', currentIndex);
    
    if (currentIndex === -1) {
      console.log('Current exercise not found in hierarchy');
      return;
    }
    
    const targetIndex = direction === 'next' ? currentIndex + 1 : currentIndex - 1;
    console.log('Target index:', targetIndex, 'direction:', direction);
    
    // Check bounds
    if (targetIndex < 0 || targetIndex >= this.exerciseHierarchy.length) {
      console.log('No target exercise available');
      return;
    }
    
    const targetExercise = this.exerciseHierarchy[targetIndex];
    console.log('Target exercise:', targetExercise.path);
    console.log('Current chapter:', this.currentExercise.metadata?.chapter, 'Target chapter:', targetExercise.metadata?.chapter);
    
    if (targetExercise) {
      document.dispatchEvent(new CustomEvent('exercise-selected', {
        detail: { path: targetExercise.path }
      }));
    }
  }

  updateChapterNavigation() {
    // Build hierarchy if not exists
    if (!this.exerciseHierarchy) {
      this.exerciseHierarchy = this.buildExerciseHierarchy();
    }
    
    if (!this.exerciseHierarchy || !this.currentExercise) {
      return;
    }

    const currentIndex = this.findCurrentExerciseIndex();
    const hasPrevious = currentIndex > 0;
    const hasNext = currentIndex < this.exerciseHierarchy.length - 1;

    const prevBtn = document.getElementById('prev-chapter-btn');
    const nextBtn = document.getElementById('next-chapter-btn');
    const counter = document.getElementById('exercise-counter');
    
    if (prevBtn && nextBtn) {
      prevBtn.disabled = !hasPrevious;
      nextBtn.disabled = !hasNext;
    }

    // Update exercise counter
    if (counter) {
      const position = this.getExercisePositionInChapter();
      counter.textContent = `${position.current}/${position.total}`;
    }
  }

  setupOutputResize() {
    const mainLayout = document.querySelector('.main-layout');
    const editorContainer = document.querySelector('.editor-container');
    const outputContainer = document.querySelector('.output-container');
    
    if (!mainLayout || !editorContainer || !outputContainer) {
      console.warn('Required elements for resize not found');
      return;
    }
    
    // Create resize handle
    const resizeHandle = document.createElement('div');
    resizeHandle.className = 'output-resize-handle';
    resizeHandle.innerHTML = '<div class="resize-handle-bar"></div>';
    
    // Insert resize handle at the top of the output container
    outputContainer.insertBefore(resizeHandle, outputContainer.firstChild);
    console.log('Resize handle inserted:', resizeHandle);
    
    let isResizing = false;
    let startY = 0;
    let startEditorHeight = 0;
    let startOutputHeight = 0;
    
    resizeHandle.addEventListener('mousedown', (e) => {
      isResizing = true;
      startY = e.clientY;
      
      const editorRect = editorContainer.getBoundingClientRect();
      const outputRect = outputContainer.getBoundingClientRect();
      startEditorHeight = editorRect.height;
      startOutputHeight = outputRect.height;
      
      document.body.classList.add('resizing-output');
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      e.preventDefault();
    });
    
    const handleMouseMove = (e) => {
      if (!isResizing) return;
      
      const deltaY = e.clientY - startY;
      const totalHeight = startEditorHeight + startOutputHeight;
      const minHeight = 150; // Minimum height for both sections
      
      let newEditorHeight = startEditorHeight + deltaY;
      let newOutputHeight = startOutputHeight - deltaY;
      
      // Enforce minimum heights
      if (newEditorHeight < minHeight) {
        newEditorHeight = minHeight;
        newOutputHeight = totalHeight - minHeight;
      } else if (newOutputHeight < minHeight) {
        newOutputHeight = minHeight;
        newEditorHeight = totalHeight - minHeight;
      }
      
      // Update grid template rows
      const headerHeight = 60;
      mainLayout.style.gridTemplateRows = `${headerHeight}px ${newEditorHeight}px ${newOutputHeight}px`;
      
      // Resize Monaco editor
      if (this.editor) {
        this.editor.layout();
      }
    };
    
    const handleMouseUp = () => {
      isResizing = false;
      document.body.classList.remove('resizing-output');
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      
      // Final resize of Monaco editor
      if (this.editor) {
        setTimeout(() => this.editor.layout(), 100);
      }
    };
  }

  // Theme management methods
  initializeTheme() {
    // Apply the current theme to the document
    document.documentElement.setAttribute('data-theme', this.currentTheme);
    
    // Use timeout to ensure DOM is ready for icon update
    setTimeout(() => {
      this.updateThemeIcon();
    }, 100);
  }

  toggleTheme() {
    this.currentTheme = this.currentTheme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', this.currentTheme);
    localStorage.setItem('theme', this.currentTheme);
    this.updateThemeIcon();
    
    // Update Monaco editor theme if it exists
    if (this.editor) {
      const theme = this.currentTheme === 'dark' ? 'vs-dark' : 'vs';
      monaco.editor.setTheme(theme);
    }
  }

  updateThemeIcon() {
    const themeIcon = document.querySelector('.theme-icon');
    if (themeIcon) {
      // Show sun when in dark mode (to switch to light), moon when in light mode (to switch to dark)
      themeIcon.innerHTML = this.currentTheme === 'dark' ? '<i class="fas fa-sun"></i>' : '<i class="fas fa-moon"></i>';
    }
    
    // Update tooltip
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
      themeToggle.title = `Switch to ${this.currentTheme === 'dark' ? 'light' : 'dark'} theme`;
    }
  }
}