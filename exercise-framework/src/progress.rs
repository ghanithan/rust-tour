use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// User's overall progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub overall_progress: f64, // 0.0 to 1.0
    pub chapters_completed: u32,
    pub exercises_completed: u32,
    pub total_exercises: u32,
    pub total_time_minutes: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub chapters: HashMap<u32, ChapterProgress>,
    pub exercise_history: Vec<ExerciseCompletion>,
    pub achievements: Vec<Achievement>,
    pub preferences: UserPreferences,
    pub analytics: LearningAnalytics,
}

/// Progress for a specific chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterProgress {
    pub chapter_number: u32,
    pub title: String,
    pub exercises_completed: u32,
    pub total_exercises: u32,
    pub completion_percentage: f64,
    pub time_spent_minutes: u32,
    pub first_started: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub concept_mastery: HashMap<String, ConceptMastery>,
    pub is_unlocked: bool,
}

/// Mastery level for individual concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMastery {
    pub concept: String,
    pub mastery_level: f64, // 0.0 to 1.0
    pub confidence_level: f64, // 0.0 to 1.0
    pub last_practiced: chrono::DateTime<chrono::Utc>,
    pub practice_count: u32,
    pub needs_review: bool,
}

/// Individual exercise completion record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseCompletion {
    pub exercise_id: String,
    pub chapter: u32,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub time_taken_minutes: u32,
    pub attempts: u32,
    pub hints_used: u32,
    pub test_passes: u32,
    pub test_failures: u32,
    pub code_quality_score: f64,
    pub concepts_learned: Vec<String>,
}

/// User achievements and badges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub earned_at: chrono::DateTime<chrono::Utc>,
    pub category: AchievementCategory,
    pub points: u32,
}

/// Categories of achievements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Progress,    // Chapter completions, streaks
    Mastery,     // Concept mastery, code quality
    Challenge,   // Performance, advanced exercises
    Community,   // Contributions, helping others
    Special,     // Holiday events, milestones
}

/// User preferences and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub difficulty_preference: DifficultyPreference,
    pub hint_usage: HintPreference,
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub auto_advance: bool,
    pub practice_reminders: bool,
}

/// Difficulty preference settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyPreference {
    Adaptive,    // System chooses based on performance
    Progressive, // Gradual increase in difficulty
    Consistent,  // User-selected fixed difficulty
    Challenge,   // Always prefer harder exercises
}

/// Hint system preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HintPreference {
    Minimal,     // Prefer conceptual hints only
    Progressive, // Use all three hint levels
    Detailed,    // Prefer implementation hints
    Disabled,    // No hints (hard mode)
}

/// Learning analytics and insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningAnalytics {
    pub learning_velocity: f64, // exercises per week
    pub average_session_time: f64, // minutes
    pub peak_learning_hours: Vec<u32>, // hours of day (0-23)
    pub concept_strengths: Vec<String>,
    pub concept_weaknesses: Vec<String>,
    pub error_patterns: HashMap<String, u32>,
    pub improvement_trends: HashMap<String, f64>,
    pub predicted_completion_time: u32, // days to full completion
}

/// Progress tracker manages user learning progress
pub struct ProgressTracker {
    progress_file: std::path::PathBuf,
    current_progress: UserProgress,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new<P: AsRef<Path>>(exercises_root: P) -> Result<Self> {
        let progress_file = exercises_root.as_ref()
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("progress")
            .join("user_progress.json");

        // Create progress directory if it doesn't exist
        if let Some(parent) = progress_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let current_progress = if progress_file.exists() {
            Self::load_progress(&progress_file)?
        } else {
            Self::create_default_progress()
        };

        Ok(Self {
            progress_file,
            current_progress,
        })
    }

    /// Load progress from file
    fn load_progress(path: &Path) -> Result<UserProgress> {
        let content = std::fs::read_to_string(path)?;
        let progress: UserProgress = serde_json::from_str(&content)?;
        Ok(progress)
    }

    /// Create default progress for new user
    fn create_default_progress() -> UserProgress {
        let now = chrono::Utc::now();
        
        UserProgress {
            user_id: "default".to_string(),
            created_at: now,
            last_updated: now,
            overall_progress: 0.0,
            chapters_completed: 0,
            exercises_completed: 0,
            total_exercises: 200, // Estimated total
            total_time_minutes: 0,
            current_streak: 0,
            longest_streak: 0,
            chapters: HashMap::new(),
            exercise_history: Vec::new(),
            achievements: Vec::new(),
            preferences: UserPreferences {
                difficulty_preference: DifficultyPreference::Adaptive,
                hint_usage: HintPreference::Progressive,
                theme: "rust".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
                auto_advance: false,
                practice_reminders: true,
            },
            analytics: LearningAnalytics {
                learning_velocity: 0.0,
                average_session_time: 0.0,
                peak_learning_hours: Vec::new(),
                concept_strengths: Vec::new(),
                concept_weaknesses: Vec::new(),
                error_patterns: HashMap::new(),
                improvement_trends: HashMap::new(),
                predicted_completion_time: 0,
            },
        }
    }

    /// Save current progress to file
    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.current_progress)?;
        std::fs::write(&self.progress_file, content)?;
        Ok(())
    }

    /// Get current progress
    pub fn get_progress(&self) -> Result<UserProgress> {
        Ok(self.current_progress.clone())
    }

    /// Complete an exercise and update progress
    pub fn complete_exercise(&mut self, exercise_id: &str, time_taken_minutes: u32) -> Result<()> {
        let now = chrono::Utc::now();

        // Parse chapter from exercise ID
        let chapter = exercise_id
            .strip_prefix("ch")
            .and_then(|s| s[..2].parse::<u32>().ok())
            .unwrap_or(1);

        // Create exercise completion record
        let completion = ExerciseCompletion {
            exercise_id: exercise_id.to_string(),
            chapter,
            completed_at: now,
            time_taken_minutes,
            attempts: 1, // TODO: Track actual attempts
            hints_used: 0, // TODO: Track actual hint usage
            test_passes: 1, // TODO: Track actual test results
            test_failures: 0,
            code_quality_score: 0.8, // TODO: Calculate from clippy/fmt
            concepts_learned: Vec::new(), // TODO: Get from exercise metadata
        };

        // Add to history
        self.current_progress.exercise_history.push(completion);

        // Update overall progress
        self.current_progress.exercises_completed += 1;
        self.current_progress.total_time_minutes += time_taken_minutes;
        self.current_progress.overall_progress = 
            self.current_progress.exercises_completed as f64 / 
            self.current_progress.total_exercises as f64;

        // Update chapter progress
        let chapter_progress = self.current_progress.chapters
            .entry(chapter)
            .or_insert_with(|| ChapterProgress {
                chapter_number: chapter,
                title: format!("Chapter {}", chapter),
                exercises_completed: 0,
                total_exercises: 10, // TODO: Get from metadata
                completion_percentage: 0.0,
                time_spent_minutes: 0,
                first_started: Some(now),
                completed_at: None,
                concept_mastery: HashMap::new(),
                is_unlocked: true,
            });

        chapter_progress.exercises_completed += 1;
        chapter_progress.time_spent_minutes += time_taken_minutes;
        chapter_progress.completion_percentage = 
            chapter_progress.exercises_completed as f64 / 
            chapter_progress.total_exercises as f64;

        // Check if chapter is complete
        if chapter_progress.exercises_completed >= chapter_progress.total_exercises {
            chapter_progress.completed_at = Some(now);
            self.current_progress.chapters_completed += 1;
        }

        // Update streak
        self.update_streak();

        // Check for new achievements
        self.check_achievements();

        // Update analytics
        self.update_analytics();

        // Update timestamp
        self.current_progress.last_updated = now;

        // Save progress
        self.save()?;

        Ok(())
    }

    /// Update learning streak
    fn update_streak(&mut self) {
        let now = chrono::Utc::now().date_naive();
        
        if let Some(last_completion) = self.current_progress.exercise_history.last() {
            let last_date = last_completion.completed_at.date_naive();
            let days_diff = (now - last_date).num_days();

            if days_diff <= 1 {
                // Continuing or maintaining streak
                if days_diff == 1 {
                    self.current_progress.current_streak += 1;
                }
                // days_diff == 0 means same day, don't increment
            } else {
                // Streak broken
                self.current_progress.current_streak = 1;
            }

            // Update longest streak
            if self.current_progress.current_streak > self.current_progress.longest_streak {
                self.current_progress.longest_streak = self.current_progress.current_streak;
            }
        }
    }

    /// Check for new achievements
    fn check_achievements(&mut self) {
        let mut new_achievements = Vec::new();

        // First exercise achievement
        if self.current_progress.exercises_completed == 1 {
            new_achievements.push(Achievement {
                id: "first_exercise".to_string(),
                title: "Hello, Rust!".to_string(),
                description: "Completed your first exercise".to_string(),
                icon: "🦀".to_string(),
                earned_at: chrono::Utc::now(),
                category: AchievementCategory::Progress,
                points: 10,
            });
        }

        // Streak achievements
        if self.current_progress.current_streak == 7 {
            new_achievements.push(Achievement {
                id: "week_streak".to_string(),
                title: "Consistent Learner".to_string(),
                description: "Maintained a 7-day learning streak".to_string(),
                icon: "🔥".to_string(),
                earned_at: chrono::Utc::now(),
                category: AchievementCategory::Progress,
                points: 25,
            });
        }

        // Add new achievements to the list
        self.current_progress.achievements.extend(new_achievements);
    }

    /// Update learning analytics
    fn update_analytics(&mut self) {
        let analytics = &mut self.current_progress.analytics;

        // Calculate learning velocity (exercises per week)
        if !self.current_progress.exercise_history.is_empty() {
            let first_exercise = self.current_progress.exercise_history.first().unwrap();
            let days_learning = (chrono::Utc::now() - first_exercise.completed_at).num_days();
            
            if days_learning > 0 {
                analytics.learning_velocity = 
                    (self.current_progress.exercises_completed as f64 * 7.0) / days_learning as f64;
            }
        }

        // Calculate average session time
        if !self.current_progress.exercise_history.is_empty() {
            analytics.average_session_time = 
                self.current_progress.total_time_minutes as f64 / 
                self.current_progress.exercise_history.len() as f64;
        }

        // Update predicted completion time
        if analytics.learning_velocity > 0.0 {
            let remaining_exercises = 
                self.current_progress.total_exercises - self.current_progress.exercises_completed;
            analytics.predicted_completion_time = 
                (remaining_exercises as f64 / analytics.learning_velocity * 7.0) as u32;
        }
    }

    /// Get next recommended exercises
    pub fn get_recommendations(&self, limit: usize) -> Result<Vec<String>> {
        // TODO: Implement sophisticated recommendation algorithm
        // For now, return simple sequential progression
        let next_chapter = self.current_progress.chapters_completed + 1;
        Ok(vec![format!("ch{:02}-ex01", next_chapter)])
    }

    /// Check if exercise is unlocked for user
    pub fn is_exercise_unlocked(&self, exercise_id: &str) -> bool {
        // TODO: Implement based on prerequisites and current progress
        // For now, allow first 3 chapters
        let chapter = exercise_id
            .strip_prefix("ch")
            .and_then(|s| s[..2].parse::<u32>().ok())
            .unwrap_or(1);
        
        chapter <= 3 || chapter <= self.current_progress.chapters_completed + 2
    }
}