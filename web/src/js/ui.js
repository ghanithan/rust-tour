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
            <span class="hamburger-icon">☰</span>
          </button>
          <div class="logo">
            <span class="logo-icon">🦀</span>
            <span>Rust Learning Platform</span>
          </div>
          <div class="header-controls">
            <button class="theme-toggle" id="theme-toggle" title="Toggle Theme">
              <span class="theme-icon">🌙</span>
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
            <span>📚 Exercises</span>
          </div>
          <div class="exercise-list" id="exercise-list">
          </div>
        </aside>

        <!-- Editor Area -->
        <main class="editor-container">
          <div class="editor-header">
            <div class="editor-title" id="editor-title">Select an exercise to begin</div>
            <div class="editor-actions">
              <button class="btn btn-secondary" id="save-btn" title="Save (Ctrl+S)">
                💾 Save
              </button>
              <button class="btn btn-primary" id="run-btn" title="Run (Ctrl+Enter)">
                ▶️ Run
              </button>
              <button class="btn btn-success" id="test-btn" title="Test (Ctrl+T)">
                🧪 Test
              </button>
              <button class="btn btn-secondary" id="check-btn" title="Check (Ctrl+L)">
                🔍 Check
              </button>
              <button class="btn btn-secondary" id="terminal-btn" title="Terminal (Ctrl+~)">
                🖥️ Terminal
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
            <div class="output-tab active" data-tab="output">📝 Output</div>
            <div class="output-tab" data-tab="tests">🧪 Tests</div>
            <div class="output-tab" data-tab="clippy">🔍 Clippy</div>
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
            <span class="terminal-title">🖥️ Terminal</span>
            <div class="terminal-controls">
              <button class="btn btn-small" id="terminal-minimize-btn" title="Minimize Terminal">−</button>
              <button class="btn btn-small" id="terminal-maximize-btn" title="Maximize Terminal" style="display: none;">□</button>
              <button class="btn btn-small" id="terminal-close-btn" title="Close Terminal">✕</button>
            </div>
          </div>
          <div class="terminal-content" id="terminal"></div>
        </section>

        <!-- Right Panel - Book Integration & Hints -->
        <aside class="right-panel">
          <div class="panel-header">
            <span class="panel-title">📖 Learning Resources</span>
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
            <div class="panel-tab active" data-tab="info">Info</div>
            <div class="panel-tab" data-tab="hints">Hints</div>
            <div class="panel-tab" data-tab="book">Book</div>
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
                    💡 Level 1: Conceptual Hint
                  </button>
                  <div class="hint-content" id="hint-1"></div>
                </div>
                <div class="hint-level">
                  <button class="hint-button" data-level="2" disabled>
                    🎯 Level 2: Strategic Hint
                  </button>
                  <div class="hint-content" id="hint-2"></div>
                </div>
                <div class="hint-level">
                  <button class="hint-button" data-level="3" disabled>
                    🔧 Level 3: Implementation Hint
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
      document.dispatchEvent(new CustomEvent('run-code'));
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

    // Theme toggle handler
    document.getElementById('theme-toggle').addEventListener('click', () => {
      this.toggleTheme();
    });

    // ESC key handler
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.sidebarOpen) {
        this.closeSidebar();
      }
    });

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
    document.getElementById('run-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('run-code'));
    });

    document.getElementById('test-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('test-code'));
    });

    document.getElementById('check-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('check-code'));
    });

    document.getElementById('save-btn').addEventListener('click', () => {
      document.dispatchEvent(new CustomEvent('save-code'));
    });

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
        
        const statusIcon = isCompleted ? '✅' : '📝';
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
          <h3 style="margin-bottom: 15px; color: var(--rust-orange);">📖 Instructions</h3>
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
        <h3 class="book-link-title">📖 ${section.title}</h3>
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
    // this.showNotification(`🎉 "${exerciseMetadata.title}" completed!`, 'success');
    // Refresh exercise list to show completion status
    if (this.platform && this.platform.exercises) {
      this.updateExerciseList(this.platform.exercises);
    }
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
    this.updateThemeIcon();
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
      themeIcon.textContent = this.currentTheme === 'dark' ? '☀️' : '🌙';
    }
    
    // Update tooltip
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
      themeToggle.title = `Switch to ${this.currentTheme === 'dark' ? 'light' : 'dark'} theme`;
    }
  }
}