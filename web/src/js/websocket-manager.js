export class WebSocketManager {
  constructor() {
    this.ws = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 1000; // Start with 1 second
    this.messageHandlers = [];
    this.isConnecting = false;
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
      const wsUrl = `${protocol}//${window.location.hostname}:8080`;
      
      console.log(`Connecting to WebSocket: ${wsUrl}`);
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
    console.log('Received WebSocket message:', data);
    
    // Dispatch to all registered handlers
    this.messageHandlers.forEach(handler => {
      try {
        handler(data);
      } catch (error) {
        console.error('Error in WebSocket message handler:', error);
      }
    });

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
      default:
        console.log('Unhandled message type:', data.type);
    }
  }

  handleFileUpdated(data) {
    // Show notification about external file update
    this.showNotification(`File updated: ${data.file}`, 'info');
    
    // Dispatch custom event
    document.dispatchEvent(new CustomEvent('file-updated', { 
      detail: data 
    }));
  }

  handleFileChanged(data) {
    // Handle file system changes
    console.log(`File changed: ${data.file}`);
    
    // Dispatch custom event
    document.dispatchEvent(new CustomEvent('file-changed', { 
      detail: data 
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
      info: 'ℹ️',
      success: '✅',
      warning: '⚠️',
      error: '❌'
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
}