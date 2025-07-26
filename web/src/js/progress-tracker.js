export class ProgressTracker {
  constructor() {
    this.progress = null;
    this.sessionStartTime = Date.now();
    this.currentExerciseStartTime = null;
  }

  async init(exercises = null) {
    await this.loadProgress();
    
    // Set total exercises based on actual loaded exercises
    if (exercises && Array.isArray(exercises)) {
      this.setTotalExercises(exercises.length);
    }
    
    console.log('Progress tracker initialized');
  }
  
  setTotalExercises(count) {
    if (this.progress) {
      this.progress.total_exercises = count;
      this.updateProgressDisplay();
    }
  }

  async loadProgress() {
    try {
      const response = await fetch('/api/progress');
      if (response.ok) {
        this.progress = await response.json();
      } else {
        this.progress = this.getDefaultProgress();
      }
      
      this.updateProgressDisplay();
    } catch (error) {
      console.error('Failed to load progress:', error);
      this.progress = this.getDefaultProgress();
    }
  }

  getDefaultProgress() {
    return {
      user_id: 'default',
      created_at: new Date().toISOString(),
      overall_progress: 0.0,
      chapters_completed: 0,
      exercises_completed: 0,
      total_exercises: 200,
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
  }

  async trackExerciseViewed(exerciseId) {
    // Ensure progress is loaded
    if (!this.progress) {
      await this.loadProgress();
    }
    
    this.currentExerciseStartTime = Date.now();
    
    try {
      const response = await fetch('/api/progress/view', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          exercise_id: exerciseId
        })
      });
      
      if (response.ok) {
        const result = await response.json();
        this.progress = result.progress;
        this.updateProgressDisplay();
        console.log(`Exercise view tracked: ${exerciseId}`);
      } else {
        console.error('Failed to track exercise view');
        // Fallback to local tracking
        this.progress.session_stats.exercises_viewed += 1;
        this.updateProgressDisplay();
      }
    } catch (error) {
      console.error('Error tracking exercise view:', error);
      // Fallback to local tracking
      if (!this.progress.session_stats) {
        this.progress.session_stats = {
          exercises_viewed: 0,
          exercises_completed: 0,
          hints_used: 0,
          time_spent: 0
        };
      }
      this.progress.session_stats.exercises_viewed += 1;
      this.updateProgressDisplay();
    }
  }

  async trackHintUsed(exerciseId, level) {
    if (!this.progress) {
      await this.loadProgress();
    }
    
    try {
      const response = await fetch('/api/progress/hint', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          exercise_id: exerciseId,
          hint_level: level
        })
      });
      
      if (response.ok) {
        const result = await response.json();
        this.progress = result.progress;
        this.updateProgressDisplay();
        console.log(`Hint tracked: ${exerciseId}, level ${level}`);
      } else {
        console.error('Failed to track hint usage');
      }
    } catch (error) {
      console.error('Error tracking hint usage:', error);
    }
  }

  async completeExercise(exerciseId, timeSpentMinutes) {
    if (!this.progress) {
      await this.loadProgress();
    }
    
    // Check if already completed to avoid duplicates
    if (this.isExerciseCompleted(exerciseId)) {
      console.log(`Exercise ${exerciseId} already completed`);
      return;
    }
    
    try {
      // Ensure exercise_history exists
      if (!this.progress.exercise_history) {
        this.progress.exercise_history = [];
      }
      
      // Update local progress
      this.progress.exercises_completed += 1;
      this.progress.session_stats.exercises_completed += 1;
      this.progress.total_time_minutes += timeSpentMinutes;
      
      // Calculate overall progress
      this.progress.overall_progress = 
        this.progress.exercises_completed / this.progress.total_exercises;

      // Add to exercise history
      this.progress.exercise_history.push({
        exercise_id: exerciseId,
        completed_at: new Date().toISOString(),
        time_taken_minutes: timeSpentMinutes,
        session_id: this.getSessionId()
      });

      // Update backend
      const response = await fetch('/api/progress/complete', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          exercise_id: exerciseId,
          time_taken_minutes: timeSpentMinutes
        })
      });

      if (!response.ok) {
        console.error('Failed to update progress on backend');
      }

      // Check for achievements
      this.checkForAchievements();
      
      // Update UI
      this.updateProgressDisplay();
      
    } catch (error) {
      console.error('Failed to complete exercise:', error);
    }
  }

  checkForAchievements() {
    if (!this.progress) return;
    
    // Ensure achievements array exists
    if (!this.progress.achievements) {
      this.progress.achievements = [];
    }
    
    const achievements = this.progress.achievements;
    const completed = this.progress.exercises_completed;
    
    // First exercise achievement
    if (completed === 1 && !achievements.find(a => a.id === 'first_exercise')) {
      this.unlockAchievement({
        id: 'first_exercise',
        title: 'Hello, Rust! 🦀',
        description: 'Completed your first exercise',
        icon: '🎉',
        points: 10,
        unlocked_at: new Date().toISOString()
      });
    }

    // Chapter completion achievements
    if (!this.progress.chapters) {
      this.progress.chapters = {};
    }
    const chaptersCompleted = Object.keys(this.progress.chapters).length;
    if (chaptersCompleted === 1 && !achievements.find(a => a.id === 'first_chapter')) {
      this.unlockAchievement({
        id: 'first_chapter',
        title: 'Chapter Master',
        description: 'Completed your first chapter',
        icon: '📚',
        points: 25,
        unlocked_at: new Date().toISOString()
      });
    }

    // Session achievements
    if (this.progress.session_stats.exercises_completed === 3 && 
        !achievements.find(a => a.id === 'session_streak')) {
      this.unlockAchievement({
        id: 'session_streak',
        title: 'Momentum Builder',
        description: 'Completed 3 exercises in one session',
        icon: '🔥',
        points: 15,
        unlocked_at: new Date().toISOString()
      });
    }
  }

  unlockAchievement(achievement) {
    this.progress.achievements.push(achievement);
    
    // Show achievement notification
    this.showAchievementNotification(achievement);
    
    console.log('Achievement unlocked:', achievement);
  }

  showAchievementNotification(achievement) {
    // Create achievement notification
    const notification = document.createElement('div');
    notification.className = 'achievement-notification';
    notification.innerHTML = `
      <div class="achievement-content">
        <div class="achievement-icon">${achievement.icon}</div>
        <div class="achievement-info">
          <div class="achievement-title">${achievement.title}</div>
          <div class="achievement-description">${achievement.description}</div>
          <div class="achievement-points">+${achievement.points} points</div>
        </div>
      </div>
    `;
    
    // Add styles
    notification.style.cssText = `
      position: fixed;
      top: 80px;
      right: 20px;
      background: linear-gradient(135deg, var(--rust-orange), var(--rust-orange-light));
      color: white;
      padding: 15px;
      border-radius: 8px;
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
      z-index: 1001;
      min-width: 300px;
      opacity: 0;
      transform: translateX(100%);
      transition: all 0.5s ease;
    `;

    document.body.appendChild(notification);
    
    // Show with animation
    setTimeout(() => {
      notification.style.opacity = '1';
      notification.style.transform = 'translateX(0)';
    }, 100);
    
    // Hide after 5 seconds
    setTimeout(() => {
      notification.style.opacity = '0';
      notification.style.transform = 'translateX(100%)';
      setTimeout(() => document.body.removeChild(notification), 500);
    }, 5000);
  }

  getTimeSpentOnCurrentExercise() {
    if (!this.currentExerciseStartTime) return 0;
    return Math.round((Date.now() - this.currentExerciseStartTime) / (1000 * 60)); // minutes
  }

  getSessionId() {
    // Simple session ID based on start time
    return `session_${this.sessionStartTime}`;
  }

  updateProgressDisplay() {
    if (!this.progress) return;
    
    const progressText = document.getElementById('progress-text');
    if (progressText) {
      const completed = this.progress.exercises_completed || 0;
      const total = this.progress.total_exercises || 0;
      const percentage = total > 0 ? Math.round((completed / total) * 100) : 0;
      
      if (total > 0) {
        progressText.textContent = `${percentage}% Complete (${completed}/${total})`;
      } else {
        progressText.textContent = 'Loading exercises...';
      }
    }
  }

  getProgress() {
    return this.progress;
  }

  isExerciseCompleted(exerciseId) {
    if (!this.progress || !this.progress.exercise_history) return false;
    return this.progress.exercise_history.some(entry => entry.exercise_id === exerciseId);
  }

  getSessionStats() {
    if (!this.progress) return null;
    
    return {
      ...this.progress.session_stats,
      session_duration: Math.round((Date.now() - this.sessionStartTime) / (1000 * 60)) // minutes
    };
  }

  getChapterProgress(chapterNum) {
    if (!this.progress) return {
      completed: false,
      exercises_completed: 0,
      total_exercises: 0,
      time_spent: 0
    };
    
    return this.progress.chapters[chapterNum] || {
      completed: false,
      exercises_completed: 0,
      total_exercises: 0,
      time_spent: 0
    };
  }

  getRecentAchievements(limit = 5) {
    if (!this.progress) return [];
    
    return this.progress.achievements
      .sort((a, b) => new Date(b.unlocked_at) - new Date(a.unlocked_at))
      .slice(0, limit);
  }

  exportProgress() {
    const data = {
      ...this.progress,
      exported_at: new Date().toISOString(),
      platform_version: '0.1.0'
    };
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { 
      type: 'application/json' 
    });
    
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `rust-learning-progress-${Date.now()}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }
}