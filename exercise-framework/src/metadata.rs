use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Exercise metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseMetadata {
    pub id: String,
    pub title: String,
    pub description: String,
    pub chapter: u32,
    pub exercise_number: u32,
    pub difficulty: String,
    pub estimated_time_minutes: u32,
    pub concepts: Vec<String>,
    pub prerequisites: Vec<String>,
    pub exercise_type: String,
    pub rust_book_refs: RustBookRefs,
    pub hints: HintConfig,
    pub testing: TestConfig,
    pub validation: ValidationConfig,
}

/// Rust Book integration references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustBookRefs {
    pub primary_chapter: String,
    pub supporting_chapters: Vec<String>,
    pub specific_sections: Vec<BookSection>,
}

/// Individual book section reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSection {
    pub chapter: String,
    pub title: String,
    pub url: String,
    pub relevance: String, // "core_concept", "supporting", "advanced"
}

/// Hint system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintConfig {
    pub available: u32,
    pub auto_unlock: bool,
    pub custom_hints: Option<Vec<String>>,
}

/// Test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub timeout_seconds: u32,
    pub memory_limit_mb: u32,
    pub allow_std_only: bool,
    pub custom_checks: Vec<String>,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub clippy_level: String, // "warn", "deny"
    pub format_required: bool,
    pub custom_checks: Vec<String>,
    pub performance_requirements: Option<PerformanceRequirements>,
}

/// Performance requirements for optimization exercises
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_execution_time_ms: u32,
    pub max_memory_usage_mb: u32,
    pub benchmark_targets: HashMap<String, BenchmarkTarget>,
}

/// Individual benchmark target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTarget {
    pub target_time_ns: u64,
    pub target_memory_bytes: u64,
    pub tolerance_percent: f64,
}

impl ExerciseMetadata {
    /// Load metadata from JSON file
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let metadata: Self = serde_json::from_str(&content)?;
        metadata.validate()?;
        Ok(metadata)
    }

    /// Validate metadata for consistency and completeness
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate ID format: chXX-exYY-name
        if !self.id.starts_with(&format!("ch{:02}", self.chapter)) {
            anyhow::bail!("Exercise ID must start with ch{:02}", self.chapter);
        }

        // Validate difficulty
        if !["beginner", "intermediate", "advanced"].contains(&self.difficulty.as_str()) {
            anyhow::bail!("Difficulty must be beginner, intermediate, or advanced");
        }

        // Validate exercise type
        let valid_types = [
            "code_completion",
            "bug_fixing",
            "from_scratch",
            "code_review",
            "performance",
        ];
        if !valid_types.contains(&self.exercise_type.as_str()) {
            anyhow::bail!("Invalid exercise type: {}", self.exercise_type);
        }

        // Validate estimated time (reasonable bounds)
        if self.estimated_time_minutes == 0 || self.estimated_time_minutes > 180 {
            anyhow::bail!("Estimated time must be between 1 and 180 minutes");
        }

        // Validate concepts are not empty
        if self.concepts.is_empty() {
            anyhow::bail!("Exercise must teach at least one concept");
        }

        // Validate Rust Book references
        if self.rust_book_refs.primary_chapter.is_empty() {
            anyhow::bail!("Primary Rust Book chapter reference required");
        }

        Ok(())
    }

    /// Get the difficulty as an enum
    pub fn difficulty_enum(&self) -> super::ExerciseDifficulty {
        match self.difficulty.as_str() {
            "beginner" => super::ExerciseDifficulty::Beginner,
            "intermediate" => super::ExerciseDifficulty::Intermediate,
            "advanced" => super::ExerciseDifficulty::Advanced,
            _ => super::ExerciseDifficulty::Beginner, // Default fallback
        }
    }

    /// Get the exercise type as an enum
    pub fn exercise_type_enum(&self) -> super::ExerciseType {
        match self.exercise_type.as_str() {
            "code_completion" => super::ExerciseType::CodeCompletion,
            "bug_fixing" => super::ExerciseType::BugFixing,
            "from_scratch" => super::ExerciseType::FromScratch,
            "code_review" => super::ExerciseType::CodeReview,
            "performance" => super::ExerciseType::Performance,
            _ => super::ExerciseType::FromScratch, // Default fallback
        }
    }

    /// Check if exercise has prerequisites
    pub fn has_prerequisites(&self) -> bool {
        !self.prerequisites.is_empty()
    }

    /// Get prerequisite chapter numbers
    pub fn prerequisite_chapters(&self) -> Vec<u32> {
        self.prerequisites
            .iter()
            .filter_map(|prereq| prereq.strip_prefix("ch").and_then(|s| s[..2].parse().ok()))
            .collect()
    }

    /// Get missing prerequisites for guidance (not enforcement)
    pub fn get_missing_prerequisites(&self, completed_exercises: &[String]) -> Vec<String> {
        if !self.has_prerequisites() {
            return Vec::new();
        }

        self.prerequisites
            .iter()
            .filter(|prereq| !completed_exercises.contains(prereq))
            .cloned()
            .collect()
    }

    /// Check if exercise has all prerequisites completed (for recommendation scoring)
    pub fn prerequisites_completed(&self, completed_exercises: &[String]) -> bool {
        if !self.has_prerequisites() {
            return true;
        }

        self.prerequisites
            .iter()
            .all(|prereq| completed_exercises.contains(prereq))
    }

    /// Get primary Rust Book URL
    pub fn primary_book_url(&self) -> String {
        format!(
            "https://doc.rust-lang.org/book/ch{}.html",
            self.rust_book_refs.primary_chapter
        )
    }

    /// Estimate points value based on difficulty and concepts
    pub fn point_value(&self) -> u32 {
        let base_points = match self.difficulty.as_str() {
            "beginner" => 10,
            "intermediate" => 25,
            "advanced" => 50,
            _ => 10,
        };

        let concept_bonus = self.concepts.len() as u32 * 5;
        let time_factor = (self.estimated_time_minutes / 10).max(1);

        base_points + concept_bonus + time_factor
    }
}
