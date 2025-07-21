use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hint system for progressive learning assistance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HintLevel {
    Conceptual,     // Level 1: Points to concepts and documentation
    Strategic,      // Level 2: Suggests approach without code
    Implementation, // Level 3: Provides specific code guidance
}

/// Individual hint with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub level: HintLevel,
    pub title: String,
    pub content: String,
    pub rust_book_links: Vec<String>,
    pub code_snippets: Vec<CodeSnippet>,
    pub related_concepts: Vec<String>,
}

/// Code snippet within a hint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String, // Usually "rust"
    pub code: String,
    pub explanation: String,
    pub is_complete: bool, // true if runnable, false if just a fragment
}

/// Hint usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintUsage {
    pub exercise_id: String,
    pub user_id: String,
    pub hints_requested: Vec<HintRequest>,
    pub total_hints_used: u32,
    pub time_before_first_hint: u32, // seconds
    pub solved_after_hints: bool,
}

/// Individual hint request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintRequest {
    pub level: HintLevel,
    pub requested_at: chrono::DateTime<chrono::Utc>,
    pub was_helpful: Option<bool>, // User feedback
}

/// Hint system manager
pub struct HintSystem {
    exercise_hints: HashMap<String, Vec<Hint>>,
    usage_tracker: HintUsageTracker,
}

/// Tracks hint usage for analytics
pub struct HintUsageTracker {
    usage_data: HashMap<String, Vec<HintUsage>>,
}

impl HintSystem {
    /// Create a new hint system
    pub fn new() -> Self {
        Self {
            exercise_hints: HashMap::new(),
            usage_tracker: HintUsageTracker::new(),
        }
    }

    /// Load hints for an exercise from markdown file
    pub fn load_exercise_hints(&mut self, exercise_id: &str, hints_content: &str) -> anyhow::Result<()> {
        let hints = Self::parse_hints_from_markdown(hints_content)?;
        self.exercise_hints.insert(exercise_id.to_string(), hints);
        Ok(())
    }

    /// Get next available hint for an exercise
    pub fn get_next_hint(&self, exercise_id: &str, current_level: Option<HintLevel>) -> Option<&Hint> {
        let hints = self.exercise_hints.get(exercise_id)?;
        
        let target_level = match current_level {
            None => HintLevel::Conceptual,
            Some(HintLevel::Conceptual) => HintLevel::Strategic,
            Some(HintLevel::Strategic) => HintLevel::Implementation,
            Some(HintLevel::Implementation) => return None, // No more hints
        };

        hints.iter().find(|hint| matches!(hint.level, target_level) == matches!(target_level, _))
    }

    /// Get all hints for an exercise (for instructors/solutions)
    pub fn get_all_hints(&self, exercise_id: &str) -> Option<&Vec<Hint>> {
        self.exercise_hints.get(exercise_id)
    }

    /// Record hint usage for analytics
    pub fn record_hint_usage(&mut self, exercise_id: &str, user_id: &str, level: HintLevel) {
        self.usage_tracker.record_usage(exercise_id, user_id, level);
    }

    /// Get hint effectiveness statistics
    pub fn get_hint_effectiveness(&self, exercise_id: &str) -> HintEffectiveness {
        self.usage_tracker.calculate_effectiveness(exercise_id)
    }

    /// Parse hints from markdown content
    fn parse_hints_from_markdown(content: &str) -> anyhow::Result<Vec<Hint>> {
        let mut hints = Vec::new();
        let mut current_hint: Option<Hint> = None;
        let mut current_content = String::new();
        let mut in_code_block = false;
        let mut current_code_snippet = String::new();

        for line in content.lines() {
            // Check for hint level headers
            if let Some(level) = Self::parse_hint_level(line) {
                // Save previous hint if exists
                if let Some(mut hint) = current_hint.take() {
                    hint.content = current_content.trim().to_string();
                    hints.push(hint);
                    current_content.clear();
                }

                // Start new hint
                current_hint = Some(Hint {
                    level,
                    title: line.trim_start_matches('#').trim().to_string(),
                    content: String::new(),
                    rust_book_links: Vec::new(),
                    code_snippets: Vec::new(),
                    related_concepts: Vec::new(),
                });
            } else if line.starts_with("```rust") {
                in_code_block = true;
                current_code_snippet.clear();
            } else if line.starts_with("```") && in_code_block {
                in_code_block = false;
                if let Some(ref mut hint) = current_hint {
                    hint.code_snippets.push(CodeSnippet {
                        language: "rust".to_string(),
                        code: current_code_snippet.clone(),
                        explanation: "Code example".to_string(),
                        is_complete: current_code_snippet.contains("fn main()"),
                    });
                }
                current_code_snippet.clear();
            } else if in_code_block {
                current_code_snippet.push_str(line);
                current_code_snippet.push('\n');
            } else {
                // Regular content
                if current_hint.is_some() {
                    // Look for Rust Book links
                    if line.contains("https://doc.rust-lang.org/book/") {
                        if let Some(ref mut hint) = current_hint {
                            Self::extract_book_links(line, &mut hint.rust_book_links);
                        }
                    }
                    current_content.push_str(line);
                    current_content.push('\n');
                }
            }
        }

        // Don't forget the last hint
        if let Some(mut hint) = current_hint {
            hint.content = current_content.trim().to_string();
            hints.push(hint);
        }

        Ok(hints)
    }

    /// Parse hint level from markdown header
    fn parse_hint_level(line: &str) -> Option<HintLevel> {
        let line = line.trim();
        if line.starts_with("## Level 1") || line.contains("Conceptual") {
            Some(HintLevel::Conceptual)
        } else if line.starts_with("## Level 2") || line.contains("Strategic") {
            Some(HintLevel::Strategic)
        } else if line.starts_with("## Level 3") || line.contains("Implementation") {
            Some(HintLevel::Implementation)
        } else {
            None
        }
    }

    /// Extract Rust Book links from text
    fn extract_book_links(text: &str, links: &mut Vec<String>) {
        // Simple regex-like extraction
        if let Some(start) = text.find("https://doc.rust-lang.org/book/") {
            if let Some(end) = text[start..].find([' ', ')', '\n', ']']) {
                let link = &text[start..start + end];
                links.push(link.to_string());
            } else {
                // Link goes to end of line
                let link = &text[start..];
                links.push(link.to_string());
            }
        }
    }

    /// Generate contextual hints based on error patterns
    pub fn generate_contextual_hint(&self, exercise_id: &str, error_message: &str) -> Option<Hint> {
        // Analyze common error patterns and provide targeted hints
        if error_message.contains("cannot borrow") {
            Some(Hint {
                level: HintLevel::Conceptual,
                title: "Borrowing Error".to_string(),
                content: "This error occurs when you're trying to borrow a value in a way that violates Rust's borrowing rules.".to_string(),
                rust_book_links: vec!["https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html".to_string()],
                code_snippets: vec![
                    CodeSnippet {
                        language: "rust".to_string(),
                        code: "// Instead of:\n// let mut x = vec![1, 2, 3];\n// let y = &mut x;\n// let z = &mut x; // Error!\n\n// Do:\nlet mut x = vec![1, 2, 3];\n{\n    let y = &mut x;\n    // use y here\n}\nlet z = &mut x; // OK now".to_string(),
                        explanation: "Only one mutable reference is allowed at a time".to_string(),
                        is_complete: true,
                    }
                ],
                related_concepts: vec!["borrowing".to_string(), "references".to_string()],
            })
        } else if error_message.contains("move occurs") {
            Some(Hint {
                level: HintLevel::Conceptual,
                title: "Ownership Move".to_string(),
                content: "This error happens when you try to use a value after it has been moved to another owner.".to_string(),
                rust_book_links: vec!["https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html".to_string()],
                code_snippets: vec![
                    CodeSnippet {
                        language: "rust".to_string(),
                        code: "// Instead of moving, try borrowing:\nlet s1 = String::from(\"hello\");\nlet len = calculate_length(&s1); // borrow instead of move\nprintln!(\"{}\", s1); // s1 is still valid".to_string(),
                        explanation: "Use references to avoid moving ownership".to_string(),
                        is_complete: false,
                    }
                ],
                related_concepts: vec!["ownership".to_string(), "move-semantics".to_string()],
            })
        } else if error_message.contains("type annotations needed") {
            Some(Hint {
                level: HintLevel::Strategic,
                title: "Type Inference".to_string(),
                content: "Rust can't figure out what type you want. You need to help it with a type annotation.".to_string(),
                rust_book_links: vec!["https://doc.rust-lang.org/book/ch03-02-data-types.html".to_string()],
                code_snippets: vec![
                    CodeSnippet {
                        language: "rust".to_string(),
                        code: "// Be explicit about the type:\nlet numbers: Vec<i32> = vec.iter().map(|x| x * 2).collect();\n// or\nlet numbers = vec.iter().map(|x| x * 2).collect::<Vec<i32>>();".to_string(),
                        explanation: "Specify the type Rust should infer".to_string(),
                        is_complete: false,
                    }
                ],
                related_concepts: vec!["type-inference".to_string(), "generics".to_string()],
            })
        } else {
            None
        }
    }
}

impl Default for HintSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HintUsageTracker {
    /// Create a new usage tracker
    pub fn new() -> Self {
        Self {
            usage_data: HashMap::new(),
        }
    }

    /// Record hint usage
    pub fn record_usage(&mut self, exercise_id: &str, user_id: &str, level: HintLevel) {
        let usage = self.usage_data
            .entry(exercise_id.to_string())
            .or_insert_with(Vec::new);

        // Find or create usage record for this user
        if let Some(user_usage) = usage.iter_mut().find(|u| u.user_id == user_id) {
            user_usage.hints_requested.push(HintRequest {
                level,
                requested_at: chrono::Utc::now(),
                was_helpful: None, // Will be updated by user feedback
            });
            user_usage.total_hints_used += 1;
        } else {
            // New user
            usage.push(HintUsage {
                exercise_id: exercise_id.to_string(),
                user_id: user_id.to_string(),
                hints_requested: vec![HintRequest {
                    level,
                    requested_at: chrono::Utc::now(),
                    was_helpful: None,
                }],
                total_hints_used: 1,
                time_before_first_hint: 0, // TODO: Calculate from exercise start time
                solved_after_hints: false, // TODO: Update when exercise is completed
            });
        }
    }

    /// Calculate hint effectiveness for an exercise
    pub fn calculate_effectiveness(&self, exercise_id: &str) -> HintEffectiveness {
        let usage_data = self.usage_data.get(exercise_id);
        
        if let Some(data) = usage_data {
            let total_users = data.len() as f64;
            let users_who_used_hints = data.iter().filter(|u| u.total_hints_used > 0).count() as f64;
            let users_solved_after_hints = data.iter().filter(|u| u.solved_after_hints).count() as f64;
            
            let hint_usage_rate = users_who_used_hints / total_users;
            let success_rate_with_hints = if users_who_used_hints > 0.0 {
                users_solved_after_hints / users_who_used_hints
            } else {
                0.0
            };

            let avg_hints_per_user = if users_who_used_hints > 0.0 {
                data.iter().map(|u| u.total_hints_used).sum::<u32>() as f64 / users_who_used_hints
            } else {
                0.0
            };

            HintEffectiveness {
                exercise_id: exercise_id.to_string(),
                hint_usage_rate,
                success_rate_with_hints,
                average_hints_per_user: avg_hints_per_user,
                most_requested_level: HintLevel::Conceptual, // TODO: Calculate actual
                improvement_suggestions: vec![
                    "Consider adding more conceptual hints".to_string(),
                    "Strategic hints may need more detail".to_string(),
                ],
            }
        } else {
            HintEffectiveness {
                exercise_id: exercise_id.to_string(),
                hint_usage_rate: 0.0,
                success_rate_with_hints: 0.0,
                average_hints_per_user: 0.0,
                most_requested_level: HintLevel::Conceptual,
                improvement_suggestions: vec!["No usage data available".to_string()],
            }
        }
    }
}

/// Hint effectiveness analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintEffectiveness {
    pub exercise_id: String,
    pub hint_usage_rate: f64, // Percentage of users who use hints
    pub success_rate_with_hints: f64, // Success rate for users who used hints
    pub average_hints_per_user: f64,
    pub most_requested_level: HintLevel,
    pub improvement_suggestions: Vec<String>,
}

/// Helper functions for hint formatting
impl Hint {
    /// Format hint for display in terminal
    pub fn format_for_terminal(&self) -> String {
        let mut formatted = String::new();
        
        formatted.push_str(&format!("💡 {} Hint: {}\n", 
            match self.level {
                HintLevel::Conceptual => "🔍",
                HintLevel::Strategic => "🎯",
                HintLevel::Implementation => "🔧",
            },
            self.title
        ));
        
        formatted.push_str("─".repeat(50).as_str());
        formatted.push('\n');
        formatted.push_str(&self.content);
        formatted.push('\n');
        
        if !self.rust_book_links.is_empty() {
            formatted.push_str("\n📚 Related Reading:\n");
            for link in &self.rust_book_links {
                formatted.push_str(&format!("  {}\n", link));
            }
        }
        
        if !self.code_snippets.is_empty() {
            formatted.push_str("\n💻 Code Example:\n");
            for snippet in &self.code_snippets {
                formatted.push_str("```rust\n");
                formatted.push_str(&snippet.code);
                formatted.push_str("\n```\n");
                if !snippet.explanation.is_empty() {
                    formatted.push_str(&format!("// {}\n", snippet.explanation));
                }
            }
        }
        
        formatted
    }

    /// Format hint for web UI (HTML)
    pub fn format_for_web(&self) -> String {
        // This would generate HTML for the web interface
        format!(
            r#"<div class="hint hint-{}">
                <h3>{}</h3>
                <p>{}</p>
                <!-- Add code snippets, links, etc. -->
            </div>"#,
            match self.level {
                HintLevel::Conceptual => "conceptual",
                HintLevel::Strategic => "strategic", 
                HintLevel::Implementation => "implementation",
            },
            self.title,
            self.content
        )
    }
}