export class ProgressTracker {
  constructor() {
    this.progress = null;
    this.sessionStartTime = Date.now();
    this.currentExerciseStartTime = null;
  }

  async init() {
    await this.loadProgress();
    console.log('Progress tracker initialized');
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
    this.currentExerciseStartTime = Date.now();
    this.progress.session_stats.exercises_viewed += 1;
    
    // Update UI
    this.updateProgressDisplay();
  }

  async trackHintUsed(exerciseId, level) {
    this.progress.session_stats.hints_used += 1;
    
    // In a real implementation, this would be saved to the backend
    console.log(`Hint used: ${exerciseId}, level ${level}`);
  }

  async completeExercise(exerciseId, timeSpentMinutes) {
    try {
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
    const progressText = document.getElementById('progress-text');
    if (progressText) {
      const percentage = Math.round(this.progress.overall_progress * 100);
      progressText.textContent = `${percentage}% Complete (${this.progress.exercises_completed}/${this.progress.total_exercises})`;
    }
  }

  getProgress() {
    return this.progress;
  }

  getSessionStats() {
    return {
      ...this.progress.session_stats,
      session_duration: Math.round((Date.now() - this.sessionStartTime) / (1000 * 60)) // minutes
    };
  }

  getChapterProgress(chapterNum) {
    return this.progress.chapters[chapterNum] || {
      completed: false,
      exercises_completed: 0,
      total_exercises: 0,
      time_spent: 0
    };
  }

  getRecentAchievements(limit = 5) {
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