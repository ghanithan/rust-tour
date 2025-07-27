use crate::metadata::ExerciseMetadata;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Exercise difficulty levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// Types of exercises available
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseType {
    CodeCompletion, // Fill in the blanks
    BugFixing,      // Fix intentional errors
    FromScratch,    // Complete implementation
    CodeReview,     // Improve existing code
    Performance,    // Optimization challenge
}

/// Main exercise structure
#[derive(Debug, Clone)]
pub struct Exercise {
    pub metadata: ExerciseMetadata,
    pub path: PathBuf,
    pub source_files: Vec<SourceFile>,
    pub test_files: Vec<TestFile>,
    pub hints: Vec<String>,
    pub solutions: Vec<Solution>,
}

/// Source file within an exercise
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
    pub is_template: bool,
}

/// Test file for validation
#[derive(Debug, Clone)]
pub struct TestFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
    pub test_type: TestType,
}

/// Types of test files
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestType {
    Unit,        // Unit tests for functionality
    Integration, // Integration tests
    Quality,     // Code quality checks
    Performance, // Benchmark tests
}

/// Solution with explanation
#[derive(Debug, Clone)]
pub struct Solution {
    pub name: String,
    pub content: String,
    pub explanation: String,
    pub is_primary: bool,
}

impl Exercise {
    /// Load an exercise from the filesystem
    pub fn load<P: AsRef<Path>>(exercises_root: P, exercise_id: &str) -> Result<Self> {
        let exercise_path = Self::find_exercise_path(&exercises_root, exercise_id)?;

        // Load metadata
        let metadata_path = exercise_path.join("metadata.json");
        let metadata = ExerciseMetadata::load_from_file(&metadata_path)
            .context("Failed to load exercise metadata")?;

        // Load source files
        let source_files = Self::load_source_files(&exercise_path)?;

        // Load test files
        let test_files = Self::load_test_files(&exercise_path)?;

        // Load hints
        let hints = Self::load_hints(&exercise_path)?;

        // Load solutions
        let solutions = Self::load_solutions(&exercise_path)?;

        Ok(Exercise {
            metadata,
            path: exercise_path,
            source_files,
            test_files,
            hints,
            solutions,
        })
    }

    /// Find the path to an exercise by ID
    fn find_exercise_path<P: AsRef<Path>>(exercises_root: P, exercise_id: &str) -> Result<PathBuf> {
        // Parse chapter from exercise ID (e.g., "ch03-ex02-variables" -> chapter 3)
        let chapter_num = exercise_id
            .strip_prefix("ch")
            .and_then(|s| s[..2].parse::<u32>().ok())
            .context("Invalid exercise ID format")?;

        // Look for chapter directory
        let chapter_pattern = format!("ch{:02}_", chapter_num);
        let exercises_root = exercises_root.as_ref();

        for entry in std::fs::read_dir(exercises_root)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();

            if name.starts_with(&chapter_pattern) && entry.file_type()?.is_dir() {
                // Look for exercise within chapter
                let chapter_path = entry.path();
                for exercise_entry in std::fs::read_dir(&chapter_path)? {
                    let exercise_entry = exercise_entry?;
                    let _exercise_name = exercise_entry.file_name().to_string_lossy();

                    // Check if this exercise matches the ID
                    let exercise_path = exercise_entry.path();
                    let metadata_path = exercise_path.join("metadata.json");

                    if metadata_path.exists() {
                        if let Ok(metadata) = ExerciseMetadata::load_from_file(&metadata_path) {
                            if metadata.id == exercise_id {
                                return Ok(exercise_path);
                            }
                        }
                    }
                }
            }
        }

        anyhow::bail!("Exercise not found: {}", exercise_id);
    }

    /// Load all source files for an exercise
    fn load_source_files(exercise_path: &Path) -> Result<Vec<SourceFile>> {
        let src_path = exercise_path.join("src");
        if !src_path.exists() {
            return Ok(vec![]);
        }

        let mut source_files = Vec::new();

        for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read source file: {:?}", path))?;

                let is_template = name.contains("_template")
                    || content.contains("// TODO:")
                    || content.contains("unimplemented!");

                source_files.push(SourceFile {
                    name,
                    path: path.to_path_buf(),
                    content,
                    is_template,
                });
            }
        }

        Ok(source_files)
    }

    /// Load all test files for an exercise
    fn load_test_files(exercise_path: &Path) -> Result<Vec<TestFile>> {
        let tests_path = exercise_path.join("tests");
        if !tests_path.exists() {
            return Ok(vec![]);
        }

        let mut test_files = Vec::new();

        for entry in WalkDir::new(&tests_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read test file: {:?}", path))?;

                let test_type = if name.contains("unit") {
                    TestType::Unit
                } else if name.contains("integration") {
                    TestType::Integration
                } else if name.contains("quality") {
                    TestType::Quality
                } else if name.contains("performance") || name.contains("bench") {
                    TestType::Performance
                } else {
                    TestType::Unit // Default
                };

                test_files.push(TestFile {
                    name,
                    path: path.to_path_buf(),
                    content,
                    test_type,
                });
            }
        }

        Ok(test_files)
    }

    /// Load hints from hints.md file
    fn load_hints(exercise_path: &Path) -> Result<Vec<String>> {
        let hints_path = exercise_path.join("src").join("hints.md");
        if !hints_path.exists() {
            return Ok(vec![]);
        }

        let content = std::fs::read_to_string(&hints_path).context("Failed to read hints file")?;

        // Parse hints by looking for level markers
        let mut hints = Vec::new();
        let mut current_hint = String::new();
        let mut in_hint = false;

        for line in content.lines() {
            if line.starts_with("## Level") {
                if in_hint && !current_hint.trim().is_empty() {
                    hints.push(current_hint.trim().to_string());
                }
                current_hint.clear();
                in_hint = true;
            } else if in_hint {
                current_hint.push_str(line);
                current_hint.push('\n');
            }
        }

        // Don't forget the last hint
        if in_hint && !current_hint.trim().is_empty() {
            hints.push(current_hint.trim().to_string());
        }

        Ok(hints)
    }

    /// Load solutions with explanations
    fn load_solutions(exercise_path: &Path) -> Result<Vec<Solution>> {
        let solutions_path = exercise_path.join("solutions");
        if !solutions_path.exists() {
            return Ok(vec![]);
        }

        let mut solutions = Vec::new();

        // Look for solution files
        for entry in std::fs::read_dir(&solutions_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let name = path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let content = std::fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read solution file: {:?}", path))?;

                // Look for explanation file
                let explanation_path = solutions_path.join(format!("{}.md", name));
                let explanation = if explanation_path.exists() {
                    std::fs::read_to_string(&explanation_path)
                        .unwrap_or_else(|_| "No explanation available.".to_string())
                } else {
                    "No explanation available.".to_string()
                };

                let is_primary = name == "reference" || name == "primary";

                solutions.push(Solution {
                    name,
                    content,
                    explanation,
                    is_primary,
                });
            }
        }

        // Sort solutions so primary comes first
        solutions.sort_by(|a, b| b.is_primary.cmp(&a.is_primary));

        Ok(solutions)
    }

    /// List all available exercises
    pub fn list_all<P: AsRef<Path>>(exercises_root: P) -> Result<Vec<ExerciseMetadata>> {
        let exercises_root = exercises_root.as_ref();
        let mut exercises = Vec::new();

        for entry in WalkDir::new(exercises_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "metadata.json")
        {
            if let Ok(metadata) = ExerciseMetadata::load_from_file(entry.path()) {
                exercises.push(metadata);
            }
        }

        // Sort by chapter and exercise number
        exercises.sort_by(|a, b| {
            a.chapter
                .cmp(&b.chapter)
                .then_with(|| a.exercise_number.cmp(&b.exercise_number))
        });

        Ok(exercises)
    }

    /// Get the primary source file for editing
    pub fn get_primary_source(&self) -> Option<&SourceFile> {
        // Look for main.rs first, then lib.rs, then any template file
        self.source_files
            .iter()
            .find(|f| f.name == "main.rs")
            .or_else(|| self.source_files.iter().find(|f| f.name == "lib.rs"))
            .or_else(|| self.source_files.iter().find(|f| f.is_template))
            .or_else(|| self.source_files.first())
    }

    /// Check if exercise is complete (has working implementation)
    pub fn is_complete(&self) -> bool {
        // Simple heuristic: no TODO comments or unimplemented! macros
        self.source_files.iter().all(|file| {
            !file.content.contains("TODO")
                && !file.content.contains("unimplemented!")
                && !file.content.contains("todo!")
        })
    }

    /// Get estimated completion time
    pub fn estimated_time(&self) -> u32 {
        self.metadata.estimated_time_minutes
    }

    /// Get primary Rust Book reference
    pub fn primary_book_reference(&self) -> &str {
        &self.metadata.rust_book_refs.primary_chapter
    }
}
