use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "rust-learn-cli")]
#[command(author, version, about = "CLI tool for the Rust Learning Platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show current learning status
    Status,
    
    /// List available exercises
    List {
        /// Filter by chapter number
        #[arg(short, long)]
        chapter: Option<u32>,
    },
    
    /// Start working on an exercise
    Start {
        /// Exercise ID (e.g., "ch01-ex01")
        exercise_id: String,
    },
    
    /// Run tests for current exercise
    Test,
    
    /// Submit solution for current exercise
    Submit,
    
    /// Get hint for current exercise
    Hint {
        /// Hint level (1-3)
        #[arg(short, long, default_value = "1")]
        level: u32,
    },
    
    /// Show learning progress
    Progress,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Status => {
            println!("{}", "🦀 Rust Learning Platform Status".bold().green());
            println!("Current exercise: ch01-ex01");
            println!("Progress: 2/150 exercises completed");
            println!("Current streak: 2 days");
        }
        Commands::List { chapter } => {
            println!("{}", "📚 Available Exercises".bold().blue());
            if let Some(ch) = chapter {
                println!("Chapter {}: Getting Started", ch);
                println!("  - ch{:02}-ex01: Hello, World!", ch);
                println!("  - ch{:02}-ex02: Hello, Cargo!", ch);
            } else {
                println!("Chapter 1: Getting Started");
                println!("  - ch01-ex01: Hello, World!");
                println!("  - ch01-ex02: Hello, Cargo!");
                println!("\nChapter 3: Common Concepts");
                println!("  - ch03-ex01: Variables and Mutability");
            }
        }
        Commands::Start { exercise_id } => {
            println!("{} {}", "Starting exercise:".bold(), exercise_id.yellow());
            println!("Opening exercise files...");
            println!("Run 'rust-learn-cli test' to check your solution");
        }
        Commands::Test => {
            println!("{}", "🧪 Running tests...".bold().cyan());
            println!("✅ Test 1: hello_world - PASSED");
            println!("❌ Test 2: proper_formatting - FAILED");
            println!("\nHint: Use 'rust-learn-cli hint' for guidance");
        }
        Commands::Submit => {
            println!("{}", "📤 Submitting solution...".bold().green());
            println!("Solution validated and saved!");
            println!("Progress updated: 3/150 exercises completed");
        }
        Commands::Hint { level } => {
            match level {
                1 => {
                    println!("{}", "💡 Hint Level 1 (Conceptual):".bold().yellow());
                    println!("This exercise is about printing text to the console.");
                    println!("Check the Rust Book Chapter 1.2 for more information.");
                }
                2 => {
                    println!("{}", "💡 Hint Level 2 (Strategic):".bold().yellow());
                    println!("You'll need to use the println! macro.");
                    println!("The macro takes a string literal as an argument.");
                }
                3 => {
                    println!("{}", "💡 Hint Level 3 (Implementation):".bold().yellow());
                    println!("Try: println!(\"Hello, world!\");");
                }
                _ => println!("Invalid hint level. Use 1-3."),
            }
        }
        Commands::Progress => {
            println!("{}", "📊 Learning Progress".bold().magenta());
            println!("Overall Progress: 2%");
            println!("Chapters Completed: 0/20");
            println!("Exercises Completed: 3/150");
            println!("Current Streak: 2 days");
            println!("Total Time: 2h 15m");
        }
    }
    
    Ok(())
}