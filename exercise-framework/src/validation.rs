use crate::exercise::{Exercise, ExerciseType};
use crate::metadata::ExerciseMetadata;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Exercise validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub exercise_id: String,
    pub is_valid: bool,
    pub score: f64, // 0.0 to 1.0
    pub issues: Vec<ValidationIssue>,
    pub suggestions: Vec<String>,
    pub metadata_check: MetadataValidation,
    pub content_check: ContentValidation,
    pub pedagogical_check: PedagogicalValidation,
}

/// Individual validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub suggestion: Option<String>,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Error,   // Must fix
    Warning, // Should fix
    Info,    // Nice to fix
}

/// Categories of validation issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Metadata,      // Exercise metadata issues
    Content,       // Code content issues
    Pedagogical,   // Learning effectiveness issues
    Technical,     // Technical implementation issues
    Accessibility, // Accessibility concerns
}

/// Metadata validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataValidation {
    pub valid: bool,
    pub required_fields_present: bool,
    pub difficulty_appropriate: bool,
    pub time_estimate_reasonable: bool,
    pub concepts_well_defined: bool,
    pub rust_book_refs_valid: bool,
}

/// Content validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentValidation {
    pub valid: bool,
    pub code_compiles: bool,
    pub tests_comprehensive: bool,
    pub hints_helpful: bool,
    pub solutions_correct: bool,
    pub documentation_clear: bool,
}

/// Pedagogical validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedagogicalValidation {
    pub valid: bool,
    pub learning_objectives_clear: bool,
    pub difficulty_progression_smooth: bool,
    pub concepts_properly_introduced: bool,
    pub real_world_relevance: bool,
    pub beginner_friendly: bool,
}

/// Exercise validator
pub struct ExerciseValidator {
    rust_book_chapters: HashSet<String>,
    concept_taxonomy: HashSet<String>,
}

impl ExerciseValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            rust_book_chapters: Self::load_rust_book_chapters(),
            concept_taxonomy: Self::load_concept_taxonomy(),
        }
    }

    /// Validate an exercise comprehensively
    pub fn validate(&self, exercise: &Exercise) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Validate metadata
        let metadata_check = self.validate_metadata(&exercise.metadata, &mut issues)?;

        // Validate content
        let content_check = self.validate_content(exercise, &mut issues)?;

        // Validate pedagogical aspects
        let pedagogical_check = self.validate_pedagogical_aspects(exercise, &mut issues)?;

        // Calculate overall score
        let score =
            self.calculate_score(&metadata_check, &content_check, &pedagogical_check, &issues);

        // Generate suggestions
        self.generate_suggestions(&issues, &mut suggestions);

        let is_valid = score >= 0.7
            && !issues
                .iter()
                .any(|i| matches!(i.severity, IssueSeverity::Error));

        Ok(ValidationResult {
            exercise_id: exercise.metadata.id.clone(),
            is_valid,
            score,
            issues,
            suggestions,
            metadata_check,
            content_check,
            pedagogical_check,
        })
    }

    /// Validate exercise metadata
    fn validate_metadata(
        &self,
        metadata: &ExerciseMetadata,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<MetadataValidation> {
        let mut valid = true;

        // Check required fields
        let required_fields_present = self.check_required_fields(metadata, issues);
        if !required_fields_present {
            valid = false;
        }

        // Check difficulty appropriateness
        let difficulty_appropriate = self.check_difficulty_appropriateness(metadata, issues);

        // Check time estimate
        let time_estimate_reasonable = self.check_time_estimate(metadata, issues);

        // Check concepts
        let concepts_well_defined = self.check_concepts(metadata, issues);

        // Check Rust Book references
        let rust_book_refs_valid = self.check_rust_book_refs(metadata, issues);
        if !rust_book_refs_valid {
            valid = false;
        }

        Ok(MetadataValidation {
            valid,
            required_fields_present,
            difficulty_appropriate,
            time_estimate_reasonable,
            concepts_well_defined,
            rust_book_refs_valid,
        })
    }

    /// Check if all required metadata fields are present
    fn check_required_fields(
        &self,
        metadata: &ExerciseMetadata,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        let mut all_present = true;

        if metadata.id.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Metadata,
                message: "Exercise ID is required".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some("Add a unique exercise ID like 'ch03-ex02-variables'".to_string()),
            });
            all_present = false;
        }

        if metadata.title.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Metadata,
                message: "Exercise title is required".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some("Add a descriptive title for the exercise".to_string()),
            });
            all_present = false;
        }

        if metadata.description.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Metadata,
                message: "Exercise description is required".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some(
                    "Add a clear description explaining what students will learn".to_string(),
                ),
            });
            all_present = false;
        }

        all_present
    }

    /// Check if difficulty level is appropriate
    fn check_difficulty_appropriateness(
        &self,
        metadata: &ExerciseMetadata,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        // Check if time estimate matches difficulty
        let expected_time = match metadata.difficulty.as_str() {
            "beginner" => (5, 25),
            "intermediate" => (15, 45),
            "advanced" => (30, 90),
            _ => return false,
        };

        if metadata.estimated_time_minutes < expected_time.0
            || metadata.estimated_time_minutes > expected_time.1
        {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Metadata,
                message: format!(
                    "Time estimate ({} min) may not match difficulty level ({})",
                    metadata.estimated_time_minutes, metadata.difficulty
                ),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some(format!(
                    "Consider adjusting time estimate to {}-{} minutes for {} exercises",
                    expected_time.0, expected_time.1, metadata.difficulty
                )),
            });
            return false;
        }

        true
    }

    /// Check if time estimate is reasonable
    fn check_time_estimate(
        &self,
        metadata: &ExerciseMetadata,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        if metadata.estimated_time_minutes == 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Metadata,
                message: "Time estimate must be greater than 0".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some("Provide a realistic time estimate in minutes".to_string()),
            });
            return false;
        }

        if metadata.estimated_time_minutes > 180 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Metadata,
                message: "Time estimate is very high (>3 hours)".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some(
                    "Consider breaking this into multiple smaller exercises".to_string(),
                ),
            });
        }

        true
    }

    /// Check if concepts are well-defined
    fn check_concepts(
        &self,
        metadata: &ExerciseMetadata,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        if metadata.concepts.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Metadata,
                message: "At least one concept must be specified".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some("Add the main Rust concepts this exercise teaches".to_string()),
            });
            return false;
        }

        // Check if concepts are in our taxonomy
        for concept in &metadata.concepts {
            if !self.concept_taxonomy.contains(concept) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    category: IssueCategory::Metadata,
                    message: format!("Concept '{}' is not in the standard taxonomy", concept),
                    file: Some("metadata.json".to_string()),
                    line: None,
                    suggestion: Some("Use standard concept names for consistency".to_string()),
                });
            }
        }

        true
    }

    /// Check if Rust Book references are valid
    fn check_rust_book_refs(
        &self,
        metadata: &ExerciseMetadata,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        if metadata.rust_book_refs.primary_chapter.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Metadata,
                message: "Primary Rust Book chapter reference is required".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some("Add the main book chapter this exercise relates to".to_string()),
            });
            return false;
        }

        // Validate chapter format
        if !self
            .rust_book_chapters
            .contains(&metadata.rust_book_refs.primary_chapter)
        {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Metadata,
                message: format!(
                    "Chapter '{}' may not exist in the Rust Book",
                    metadata.rust_book_refs.primary_chapter
                ),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some(
                    "Verify the chapter reference against the official Rust Book".to_string(),
                ),
            });
        }

        true
    }

    /// Validate exercise content
    fn validate_content(
        &self,
        exercise: &Exercise,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<ContentValidation> {
        let mut valid = true;

        // Check if code compiles (basic check)
        let code_compiles = self.check_compilation(exercise, issues);
        if !code_compiles {
            valid = false;
        }

        // Check test comprehensiveness
        let tests_comprehensive = self.check_test_coverage(exercise, issues);

        // Check hint quality
        let hints_helpful = self.check_hint_quality(exercise, issues);

        // Check solution correctness
        let solutions_correct = self.check_solutions(exercise, issues);

        // Check documentation clarity
        let documentation_clear = self.check_documentation(exercise, issues);

        Ok(ContentValidation {
            valid,
            code_compiles,
            tests_comprehensive,
            hints_helpful,
            solutions_correct,
            documentation_clear,
        })
    }

    /// Check if code has basic compilation issues
    fn check_compilation(&self, exercise: &Exercise, issues: &mut Vec<ValidationIssue>) -> bool {
        // Look for obvious syntax errors in source files
        for source_file in &exercise.source_files {
            if source_file.content.contains("unimplemented!") && !source_file.is_template {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    category: IssueCategory::Content,
                    message: "Source file contains unimplemented! macro".to_string(),
                    file: Some(source_file.name.clone()),
                    line: None,
                    suggestion: Some("Complete the implementation or mark as template".to_string()),
                });
            }
        }

        // Check for missing main function in executable exercises
        if exercise.metadata.exercise_type_enum() != ExerciseType::Performance {
            let has_main = exercise
                .source_files
                .iter()
                .any(|f| f.content.contains("fn main()"));

            if !has_main && !exercise.source_files.iter().any(|f| f.name == "lib.rs") {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Content,
                    message: "No main function found in executable exercise".to_string(),
                    file: Some("src/main.rs".to_string()),
                    line: None,
                    suggestion: Some(
                        "Add a main function or convert to library exercise".to_string(),
                    ),
                });
                return false;
            }
        }

        true
    }

    /// Check test coverage and quality
    fn check_test_coverage(&self, exercise: &Exercise, issues: &mut Vec<ValidationIssue>) -> bool {
        if exercise.test_files.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Content,
                message: "No test files found".to_string(),
                file: None,
                line: None,
                suggestion: Some("Add test files to validate exercise solutions".to_string()),
            });
            return false;
        }

        // Check for basic test patterns
        let mut has_unit_tests = false;
        for test_file in &exercise.test_files {
            if test_file.content.contains("#[test]") {
                has_unit_tests = true;
                break;
            }
        }

        if !has_unit_tests {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Content,
                message: "No unit tests found in test files".to_string(),
                file: Some("tests/".to_string()),
                line: None,
                suggestion: Some(
                    "Add #[test] functions to validate exercise implementation".to_string(),
                ),
            });
            return false;
        }

        true
    }

    /// Check quality of hints
    fn check_hint_quality(&self, exercise: &Exercise, issues: &mut Vec<ValidationIssue>) -> bool {
        if exercise.hints.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Content,
                message: "No hints provided".to_string(),
                file: Some("src/hints.md".to_string()),
                line: None,
                suggestion: Some("Add progressive hints to help struggling students".to_string()),
            });
            return false;
        }

        // Check for appropriate number of hints
        if exercise.hints.len() < 2 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Content,
                message: "Only one hint level provided".to_string(),
                file: Some("src/hints.md".to_string()),
                line: None,
                suggestion: Some(
                    "Provide 3 levels of hints: conceptual, strategic, implementation".to_string(),
                ),
            });
        }

        true
    }

    /// Check solution quality and correctness
    fn check_solutions(&self, exercise: &Exercise, issues: &mut Vec<ValidationIssue>) -> bool {
        if exercise.solutions.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Content,
                message: "No reference solutions provided".to_string(),
                file: Some("solutions/".to_string()),
                line: None,
                suggestion: Some(
                    "Add at least one reference solution with explanation".to_string(),
                ),
            });
            return false;
        }

        // Check for primary solution
        let has_primary = exercise.solutions.iter().any(|s| s.is_primary);
        if !has_primary {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Content,
                message: "No primary solution identified".to_string(),
                file: Some("solutions/reference.rs".to_string()),
                line: None,
                suggestion: Some("Mark one solution as the primary reference".to_string()),
            });
        }

        true
    }

    /// Check documentation clarity
    fn check_documentation(&self, exercise: &Exercise, issues: &mut Vec<ValidationIssue>) -> bool {
        // Check README exists
        let readme_exists = exercise.path.join("README.md").exists();
        if !readme_exists {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Content,
                message: "No README.md file found".to_string(),
                file: Some("README.md".to_string()),
                line: None,
                suggestion: Some(
                    "Add a README.md with exercise description and instructions".to_string(),
                ),
            });
            return false;
        }

        true
    }

    /// Validate pedagogical aspects
    fn validate_pedagogical_aspects(
        &self,
        exercise: &Exercise,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<PedagogicalValidation> {
        let learning_objectives_clear = self.check_learning_objectives(exercise, issues);
        let difficulty_progression_smooth = self.check_difficulty_progression(exercise, issues);
        let concepts_properly_introduced = self.check_concept_introduction(exercise, issues);
        let real_world_relevance = self.check_real_world_relevance(exercise, issues);
        let beginner_friendly = self.check_beginner_friendliness(exercise, issues);

        let valid = learning_objectives_clear && concepts_properly_introduced;

        Ok(PedagogicalValidation {
            valid,
            learning_objectives_clear,
            difficulty_progression_smooth,
            concepts_properly_introduced,
            real_world_relevance,
            beginner_friendly,
        })
    }

    /// Check if learning objectives are clear
    fn check_learning_objectives(
        &self,
        exercise: &Exercise,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        // Check if description explains what students will learn
        if exercise.metadata.description.len() < 50 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Pedagogical,
                message: "Exercise description is very brief".to_string(),
                file: Some("metadata.json".to_string()),
                line: None,
                suggestion: Some(
                    "Expand description to clearly explain learning objectives".to_string(),
                ),
            });
            return false;
        }

        true
    }

    /// Check difficulty progression
    fn check_difficulty_progression(
        &self,
        _exercise: &Exercise,
        _issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        // TODO: Implement cross-exercise difficulty analysis
        true
    }

    /// Check concept introduction
    fn check_concept_introduction(
        &self,
        _exercise: &Exercise,
        _issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        // TODO: Check if prerequisites are properly handled
        true
    }

    /// Check real-world relevance
    fn check_real_world_relevance(
        &self,
        _exercise: &Exercise,
        _issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        // TODO: Analyze if exercise teaches practical skills
        true
    }

    /// Check beginner-friendliness
    fn check_beginner_friendliness(
        &self,
        exercise: &Exercise,
        issues: &mut Vec<ValidationIssue>,
    ) -> bool {
        if exercise.metadata.difficulty == "beginner" {
            // Check for complex language in description
            let complex_words = ["polymorphism", "metaprogramming", "monomorphization"];
            for word in complex_words {
                if exercise.metadata.description.to_lowercase().contains(word) {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Warning,
                        category: IssueCategory::Pedagogical,
                        message: format!("Description contains complex term: '{}'", word),
                        file: Some("metadata.json".to_string()),
                        line: None,
                        suggestion: Some("Use simpler language for beginner exercises".to_string()),
                    });
                }
            }
        }

        true
    }

    /// Calculate overall validation score
    fn calculate_score(
        &self,
        metadata: &MetadataValidation,
        content: &ContentValidation,
        pedagogical: &PedagogicalValidation,
        issues: &[ValidationIssue],
    ) -> f64 {
        let metadata_score = if metadata.valid { 1.0 } else { 0.5 };
        let content_score = if content.valid { 1.0 } else { 0.3 };
        let pedagogical_score = if pedagogical.valid { 1.0 } else { 0.7 };

        let base_score = (metadata_score + content_score + pedagogical_score) / 3.0;

        // Penalize for issues
        let error_penalty = issues
            .iter()
            .filter(|i| matches!(i.severity, IssueSeverity::Error))
            .count() as f64
            * 0.2;

        let warning_penalty = issues
            .iter()
            .filter(|i| matches!(i.severity, IssueSeverity::Warning))
            .count() as f64
            * 0.05;

        (base_score - error_penalty - warning_penalty)
            .max(0.0)
            .min(1.0)
    }

    /// Generate actionable suggestions
    fn generate_suggestions(&self, issues: &[ValidationIssue], suggestions: &mut Vec<String>) {
        // Group issues by category and provide category-specific advice
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.severity, IssueSeverity::Error))
            .collect();

        if !errors.is_empty() {
            suggestions
                .push("Fix all error-level issues before publishing this exercise".to_string());
        }

        if issues.len() > 5 {
            suggestions.push(
                "Consider reviewing exercise design - many validation issues found".to_string(),
            );
        }

        // Add specific suggestions based on common patterns
        if issues.iter().any(|i| i.message.contains("test")) {
            suggestions.push("Focus on improving test coverage and quality".to_string());
        }

        if issues.iter().any(|i| i.message.contains("hint")) {
            suggestions
                .push("Enhance the hint system with progressive difficulty levels".to_string());
        }
    }

    /// Load known Rust Book chapters
    fn load_rust_book_chapters() -> HashSet<String> {
        // In a real implementation, this would load from a configuration file
        let chapters = vec![
            "1", "2", "3.1", "3.2", "3.3", "3.4", "3.5", "4.1", "4.2", "4.3", "5.1", "5.2", "5.3",
            "6.1", "6.2", "6.3", "7.1", "7.2", "7.3", "7.4", "7.5", "8.1", "8.2", "8.3", "9.1",
            "9.2", "9.3", "10.1", "10.2", "10.3", "11.1", "11.2", "11.3", "12", "13.1", "13.2",
            "13.3", "13.4", "14.1", "14.2", "14.3", "14.4", "14.5", "15.1", "15.2", "15.3", "15.4",
            "15.5", "15.6", "16.1", "16.2", "16.3", "16.4", "17.1", "17.2", "17.3", "18.1", "18.2",
            "18.3", "19.1", "19.2", "19.3", "19.4", "19.5", "19.6", "20.1", "20.2", "20.3", "20.4",
            "20.5", "20.6",
        ];

        chapters.into_iter().map(String::from).collect()
    }

    /// Load concept taxonomy
    fn load_concept_taxonomy() -> HashSet<String> {
        // Standard Rust concepts
        let concepts = vec![
            "variables",
            "mutability",
            "data-types",
            "functions",
            "control-flow",
            "ownership",
            "borrowing",
            "references",
            "lifetimes",
            "slices",
            "structs",
            "methods",
            "enums",
            "pattern-matching",
            "collections",
            "error-handling",
            "generics",
            "traits",
            "testing",
            "iterators",
            "closures",
            "smart-pointers",
            "concurrency",
            "async",
            "macros",
            "modules",
            "packages",
            "crates",
            "workspaces",
            "documentation",
        ];

        concepts.into_iter().map(String::from).collect()
    }
}

impl Default for ExerciseValidator {
    fn default() -> Self {
        Self::new()
    }
}
