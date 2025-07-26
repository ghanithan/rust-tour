import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';

export class TerminalManager {
  constructor(websocketManager) {
    this.ws = websocketManager;
    this.terminal = null;
    this.fitAddon = null;
    this.sessionId = this.loadSessionId(); // Try to restore session
    this.isInitialized = false;
    this.isMinimized = false; // Track minimize state
    this.previousHeight = Math.min(200, window.innerHeight - 120); // Align with CSS constraints
    this.debug = localStorage.getItem('DEBUG_WEBSOCKET') === 'true' || window.location.search.includes('debug=true');
    
    // Bind methods
    this.handleTerminalMessage = this.handleTerminalMessage.bind(this);
    this.onTerminalData = this.onTerminalData.bind(this);
    this.onTerminalResize = this.onTerminalResize.bind(this);
  }

  init(containerId) {
    if (this.isInitialized) {
      // If already initialized, just check if we need to recreate the session
      if (!this.sessionId) {
        this.createSession();
      }
      return;
    }

    const container = document.getElementById(containerId);
    if (!container) {
      console.error(`Terminal container ${containerId} not found`);
      return;
    }

    // Create terminal instance
    this.terminal = new Terminal({
      cursorBlink: true,
      scrollback: 1000, // Keep 1000 lines in scrollback buffer
      scrollOnUserInput: true, // Scroll on user input
      fastScrollModifier: 'alt', // Allow fast scrolling with Alt
      disableStdin: false, // Allow input
      theme: {
        background: '#1e1e1e',
        foreground: '#ffffff',
        cursor: '#ffffff',
        selection: '#3a3a3a',
        black: '#000000',
        red: '#cd3131',
        green: '#0dbc79',
        yellow: '#e5e510',
        blue: '#2472c8',
        magenta: '#bc3fbc',
        cyan: '#11a8cd',
        white: '#e5e5e5',
        brightBlack: '#666666',
        brightRed: '#f14c4c',
        brightGreen: '#23d18b',
        brightYellow: '#f5f543',
        brightBlue: '#3b8eea',
        brightMagenta: '#d670d6',
        brightCyan: '#29b8db',
        brightWhite: '#ffffff'
      },
      fontFamily: 'Monaco, "Cascadia Code", "SF Mono", Consolas, "Liberation Mono", Menlo, monospace',
      fontSize: 14,
      lineHeight: 1.2
    });

    // Create fit addon
    this.fitAddon = new FitAddon();
    this.terminal.loadAddon(this.fitAddon);

    // Open terminal
    this.terminal.open(container);
    
    // Fit to container size as defined by CSS
    this.fitAddon.fit();
    
    // Removed complex scroll observer to avoid layout conflicts

    // Set up event listeners
    this.terminal.onData(this.onTerminalData);
    this.terminal.onResize(this.onTerminalResize);
    
    // Listen for WebSocket messages
    this.ws.addMessageHandler('terminal', this.handleTerminalMessage);
    
    // Add data listener to auto-scroll when terminal receives data
    this.terminal.onWriteParsed(() => {
      // Force scroll to bottom when data is parsed
      this.forceScrollToBottom();
    });
    
    // Set up resize functionality
    this.setupResize();
    
    // Add window resize listener to refit terminal
    window.addEventListener('resize', () => {
      if (this.terminal && this.fitAddon && this.isInitialized) {
        const container = document.getElementById('terminal-container');
        if (container && container.style.display !== 'none') {
          setTimeout(() => this.fitAddon.fit(), 100);
        }
      }
    });
    
    // Create terminal session
    this.createSession();
    
    this.isInitialized = true;
    if (this.debug) {
      console.log('Terminal initialized');
    }
  }

  createSession() {
    // Check if we already have an active session
    if (this.sessionId) {
      if (this.debug) {
        console.log('Checking existing terminal session:', this.sessionId);
      }
      // Check if the existing session is still valid on the server
      this.waitForConnection(() => {
        this.ws.send({
          type: 'terminal',
          action: 'check',
          sessionId: this.sessionId
        });
      });
      return;
    }

    // Generate new session ID
    this.sessionId = `terminal_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    
    const { cols, rows } = this.terminal;
    if (this.debug) {
      console.log('Creating new terminal session:', this.sessionId, 'with dimensions:', cols, 'x', rows);
    }
    
    // Wait for WebSocket to be ready before creating session
    this.waitForConnection(() => {
      this.ws.send({
        type: 'terminal',
        action: 'create',
        sessionId: this.sessionId,
        cols: cols,
        rows: rows
      });
    });
  }

  waitForConnection(callback, retries = 0) {
    const maxRetries = 50; // 5 seconds max wait
    
    if (this.ws.ws && this.ws.ws.readyState === WebSocket.OPEN) {
      callback();
    } else if (retries < maxRetries) {
      setTimeout(() => {
        this.waitForConnection(callback, retries + 1);
      }, 100);
    } else {
      console.error('Failed to establish WebSocket connection for terminal');
    }
  }

  onTerminalData(data) {
    if (this.sessionId) {
      // Don't echo locally - let the server handle echo
      this.ws.send({
        type: 'terminal',
        action: 'input',
        sessionId: this.sessionId,
        input: data
      });
    }
  }

  onTerminalResize(size) {
    if (this.sessionId) {
      this.ws.send({
        type: 'terminal',
        action: 'resize',
        sessionId: this.sessionId,
        cols: size.cols,
        rows: size.rows
      });
    }
  }

  handleTerminalMessage(data) {
    if (this.debug) {
      console.log('Received terminal message:', data);
    }
    const { action, sessionId, data: terminalData, message } = data;
    
    if (sessionId !== this.sessionId) {
      if (this.debug) {
        console.log('Ignoring message for different session:', sessionId, 'vs', this.sessionId);
      }
      return; // Not our session
    }

    if (this.debug) {
      console.log('Processing terminal message:', action);
    }
    switch (action) {
      case 'created':
        if (this.debug) {
          console.log(`Terminal session ${sessionId} created successfully`);
        }
        break;
      case 'exists':
        if (this.debug) {
          console.log(`Terminal session ${sessionId} already exists on server`);
        }
        // Session exists, we can continue using it
        break;
      case 'not_found':
        if (this.debug) {
          console.log(`Terminal session ${sessionId} not found on server, creating new one`);
        }
        // Session doesn't exist, create a new one
        this.sessionId = null;
        this.clearSessionId();
        this.createSession();
        break;
      case 'output':
        if (this.terminal) {
          this.terminal.write(terminalData);
          // Force scroll to bottom with DOM manipulation for reliability
          this.forceScrollToBottom();
        }
        break;
      case 'exit':
        console.log(`Terminal session ${sessionId} exited`);
        if (this.terminal) {
          this.terminal.write('\r\n\x1b[31mTerminal session ended. Creating new session...\x1b[0m\r\n');
        }
        this.sessionId = null;
        this.clearSessionId();
        // Automatically create a new session after a brief delay
        setTimeout(() => {
          if (this.terminal && this.isInitialized) {
            this.createSession();
          }
        }, 1000);
        break;
      case 'error':
        console.error(`Terminal error: ${message}`);
        if (this.terminal) {
          this.terminal.write(`\r\n\x1b[31mError: ${message}\x1b[0m\r\n`);
        }
        break;
    }
  }

  show() {
    const container = document.getElementById('terminal-container');
    if (container) {
      container.style.display = 'block';
      document.body.classList.add('terminal-open');
      
      // Reset minimize state when showing
      if (this.isMinimized) {
        // If it was minimized, restore it to proper state
        container.classList.remove('minimized');
        this.isMinimized = false;
      }
      
      // Set initial height if not already set
      if (!container.style.height || container.style.height === 'auto') {
        const initialHeight = Math.min(window.innerHeight * 0.25, 300);
        container.style.height = initialHeight + 'px';
        this.previousHeight = initialHeight;
        document.body.style.setProperty('--terminal-height', initialHeight + 'px');
      }
      
      document.getElementById('terminal-minimize-btn').style.display = 'inline-block';
      document.getElementById('terminal-maximize-btn').style.display = 'none';
      
      if (this.terminal && this.fitAddon) {
        // Wait a bit for the container to be visible
        setTimeout(() => {
          this.fitAddon.fit();
          this.terminal.focus();
        }, 100);
      }
    }
  }

  hide() {
    const container = document.getElementById('terminal-container');
    if (container) {
      container.style.display = 'none';
      document.body.classList.remove('terminal-open');
      
      // Reset minimize state when hiding
      if (this.isMinimized) {
        container.classList.remove('minimized');
        container.style.height = this.previousHeight + 'px';
        document.body.style.setProperty('--terminal-height', this.previousHeight + 'px');
        this.isMinimized = false;
        
        // Reset button states and ensure safe height
        const minimizeBtn = document.getElementById('terminal-minimize-btn');
        const maximizeBtn = document.getElementById('terminal-maximize-btn');
        if (minimizeBtn) minimizeBtn.style.display = 'inline-block';
        if (maximizeBtn) maximizeBtn.style.display = 'none';
        // Ensure height doesn't exceed screen
        const maxAllowed = Math.min(300, window.innerHeight * 0.4);
        const safeHeight = Math.min(this.previousHeight, maxAllowed);
        container.style.height = safeHeight + 'px';
        document.body.style.setProperty('--terminal-height', safeHeight + 'px');
      }
    }
  }

  close() {
    // Just hide the terminal, preserve the session
    // User can reopen and continue with the same session
    this.hide();
  }

  terminate() {
    // Actually destroy the session and cleanup
    this.destroy();
    this.hide();
  }

  setupResize() {
    const resizeHandle = document.getElementById('terminal-resize-handle');
    const container = document.getElementById('terminal-container');
    const minimizeBtn = document.getElementById('terminal-minimize-btn');
    const maximizeBtn = document.getElementById('terminal-maximize-btn');
    const closeBtn = document.getElementById('terminal-close-btn');
    const terminalHeader = document.getElementById('terminal-header');
    
    if (!resizeHandle || !container) return;
    
    let isResizing = false;
    let startY = 0;
    let startHeight = 0;
    
    // Resize functionality
    resizeHandle.addEventListener('mousedown', (e) => {
      // Don't resize if minimized
      if (container.classList.contains('minimized')) return;
      
      isResizing = true;
      startY = e.clientY;
      // Get current height from style or computed style
      startHeight = parseInt(container.style.height || document.defaultView.getComputedStyle(container).height, 10);
      container.classList.add('resizing');
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      e.preventDefault();
      e.stopPropagation();
    });
    
    const handleMouseMove = (e) => {
      if (!isResizing) return;
      
      const deltaY = startY - e.clientY; // Inverted because we're resizing from bottom
      const minHeight = 100; // Minimum height
      const maxHeight = Math.min(window.innerHeight * 0.8, 600); // Max 80% of screen or 600px
      let newHeight = Math.min(Math.max(startHeight + deltaY, minHeight), maxHeight);
      
      // Update heights immediately for responsive feel
      container.style.height = newHeight + 'px';
      document.body.style.setProperty('--terminal-height', newHeight + 'px');
      
      // Store the new height
      this.previousHeight = newHeight;
      
      // Debounced terminal resize for performance
      if (this.resizeTimeout) {
        clearTimeout(this.resizeTimeout);
      }
      this.resizeTimeout = setTimeout(() => {
        if (this.terminal && this.fitAddon) {
          // Force a proper fit by refreshing the terminal
          this.fitAddon.fit();
          // Trigger a second fit after DOM updates
          setTimeout(() => this.fitAddon.fit(), 10);
        }
      }, 16); // ~60fps
    };
    
    const handleMouseUp = () => {
      isResizing = false;
      container.classList.remove('resizing');
      
      // Clear any pending resize timeout
      if (this.resizeTimeout) {
        clearTimeout(this.resizeTimeout);
        this.resizeTimeout = null;
      }
      
      // Final terminal resize with multiple attempts for reliability
      if (this.terminal && this.fitAddon) {
        this.fitAddon.fit();
        setTimeout(() => this.fitAddon.fit(), 50);
        setTimeout(() => this.fitAddon.fit(), 100);
      }
      
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    // Minimize functionality
    const minimizeTerminal = () => {
      if (!this.isMinimized) {
        // Store current height before minimizing
        const currentHeight = parseInt(container.style.height || document.defaultView.getComputedStyle(container).height, 10);
        this.previousHeight = Math.max(currentHeight, 200); // Ensure reasonable minimum
        
        container.classList.add('minimized');
        container.style.height = '40px';
        document.body.style.setProperty('--terminal-height', '40px');
        
        minimizeBtn.style.display = 'none';
        maximizeBtn.style.display = 'inline-block';
        this.isMinimized = true;
      }
    };
    
    // Maximize functionality
    const maximizeTerminal = () => {
      if (this.isMinimized) {
        container.classList.remove('minimized');
        
        // Restore to previous height
        const restoreHeight = Math.min(this.previousHeight, window.innerHeight * 0.8);
        container.style.height = restoreHeight + 'px';
        document.body.style.setProperty('--terminal-height', restoreHeight + 'px');
        
        minimizeBtn.style.display = 'inline-block';
        maximizeBtn.style.display = 'none';
        this.isMinimized = false;
        
        // Resize terminal to fit restored container
        if (this.terminal && this.fitAddon) {
          setTimeout(() => {
            this.fitAddon.fit();
            this.terminal.focus();
          }, 100);
        }
      }
    };
    
    // Event listeners
    minimizeBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      minimizeTerminal();
    });
    
    maximizeBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      maximizeTerminal();
    });
    
    // Close button event listener
    if (closeBtn) {
      closeBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        this.close();
      });
    }
    
    // Double-click header to toggle minimize/maximize
    terminalHeader.addEventListener('dblclick', () => {
      if (this.isMinimized) {
        maximizeTerminal();
      } else {
        minimizeTerminal();
      }
    });
  }

  toggle() {
    const container = document.getElementById('terminal-container');
    if (container) {
      if (container.style.display === 'none') {
        this.show();
      } else {
        this.hide();
      }
    }
  }

  resize() {
    if (this.terminal && this.fitAddon) {
      this.fitAddon.fit();
    }
  }

  destroy() {
    if (this.sessionId) {
      // Send destroy message to server
      if (this.ws && this.ws.ws && this.ws.ws.readyState === WebSocket.OPEN) {
        this.ws.send({
          type: 'terminal',
          action: 'destroy',
          sessionId: this.sessionId
        });
      }
      this.sessionId = null;
      this.clearSessionId();
    }

    if (this.terminal) {
      this.terminal.dispose();
      this.terminal = null;
    }

    this.fitAddon = null;
    this.isInitialized = false;
    
    // Remove WebSocket message handler
    if (this.ws) {
      this.ws.removeMessageHandler('terminal', this.handleTerminalMessage);
    }
    
    // Scroll observer cleanup no longer needed
    
    if (this.debug) {
      console.log('Terminal destroyed');
    }
  }

  // Navigate to a specific exercise directory
  navigateToExercise(exercisePath) {
    if (this.sessionId && this.terminal) {
      // Clear current line and send cd command
      const cdCommand = `cd /workspaces/rust-tour/exercises/${exercisePath}\r`;
      this.ws.send({
        type: 'terminal',
        action: 'input',
        sessionId: this.sessionId,
        input: cdCommand
      });
    }
  }

  // Send a predefined command
  sendCommand(command) {
    if (this.sessionId && this.terminal) {
      this.ws.send({
        type: 'terminal',
        action: 'input',
        sessionId: this.sessionId,
        input: command + '\r'
      });
    }
  }

  // Session persistence methods
  loadSessionId() {
    try {
      return localStorage.getItem('rust-tour-terminal-session');
    } catch (error) {
      if (this.debug) {
        console.log('Could not load session ID from localStorage:', error);
      }
      return null;
    }
  }

  saveSessionId() {
    if (this.sessionId) {
      try {
        localStorage.setItem('rust-tour-terminal-session', this.sessionId);
      } catch (error) {
        if (this.debug) {
          console.log('Could not save session ID to localStorage:', error);
        }
      }
    }
  }

  clearSessionId() {
    try {
      localStorage.removeItem('rust-tour-terminal-session');
    } catch (error) {
      if (this.debug) {
        console.log('Could not clear session ID from localStorage:', error);
      }
    }
  }

  forceScrollToBottom() {
    if (!this.terminal) return;
    
    // Use a simple, reliable approach
    this.terminal.scrollToBottom();
    
    // Ensure viewport scrolls to bottom as well
    setTimeout(() => {
      const terminalContainer = document.getElementById('terminal');
      if (terminalContainer) {
        const viewport = terminalContainer.querySelector('.xterm-viewport');
        if (viewport) {
          viewport.scrollTop = viewport.scrollHeight;
        }
      }
    }, 0);
  }

  
  // Removed complex scroll observer method
}