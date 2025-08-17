export class WebSocketManager {
  constructor() {
    this.ws = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 1000; // Start with 1 second
    this.messageHandlers = [];
    this.typedMessageHandlers = new Map();
    this.isConnecting = false;
    this.debug = localStorage.getItem('DEBUG_WEBSOCKET') === 'true' || window.location.search.includes('debug=true');
  }

  init(messageHandler) {
    if (messageHandler) {
      this.messageHandlers.push(messageHandler);
    }
    
    this.connect();
    console.log('WebSocket manager initialized');
  }

  connect() {
    if (this.isConnecting || (this.ws && this.ws.readyState === WebSocket.CONNECTING)) {
      return;
    }

    this.isConnecting = true;
    
    try {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      let wsUrl;
      
      // Check if we're in GitHub Codespaces or similar environment
      if (window.location.hostname.includes('github.dev') || window.location.hostname.includes('gitpod.io')) {
        // In Codespaces, connect directly to port 3000 for WebSocket (where the server is)
        const port3000Host = window.location.hostname.replace('-8000.', '-3000.');
        wsUrl = `${protocol}//${port3000Host}/ws`;
      } else {
        // Local development - connect directly to port 3000
        wsUrl = `${protocol}//${window.location.hostname}:3000/ws`;
      }
      
      if (this.debug) {
        console.log(`Connecting to WebSocket: ${wsUrl}`);
      }
      this.ws = new WebSocket(wsUrl);
      
      this.ws.onopen = () => {
        console.log('✅ WebSocket connected');
        this.reconnectAttempts = 0;
        this.reconnectDelay = 1000;
        this.isConnecting = false;
        
        this.notifyConnectionStatus(true);
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          this.handleMessage(data);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      this.ws.onclose = (event) => {
        console.log('❌ WebSocket disconnected:', event.code, event.reason);
        this.isConnecting = false;
        this.notifyConnectionStatus(false);
        
        // Attempt to reconnect unless it was a clean close
        if (event.code !== 1000 && this.reconnectAttempts < this.maxReconnectAttempts) {
          this.scheduleReconnect();
        }
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        this.isConnecting = false;
      };

    } catch (error) {
      console.error('Failed to create WebSocket connection:', error);
      this.isConnecting = false;
      this.scheduleReconnect();
    }
  }

  scheduleReconnect() {
    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1); // Exponential backoff
    
    console.log(`Scheduling reconnect attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts} in ${delay}ms`);
    
    setTimeout(() => {
      if (this.reconnectAttempts <= this.maxReconnectAttempts) {
        this.connect();
      } else {
        console.warn('Max reconnection attempts reached. WebSocket functionality disabled.');
        this.showConnectionError();
      }
    }, delay);
  }

  handleMessage(data) {
    // Only log errors and important messages, not every single message
    if (this.debug && (data.type === 'error' || data.type === 'connection')) {
      console.log('WebSocket message:', data);
    }
    
    // Dispatch to all registered handlers
    this.messageHandlers.forEach(handler => {
      try {
        handler(data);
      } catch (error) {
        console.error('Error in WebSocket message handler:', error);
      }
    });

    // Handle typed message handlers first
    const typedHandler = this.typedMessageHandlers.get(data.type);
    if (typedHandler) {
      typedHandler(data);
      return; // Don't continue to built-in handlers
    }

    // Handle built-in message types
    switch (data.type) {
      case 'file_updated':
        this.handleFileUpdated(data);
        break;
      case 'file_changed':
        this.handleFileChanged(data);
        break;
      case 'exercise_validation':
        this.handleExerciseValidation(data);
        break;
      case 'system_notification':
        this.handleSystemNotification(data);
        break;
      case 'heartbeat_response':
        this.handleHeartbeatResponse(data);
        break;
      default:
        console.log('Unhandled message type:', data.type);
    }
  }

  handleFileUpdated(data) {
    // Show notification about file update with exercise name
    const displayName = data.exercise || data.file;
    // this.showNotification(`File updated: ${displayName}`, 'info');
    
    // Dispatch custom event
    document.dispatchEvent(new CustomEvent('file-updated', { 
      detail: { ...data, displayName } 
    }));
  }

  handleFileChanged(data) {
    // Handle file system changes
    const displayName = data.exercise || data.file;
    // Only log if debug mode is explicitly enabled and it's not a build artifact
    if (this.debug && !data.file?.includes('target/')) {
      console.log(`File changed: ${displayName}`);
    }
    
    // Dispatch custom event
    document.dispatchEvent(new CustomEvent('file-changed', { 
      detail: { ...data, displayName } 
    }));
  }

  handleExerciseValidation(data) {
    // Handle exercise validation results
    document.dispatchEvent(new CustomEvent('exercise-validation', { 
      detail: data 
    }));
  }

  handleSystemNotification(data) {
    this.showNotification(data.message, data.level || 'info');
  }

  handleHeartbeatResponse(data) {
    if (this.debug) {
      const latency = Date.now() - data.timestamp;
      console.log(`Heartbeat response received. Latency: ${latency}ms`);
    }
    
    // Update connection quality indicator or health metrics
    this.updateConnectionHealth(data);
  }

  updateConnectionHealth(data) {
    // Optional: Update UI to show connection latency/quality
    // Could be used for showing connection status indicators
    if (this.debug) {
      const latency = Date.now() - data.timestamp;
      console.log(`Connection health - Latency: ${latency}ms, Server time: ${data.server_time}`);
    }
  }

  send(data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      try {
        this.ws.send(JSON.stringify(data));
        return true;
      } catch (error) {
        console.error('Failed to send WebSocket message:', error);
        return false;
      }
    } else {
      console.warn('WebSocket not connected. Message not sent:', data);
      return false;
    }
  }

  sendExerciseView(exerciseId) {
    return this.send({
      type: 'exercise_view',
      exercise_id: exerciseId,
      timestamp: Date.now()
    });
  }

  sendCodeExecution(exerciseId, action, result) {
    return this.send({
      type: 'code_execution',
      exercise_id: exerciseId,
      action: action, // 'run', 'test', 'check'
      result: result,
      timestamp: Date.now()
    });
  }

  sendProgressUpdate(progressData) {
    return this.send({
      type: 'progress_update',
      progress: progressData,
      timestamp: Date.now()
    });
  }

  sendHeartbeat() {
    return this.send({
      type: 'heartbeat',
      timestamp: Date.now()
    });
  }

  notifyConnectionStatus(connected) {
    const statusIndicator = document.querySelector('.connection-status');
    if (statusIndicator) {
      statusIndicator.className = `connection-status ${connected ? 'connected' : 'disconnected'}`;
      statusIndicator.title = connected ? 'Connected to server' : 'Disconnected from server';
    }

    // Dispatch connection status event
    document.dispatchEvent(new CustomEvent('websocket-status', { 
      detail: { connected } 
    }));

    if (connected) {
      this.startHeartbeat();
    } else {
      this.stopHeartbeat();
    }
  }

  startHeartbeat() {
    // Send heartbeat every 30 seconds
    this.heartbeatInterval = setInterval(() => {
      this.sendHeartbeat();
    }, 30000);
  }

  stopHeartbeat() {
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
  }

  showNotification(message, type = 'info') {
    // Create notification element
    const notification = document.createElement('div');
    notification.className = `ws-notification ${type}`;
    notification.innerHTML = `
      <div class="ws-notification-content">
        <span class="ws-notification-icon">${this.getNotificationIcon(type)}</span>
        <span class="ws-notification-message">${message}</span>
        <button class="ws-notification-close">&times;</button>
      </div>
    `;

    // Add styles if not already present
    if (!document.querySelector('#ws-notification-styles')) {
      const styles = document.createElement('style');
      styles.id = 'ws-notification-styles';
      styles.textContent = `
        .ws-notification {
          position: fixed;
          top: 80px;
          right: 20px;
          background: var(--bg-secondary);
          border: 1px solid var(--border-primary);
          border-radius: 6px;
          padding: 12px;
          min-width: 300px;
          max-width: 400px;
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
          z-index: 1000;
          opacity: 0;
          transform: translateX(100%);
          transition: all 0.3s ease;
        }
        
        .ws-notification.visible {
          opacity: 1;
          transform: translateX(0);
        }
        
        .ws-notification.info { border-left: 4px solid #2196f3; }
        .ws-notification.success { border-left: 4px solid #4caf50; }
        .ws-notification.warning { border-left: 4px solid #ff9800; }
        .ws-notification.error { border-left: 4px solid #f44336; }
        
        .ws-notification-content {
          display: flex;
          align-items: center;
          gap: 10px;
        }
        
        .ws-notification-icon {
          font-size: 16px;
          flex-shrink: 0;
        }
        
        .ws-notification-message {
          flex: 1;
          font-size: 14px;
          color: var(--text-primary);
        }
        
        .ws-notification-close {
          background: none;
          border: none;
          color: var(--text-muted);
          cursor: pointer;
          font-size: 18px;
          padding: 0;
          width: 20px;
          height: 20px;
          display: flex;
          align-items: center;
          justify-content: center;
        }
        
        .ws-notification-close:hover {
          color: var(--text-primary);
        }
      `;
      document.head.appendChild(styles);
    }

    // Add close functionality
    notification.querySelector('.ws-notification-close').addEventListener('click', () => {
      this.hideNotification(notification);
    });

    // Add to DOM and show
    document.body.appendChild(notification);
    
    // Trigger animation
    setTimeout(() => notification.classList.add('visible'), 100);
    
    // Auto-hide after 5 seconds
    setTimeout(() => {
      if (notification.parentNode) {
        this.hideNotification(notification);
      }
    }, 5000);
  }

  hideNotification(notification) {
    notification.classList.remove('visible');
    setTimeout(() => {
      if (notification.parentNode) {
        document.body.removeChild(notification);
      }
    }, 300);
  }

  getNotificationIcon(type) {
    const icons = {
      info: '<i class="fas fa-info-circle"></i>',
      success: '<i class="fas fa-check-circle"></i>',
      warning: '<i class="fas fa-exclamation-triangle"></i>',
      error: '<i class="fas fa-times-circle"></i>'
    };
    return icons[type] || icons.info;
  }

  showConnectionError() {
    this.showNotification(
      'Lost connection to server. Some features may not work properly. Please refresh the page.',
      'error'
    );
  }

  addMessageHandler(handler) {
    this.messageHandlers.push(handler);
  }

  removeMessageHandler(handler) {
    const index = this.messageHandlers.indexOf(handler);
    if (index > -1) {
      this.messageHandlers.splice(index, 1);
    }
  }

  getConnectionState() {
    if (!this.ws) return 'disconnected';
    
    switch (this.ws.readyState) {
      case WebSocket.CONNECTING: return 'connecting';
      case WebSocket.OPEN: return 'connected';
      case WebSocket.CLOSING: return 'closing';
      case WebSocket.CLOSED: return 'disconnected';
      default: return 'unknown';
    }
  }

  disconnect() {
    this.stopHeartbeat();
    
    if (this.ws) {
      this.ws.close(1000, 'User initiated disconnect');
      this.ws = null;
    }
  }

  // Add typed message handler
  addMessageHandler(type, handler) {
    this.typedMessageHandlers.set(type, handler);
  }

  // Remove typed message handler
  removeMessageHandler(type) {
    this.typedMessageHandlers.delete(type);
  }

}