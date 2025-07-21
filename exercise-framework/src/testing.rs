use crate::exercise::{Exercise, TestType};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// Test execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub exercise_id: String,
    pub success: bool,
    pub execution_time: Duration,
    pub test_results: Vec<IndividualTestResult>,
    pub compilation_result: CompilationResult,
    pub quality_check: QualityResult,
    pub performance_metrics: Option<PerformanceMetrics>,
}

/// Result for individual test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualTestResult {
    pub name: String,
    pub test_type: TestType,
    pub passed: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: Duration,
}

/// Compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub success: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub clippy_issues: Vec<ClippyIssue>,
}

/// Individual Clippy issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClippyIssue {
    pub level: String, // "error", "warning", "note"
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub suggestion: Option<String>,
}

/// Code quality assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityResult {
    pub format_score: f64,     // 0.0 to 1.0
    pub clippy_score: f64,     // 0.0 to 1.0
    pub test_coverage: f64,    // 0.0 to 1.0
    pub overall_score: f64,    // 0.0 to 1.0
    pub suggestions: Vec<String>,
}

/// Performance metrics for optimization exercises
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub execution_time_ns: u64,
    pub memory_usage_bytes: u64,
    pub benchmark_results: Vec<BenchmarkResult>,
    pub meets_requirements: bool,
}

/// Individual benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub time_ns: u64,
    pub iterations: u64,
    pub throughput: Option<f64>,
}

/// Test runner handles exercise validation
pub struct TestRunner {
    timeout: Duration,
    cargo_path: String,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(60), // 1 minute default timeout
            cargo_path: "cargo".to_string(),  // Assume cargo is in PATH
        }
    }

    /// Run all tests for an exercise
    pub fn run_tests(&self, exercise: &Exercise) -> Result<TestResult> {
        let start_time = Instant::now();

        // Change to exercise directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&exercise.path)?;

        // Ensure we restore directory even on error
        let _guard = DirectoryGuard::new(original_dir);

        // Step 1: Compilation check
        let compilation_result = self.check_compilation(exercise)?;
        
        // Step 2: Run unit tests
        let test_results = if compilation_result.success {
            self.run_unit_tests(exercise)?
        } else {
            Vec::new() // Can't run tests if compilation failed
        };

        // Step 3: Quality checks
        let quality_check = self.run_quality_checks(exercise)?;

        // Step 4: Performance metrics (for performance exercises)
        let performance_metrics = if exercise.metadata.exercise_type_enum() == crate::ExerciseType::Performance {
            Some(self.run_benchmarks(exercise)?)
        } else {
            None
        };

        let success = compilation_result.success && 
                     test_results.iter().all(|t| t.passed) &&
                     quality_check.overall_score >= 0.7; // Minimum quality threshold

        Ok(TestResult {
            exercise_id: exercise.metadata.id.clone(),
            success,
            execution_time: start_time.elapsed(),
            test_results,
            compilation_result,
            quality_check,
            performance_metrics,
        })
    }

    /// Check if the code compiles
    fn check_compilation(&self, exercise: &Exercise) -> Result<CompilationResult> {
        // First check with cargo check (faster)
        let check_output = Command::new(&self.cargo_path)
            .args(["check", "--message-format=json"])
            .output()
            .context("Failed to run cargo check")?;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Parse cargo output (simplified - real implementation would parse JSON)
        if !check_output.status.success() {
            let stderr = String::from_utf8_lossy(&check_output.stderr);
            for line in stderr.lines() {
                if line.contains("error:") {
                    errors.push(line.to_string());
                } else if line.contains("warning:") {
                    warnings.push(line.to_string());
                }
            }
        }

        // Run clippy for additional checks
        let clippy_issues = self.run_clippy()?;

        Ok(CompilationResult {
            success: check_output.status.success() && errors.is_empty(),
            warnings,
            errors,
            clippy_issues,
        })
    }

    /// Run clippy for code quality
    fn run_clippy(&self) -> Result<Vec<ClippyIssue>> {
        let clippy_output = Command::new(&self.cargo_path)
            .args(["clippy", "--", "-W", "clippy::all"])
            .output()
            .context("Failed to run clippy")?;

        let mut issues = Vec::new();
        
        // Parse clippy output (simplified)
        let output = String::from_utf8_lossy(&clippy_output.stdout);
        for line in output.lines() {
            if line.contains("warning:") || line.contains("error:") {
                // Simple parsing - real implementation would be more sophisticated
                issues.push(ClippyIssue {
                    level: if line.contains("error:") { "error" } else { "warning" }.to_string(),
                    message: line.to_string(),
                    file: "src/main.rs".to_string(), // Simplified
                    line: 1,
                    column: 1,
                    suggestion: None,
                });
            }
        }

        Ok(issues)
    }

    /// Run unit tests
    fn run_unit_tests(&self, exercise: &Exercise) -> Result<Vec<IndividualTestResult>> {
        let mut results = Vec::new();

        // Run cargo test with JSON output
        let test_output = Command::new(&self.cargo_path)
            .args(["test", "--", "--nocapture", "--test-threads=1"])
            .output()
            .context("Failed to run cargo test")?;

        let success = test_output.status.success();
        let output = String::from_utf8_lossy(&test_output.stdout);
        let stderr = String::from_utf8_lossy(&test_output.stderr);

        // Parse test results (simplified - real implementation would parse structured output)
        if success {
            // Count passed tests
            let passed_count = output.matches("test result: ok").count();
            for i in 0..passed_count.max(1) {
                results.push(IndividualTestResult {
                    name: format!("test_{}", i + 1),
                    test_type: TestType::Unit,
                    passed: true,
                    output: output.clone().into(),
                    error: None,
                    execution_time: Duration::from_millis(100), // Placeholder
                });
            }
        } else {
            // Test failed
            results.push(IndividualTestResult {
                name: "unit_tests".to_string(),
                test_type: TestType::Unit,
                passed: false,
                output: output.into(),
                error: Some(stderr.into()),
                execution_time: Duration::from_millis(500), // Placeholder
            });
        }

        Ok(results)
    }

    /// Run quality checks
    fn run_quality_checks(&self, exercise: &Exercise) -> Result<QualityResult> {
        // Check formatting
        let fmt_output = Command::new(&self.cargo_path)
            .args(["fmt", "--check"])
            .output()
            .context("Failed to run cargo fmt")?;

        let format_score = if fmt_output.status.success() { 1.0 } else { 0.7 };

        // Clippy score based on issues
        let clippy_output = Command::new(&self.cargo_path)
            .args(["clippy", "--", "-W", "clippy::all"])
            .output()
            .context("Failed to run clippy")?;

        let clippy_issues = String::from_utf8_lossy(&clippy_output.stderr)
            .matches("warning:").count();
        
        let clippy_score = match clippy_issues {
            0 => 1.0,
            1..=3 => 0.8,
            4..=7 => 0.6,
            _ => 0.4,
        };

        // Test coverage (placeholder - would need coverage tools)
        let test_coverage = 0.8; // Assume decent coverage

        let overall_score = (format_score + clippy_score + test_coverage) / 3.0;

        let mut suggestions = Vec::new();
        if format_score < 1.0 {
            suggestions.push("Run 'cargo fmt' to format your code".to_string());
        }
        if clippy_score < 0.8 {
            suggestions.push("Address clippy warnings to improve code quality".to_string());
        }

        Ok(QualityResult {
            format_score,
            clippy_score,
            test_coverage,
            overall_score,
            suggestions,
        })
    }

    /// Run benchmarks for performance exercises
    fn run_benchmarks(&self, exercise: &Exercise) -> Result<PerformanceMetrics> {
        // Check if benchmarks exist
        let bench_path = exercise.path.join("benches");
        if !bench_path.exists() {
            return Ok(PerformanceMetrics {
                execution_time_ns: 0,
                memory_usage_bytes: 0,
                benchmark_results: Vec::new(),
                meets_requirements: true,
            });
        }

        // Run cargo bench (simplified)
        let bench_output = Command::new(&self.cargo_path)
            .args(["bench"])
            .output()
            .context("Failed to run cargo bench")?;

        // Parse benchmark results (placeholder implementation)
        let benchmark_results = vec![
            BenchmarkResult {
                name: "main_benchmark".to_string(),
                time_ns: 1_000_000, // 1ms
                iterations: 1000,
                throughput: Some(1000.0),
            }
        ];

        Ok(PerformanceMetrics {
            execution_time_ns: 1_000_000,
            memory_usage_bytes: 1024 * 1024, // 1MB
            benchmark_results,
            meets_requirements: true, // TODO: Check against requirements
        })
    }

    /// Set custom timeout for test execution
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII guard to restore working directory
struct DirectoryGuard {
    original_dir: std::path::PathBuf,
}

impl DirectoryGuard {
    fn new(original_dir: std::path::PathBuf) -> Self {
        Self { original_dir }
    }
}

impl Drop for DirectoryGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.original_dir);
    }
}

/// Helper functions for test result analysis
impl TestResult {
    /// Check if all tests passed
    pub fn all_tests_passed(&self) -> bool {
        self.success && self.test_results.iter().all(|t| t.passed)
    }

    /// Get failed test names
    pub fn failed_tests(&self) -> Vec<&str> {
        self.test_results
            .iter()
            .filter(|t| !t.passed)
            .map(|t| t.name.as_str())
            .collect()
    }

    /// Get compilation errors
    pub fn compilation_errors(&self) -> &[String] {
        &self.compilation_result.errors
    }

    /// Check if code quality is acceptable
    pub fn meets_quality_standards(&self) -> bool {
        self.quality_check.overall_score >= 0.7
    }

    /// Get actionable feedback for the user
    pub fn get_feedback(&self) -> Vec<String> {
        let mut feedback = Vec::new();

        if !self.compilation_result.success {
            feedback.push("Fix compilation errors first".to_string());
            feedback.extend(self.compilation_result.errors.iter().cloned());
        } else if !self.all_tests_passed() {
            feedback.push("Some tests are failing:".to_string());
            for test in &self.test_results {
                if !test.passed {
                    if let Some(error) = &test.error {
                        feedback.push(format!("  {}: {}", test.name, error));
                    }
                }
            }
        } else if !self.meets_quality_standards() {
            feedback.push("Code quality can be improved:".to_string());
            feedback.extend(self.quality_check.suggestions.iter().cloned());
        } else {
            feedback.push("Great job! All tests pass and code quality is good.".to_string());
        }

        feedback
    }
}