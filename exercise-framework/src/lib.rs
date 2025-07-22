pub mod exercise;
pub mod metadata;
pub mod progress;
pub mod testing;
pub mod validation;
pub mod hints;

// Re-export main types
pub use exercise::{Exercise, ExerciseType, ExerciseDifficulty};
pub use metadata::ExerciseMetadata;
pub use progress::{ProgressTracker, UserProgress};
pub use testing::{TestRunner, TestResult};
pub use validation::{ExerciseValidator, ValidationResult};
pub use hints::{HintSystem, HintLevel};

use anyhow::Result;
use std::path::Path;

/// Main entry point for the exercise framework
pub struct Framework {
    exercises_root: std::path::PathBuf,
    progress_tracker: ProgressTracker,
    test_runner: TestRunner,
}

impl Framework {
    /// Create a new framework instance
    pub fn new<P: AsRef<Path>>(exercises_root: P) -> Result<Self> {
        let exercises_root = exercises_root.as_ref().to_path_buf();
        
        Ok(Self {
            exercises_root: exercises_root.clone(),
            progress_tracker: ProgressTracker::new(&exercises_root)?,
            test_runner: TestRunner::new(),
        })
    }

    /// Load an exercise by ID
    pub fn load_exercise(&self, exercise_id: &str) -> Result<Exercise> {
        Exercise::load(&self.exercises_root, exercise_id)
    }

    /// List all available exercises
    pub fn list_exercises(&self) -> Result<Vec<ExerciseMetadata>> {
        Exercise::list_all(&self.exercises_root)
    }

    /// Get exercises for a specific chapter
    pub fn list_chapter_exercises(&self, chapter: u32) -> Result<Vec<ExerciseMetadata>> {
        self.list_exercises()?
            .into_iter()
            .filter(|ex| ex.chapter == chapter)
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    /// Run tests for an exercise
    pub fn test_exercise(&self, exercise_id: &str) -> Result<TestResult> {
        let exercise = self.load_exercise(exercise_id)?;
        self.test_runner.run_tests(&exercise)
    }

    /// Validate an exercise for quality and correctness
    pub fn validate_exercise(&self, exercise_id: &str) -> Result<ValidationResult> {
        let exercise = self.load_exercise(exercise_id)?;
        let validator = ExerciseValidator::new();
        validator.validate(&exercise)
    }

    /// Get user's progress
    pub fn get_progress(&self) -> Result<UserProgress> {
        self.progress_tracker.get_progress()
    }

    /// Update progress for completed exercise
    pub fn complete_exercise(&mut self, exercise_id: &str, time_taken_minutes: u32) -> Result<()> {
        self.progress_tracker.complete_exercise(exercise_id, time_taken_minutes)
    }
}

// Helper trait for functional programming style
trait Pipe<T> {
    fn pipe<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R;
}

impl<T> Pipe<T> for T {
    fn pipe<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}