# Hints for Game Enhancements and Advanced Features

## Level 1: Conceptual Hint

This exercise teaches you to think beyond basic functionality toward professional software development practices.

**Understanding Advanced Software Development:**

**Feature Architecture:**
- **Modular Design**: Break complex features into logical components
- **Separation of Concerns**: Each module has a single, clear responsibility
- **Data Management**: Persistent storage of user data and preferences
- **State Management**: Tracking complex application state over time

**Professional Software Characteristics:**
- **Configurability**: Users can customize the experience
- **Data Persistence**: Information survives program restarts
- **Intelligent Behavior**: Software adapts to user patterns
- **Rich Feedback**: Informative, engaging user interfaces
- **Extensibility**: Easy to add new features over time

**Key Components to Implement:**

**1. Difficulty System:**
- Different number ranges and rule sets
- Affects game mechanics (hints, time limits)
- User choice and preference storage

**2. Hint System:**
- Context-aware suggestions based on guess history
- Limited resource that adds strategy
- Different hint types for variety

**3. Statistics Module:**
- Data collection during gameplay
- Calculations for meaningful metrics
- File persistence across sessions

**4. Configuration System:**
- User preferences and settings
- Default values with overrides
- File-based storage and loading

**5. Enhanced UX:**
- Personalization with user names
- Achievement and progress feedback
- Rich, colorful output

**Software Engineering Principles:**
- **DRY (Don't Repeat Yourself)**: Reusable functions and data structures
- **Single Responsibility**: Each function/module does one thing well
- **Error Handling**: Graceful handling of all failure modes
- **User-Centered Design**: Features that actually improve the experience

**Key concepts to understand:**
- How to design software that grows over time
- The importance of data persistence in user applications
- How small enhancements can dramatically improve user experience
- Professional coding practices that make software maintainable

**📖 Read more:** [Software Engineering Best Practices](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) for code organization

## Level 2: Strategic Hint

Break down the implementation into logical modules and data structures:

**Module Organization:**
```rust
// src/main.rs - Main program and coordination
// src/game.rs - Core game logic
// src/difficulty.rs - Difficulty levels and settings  
// src/hints.rs - Hint system and logic
// src/stats.rs - Statistics tracking and calculations
// src/config.rs - Configuration management
// src/ui.rs - User interface and display functions
```

**Key Data Structures:**

**Difficulty Configuration:**
```rust
#[derive(Debug, Clone)]
enum Difficulty {
    Easy { range: (u32, u32), max_hints: u32 },
    Medium { range: (u32, u32), max_hints: u32 },
    Hard { range: (u32, u32), max_hints: u32 },
}
```

**Game Statistics:**
```rust
#[derive(Debug, Clone)]
struct GameStats {
    games_played: u32,
    games_won: u32,
    total_attempts: u32,
    best_attempts: Option<u32>,
    current_streak: u32,
    achievements: Vec<String>,
}
```

**Hint System:**
```rust
struct HintContext {
    previous_guesses: Vec<u32>,
    secret_number: u32,
    hints_used: u32,
    max_hints: u32,
}
```

**Implementation Strategy:**

**1. Start with Data Structures:**
- Define all your data types first
- Include serialization for file I/O
- Plan how data flows between modules

**2. Implement Core Systems:**
- Statistics tracking (in-memory first)
- Configuration management 
- Difficulty selection logic
- Hint generation algorithms

**3. Add File Persistence:**
- Save/load statistics to JSON or simple format
- Configuration file handling
- Error handling for file operations

**4. Enhanced User Interface:**
- Rich formatting and colors
- Progress display
- Achievement notifications

**Key Implementation Points:**
- Use `serde` for JSON serialization if desired
- Create builder patterns for complex configurations
- Implement `Default` trait for reasonable defaults
- Use `Result` types for all file operations

**Think about:**
- How do different difficulty levels affect game mechanics?
- What makes hints helpful without being too easy?
- Which statistics are most meaningful to users?
- How can you make the interface feel responsive and engaging?

## Level 3: Implementation Hint

Here's a complete implementation structure with all enhancements:

**main.rs - Program coordination:**
```rust
mod game;
mod difficulty;
mod stats;
mod config;
mod hints;
mod ui;

use std::io::{self, Write};

fn main() {
    let mut config = config::load_config().unwrap_or_default();
    let mut stats = stats::load_stats().unwrap_or_default();
    
    ui::print_welcome(&config.player_name);
    ui::display_stats(&stats);
    
    loop {
        let difficulty = ui::select_difficulty();
        let mut game_context = game::GameContext::new(difficulty, &mut stats);
        
        game::play_game(&mut game_context);
        
        stats.save_to_file();
        
        if !ui::ask_play_again() {
            break;
        }
    }
    
    ui::print_goodbye(&stats);
}
```

**difficulty.rs - Difficulty system:**
```rust
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium, 
    Hard,
}

impl Difficulty {
    pub fn range(&self) -> (u32, u32) {
        match self {
            Difficulty::Easy => (1, 50),
            Difficulty::Medium => (1, 100),
            Difficulty::Hard => (1, 200),
        }
    }
    
    pub fn max_hints(&self) -> u32 {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Medium => 3,
            Difficulty::Hard => 1,
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium", 
            Difficulty::Hard => "Hard",
        }
    }
}
```

**stats.rs - Statistics tracking:**
```rust
use std::fs;

#[derive(Debug, Clone)]
pub struct GameStats {
    pub games_played: u32,
    pub games_won: u32,
    pub total_attempts: u32,
    pub best_attempts: Option<u32>,
    pub current_streak: u32,
    pub best_streak: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats {
            games_played: 0,
            games_won: 0,
            total_attempts: 0,
            best_attempts: None,
            current_streak: 0,
            best_streak: 0,
        }
    }
}

impl GameStats {
    pub fn record_win(&mut self, attempts: u32) {
        self.games_played += 1;
        self.games_won += 1;
        self.total_attempts += attempts;
        self.current_streak += 1;
        
        if self.current_streak > self.best_streak {
            self.best_streak = self.current_streak;
        }
        
        if self.best_attempts.is_none() || attempts < self.best_attempts.unwrap() {
            self.best_attempts = Some(attempts);
        }
    }
    
    pub fn record_loss(&mut self) {
        self.games_played += 1;
        self.current_streak = 0;
    }
    
    pub fn win_rate(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.games_won as f64 / self.games_played as f64) * 100.0
        }
    }
    
    pub fn average_attempts(&self) -> f64 {
        if self.games_won == 0 {
            0.0
        } else {
            self.total_attempts as f64 / self.games_won as f64
        }
    }
    
    pub fn save_to_file(&self) {
        let data = format!(
            "{},{},{},{:?},{},{}",
            self.games_played,
            self.games_won, 
            self.total_attempts,
            self.best_attempts,
            self.current_streak,
            self.best_streak
        );
        let _ = fs::write("game_stats.txt", data);
    }
}

pub fn load_stats() -> Result<GameStats, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("game_stats.txt")?;
    let parts: Vec<&str> = data.trim().split(',').collect();
    
    if parts.len() != 6 {
        return Err("Invalid stats file format".into());
    }
    
    let best_attempts = if parts[3] == "None" {
        None
    } else {
        Some(parts[3].parse()?)
    };
    
    Ok(GameStats {
        games_played: parts[0].parse()?,
        games_won: parts[1].parse()?,
        total_attempts: parts[2].parse()?,
        best_attempts,
        current_streak: parts[4].parse()?,
        best_streak: parts[5].parse()?,
    })
}
```

**hints.rs - Intelligent hint system:**
```rust
use crate::difficulty::Difficulty;

pub struct HintSystem {
    pub previous_guesses: Vec<u32>,
    pub hints_used: u32,
    pub max_hints: u32,
}

impl HintSystem {
    pub fn new(difficulty: Difficulty) -> Self {
        HintSystem {
            previous_guesses: Vec::new(),
            hints_used: 0,
            max_hints: difficulty.max_hints(),
        }
    }
    
    pub fn add_guess(&mut self, guess: u32) {
        self.previous_guesses.push(guess);
    }
    
    pub fn can_give_hint(&self) -> bool {
        self.hints_used < self.max_hints
    }
    
    pub fn give_hint(&mut self, secret: u32, range: (u32, u32)) -> String {
        if !self.can_give_hint() {
            return "No more hints available!".to_string();
        }
        
        self.hints_used += 1;
        
        if self.previous_guesses.is_empty() {
            return format!("💡 Hint: Try starting somewhere in the middle of the range ({}-{}).", range.0, range.1);
        }
        
        let last_guess = *self.previous_guesses.last().unwrap();
        let distance = if secret > last_guess {
            secret - last_guess
        } else {
            last_guess - secret
        };
        
        match distance {
            1..=5 => "💡 Hint: You're very close! Just a tiny adjustment needed.".to_string(),
            6..=15 => "💡 Hint: Getting warmer! You're in the right neighborhood.".to_string(),
            16..=30 => "💡 Hint: You're on the right track, but need a bigger change.".to_string(),
            _ => {
                if secret > last_guess {
                    "💡 Hint: Try a much larger number.".to_string()
                } else {
                    "💡 Hint: Try a much smaller number.".to_string()
                }
            }
        }
    }
    
    pub fn hints_remaining(&self) -> u32 {
        self.max_hints - self.hints_used
    }
}
```

**ui.rs - Enhanced user interface:**
```rust
use crate::{difficulty::Difficulty, stats::GameStats};
use std::io::{self, Write};

pub fn print_welcome(player_name: &str) {
    println!("🎲 Welcome back, {}! 🎲", player_name);
    println!();
}

pub fn display_stats(stats: &GameStats) {
    if stats.games_played == 0 {
        println!("📊 Ready for your first game!");
    } else {
        println!("📊 Your Stats:");
        println!("   Games Played: {} | Win Rate: {:.1}% | Best: {} attempts",
                 stats.games_played,
                 stats.win_rate(),
                 stats.best_attempts.map_or("N/A".to_string(), |a| a.to_string()));
        
        if stats.current_streak > 0 {
            println!("   Current Streak: {} wins 🔥", stats.current_streak);
        }
    }
    println!();
}

pub fn select_difficulty() -> Difficulty {
    loop {
        println!("🎮 Choose Difficulty:");
        println!("   1. Easy (1-50) - Extra hints available");
        println!("   2. Medium (1-100) - Standard game");
        println!("   3. Hard (1-200) - Challenge mode");
        print!("Enter choice (1-3): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        
        match input.trim() {
            "1" => return Difficulty::Easy,
            "2" => return Difficulty::Medium,
            "3" => return Difficulty::Hard,
            _ => println!("Please enter 1, 2, or 3.\n"),
        }
    }
}

pub fn ask_play_again() -> bool {
    loop {
        print!("Would you like to play again? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' for yes or 'n' for no."),
        }
    }
}

pub fn print_goodbye(stats: &GameStats) {
    println!("\n🎊 Final Statistics:");
    display_stats(stats);
    println!("Thanks for playing! Goodbye! 👋");
}
```

This complete implementation demonstrates:
- Modular architecture with clear separation of concerns
- Persistent data storage for statistics and configuration
- Intelligent hint system that adapts to gameplay
- Rich user interface with engaging feedback
- Professional error handling throughout
- Extensible design for future enhancements