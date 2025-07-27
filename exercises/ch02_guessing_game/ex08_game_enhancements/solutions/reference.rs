use std::io::{self, Write};
use std::cmp::Ordering;
use std::fs;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn range(&self) -> (u32, u32) {
        match self {
            Difficulty::Easy => (1, 50),
            Difficulty::Medium => (1, 100),
            Difficulty::Hard => (1, 200),
        }
    }
    
    fn max_hints(&self) -> u32 {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Medium => 3,
            Difficulty::Hard => 1,
        }
    }
    
    fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}

#[derive(Debug, Clone)]
struct GameStats {
    games_played: u32,
    games_won: u32,
    total_attempts: u32,
    best_attempts: Option<u32>,
    current_streak: u32,
    best_streak: u32,
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
    fn record_win(&mut self, attempts: u32) {
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
    
    fn record_loss(&mut self) {
        self.games_played += 1;
        self.current_streak = 0;
    }
    
    fn win_rate(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.games_won as f64 / self.games_played as f64) * 100.0
        }
    }
    
    fn average_attempts(&self) -> f64 {
        if self.games_won == 0 {
            0.0
        } else {
            self.total_attempts as f64 / self.games_won as f64
        }
    }
    
    fn save_to_file(&self) {
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

struct HintSystem {
    previous_guesses: Vec<u32>,
    hints_used: u32,
    max_hints: u32,
}

impl HintSystem {
    fn new(difficulty: Difficulty) -> Self {
        HintSystem {
            previous_guesses: Vec::new(),
            hints_used: 0,
            max_hints: difficulty.max_hints(),
        }
    }
    
    fn add_guess(&mut self, guess: u32) {
        self.previous_guesses.push(guess);
    }
    
    fn can_give_hint(&self) -> bool {
        self.hints_used < self.max_hints
    }
    
    fn give_hint(&mut self, secret: u32, range: (u32, u32)) -> String {
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
    
    fn hints_remaining(&self) -> u32 {
        self.max_hints - self.hints_used
    }
}

fn main() {
    let mut stats = load_stats().unwrap_or_default();
    
    print_welcome();
    display_stats(&stats);
    
    loop {
        let difficulty = select_difficulty();
        play_enhanced_game(difficulty, &mut stats);
        
        stats.save_to_file();
        
        if !ask_play_again() {
            break;
        }
        
        println!("\n🎲 New game started! I'm thinking of a new number...\n");
    }
    
    print_goodbye(&stats);
}

fn print_welcome() {
    println!("🎲 Enhanced Number Guessing Game! 🎲");
    println!("Welcome to the most advanced guessing game experience!");
    println!();
}

fn display_stats(stats: &GameStats) {
    if stats.games_played == 0 {
        println!("📊 Ready for your first game!");
    } else {
        println!("📊 Your Statistics:");
        println!("   Games Played: {} | Win Rate: {:.1}% | Best: {} attempts",
                 stats.games_played,
                 stats.win_rate(),
                 stats.best_attempts.map_or("N/A".to_string(), |a| a.to_string()));
        
        if stats.current_streak > 0 {
            println!("   Current Streak: {} wins 🔥", stats.current_streak);
        }
        
        if stats.best_streak > 1 {
            println!("   Best Streak: {} wins 🏆", stats.best_streak);
        }
    }
    println!();
}

fn select_difficulty() -> Difficulty {
    loop {
        println!("🎮 Choose Your Difficulty:");
        println!("   1. Easy (1-50) - 5 hints available");
        println!("   2. Medium (1-100) - 3 hints available");
        println!("   3. Hard (1-200) - 1 hint available");
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

fn play_enhanced_game(difficulty: Difficulty, stats: &mut GameStats) {
    let range = difficulty.range();
    let secret_number = rand::thread_rng().gen_range(range.0..=range.1);
    let mut attempts = 0;
    let mut hint_system = HintSystem::new(difficulty);
    
    println!("\n🎯 {} difficulty selected. I'm thinking of a number between {} and {}.",
             difficulty.name(), range.0, range.1);
    println!("💡 Type 'hint' for help ({} hints available)", hint_system.max_hints);
    println!();
    
    loop {
        attempts += 1;
        
        print!("Attempt #{} - Enter your guess (or 'hint'): ", attempts);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again.");
            attempts -= 1;
            continue;
        }
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("hint") {
            attempts -= 1; // Don't count hint requests as attempts
            if hint_system.can_give_hint() {
                let hint = hint_system.give_hint(secret_number, range);
                println!("{}", hint);
                println!("💡 Hints remaining: {}\n", hint_system.hints_remaining());
            } else {
                println!("❌ No more hints available!\n");
            }
            continue;
        }
        
        if input.eq_ignore_ascii_case("quit") {
            println!("Game quit. The number was {}.", secret_number);
            stats.record_loss();
            return;
        }
        
        let guess: u32 = match input.parse() {
            Ok(num) => {
                if num < range.0 || num > range.1 {
                    println!("Please enter a number between {} and {}.\n", range.0, range.1);
                    attempts -= 1;
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please enter a valid number, 'hint', or 'quit'.\n");
                attempts -= 1;
                continue;
            }
        };
        
        hint_system.add_guess(guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too low! Try a larger number.");
                provide_warmth_feedback(&hint_system.previous_guesses, secret_number);
                println!();
            },
            Ordering::Greater => {
                println!("Too high! Try a smaller number.");
                provide_warmth_feedback(&hint_system.previous_guesses, secret_number);
                println!();
            },
            Ordering::Equal => {
                celebrate_victory(guess, attempts, difficulty);
                stats.record_win(attempts);
                check_achievements(stats, attempts);
                break;
            }
        }
    }
}

fn provide_warmth_feedback(guesses: &[u32], secret: u32) {
    if guesses.len() < 2 {
        return;
    }
    
    let last_guess = guesses[guesses.len() - 1];
    let previous_guess = guesses[guesses.len() - 2];
    
    let last_distance = if secret > last_guess {
        secret - last_guess
    } else {
        last_guess - secret
    };
    
    let previous_distance = if secret > previous_guess {
        secret - previous_guess
    } else {
        previous_guess - secret
    };
    
    if last_distance < previous_distance {
        println!("🔥 Getting warmer!");
    } else if last_distance > previous_distance {
        println!("🧊 Getting colder!");
    }
}

fn celebrate_victory(guess: u32, attempts: u32, difficulty: Difficulty) {
    println!("🎉 Congratulations! You guessed it correctly!");
    println!("You found the number {} in {} attempt{} on {} difficulty.",
             guess,
             attempts,
             if attempts == 1 { "" } else { "s" },
             difficulty.name());
    
    match (attempts, difficulty) {
        (1, _) => println!("🎯 INCREDIBLE! First try! You're a mind reader!"),
        (2..=3, Difficulty::Hard) => println!("🏆 AMAZING! Lightning fast on Hard mode!"),
        (2..=4, Difficulty::Medium) => println!("⭐ EXCELLENT! Great guessing strategy!"),
        (2..=5, Difficulty::Easy) => println!("👍 NICE! Quick and efficient!"),
        (_, Difficulty::Hard) => println!("💪 IMPRESSIVE! Hard mode conquered!"),
        (_, _) => println!("🎊 WELL DONE! Victory achieved!"),
    }
}

fn check_achievements(stats: &GameStats, attempts: u32) {
    let mut achievements = Vec::new();
    
    if stats.games_played == 1 {
        achievements.push("🌟 First Game - Welcome to the journey!");
    }
    
    if stats.games_won == 10 {
        achievements.push("🏅 Ten Victories - You're getting good at this!");
    }
    
    if stats.current_streak == 5 {
        achievements.push("🔥 Five in a Row - You're on fire!");
    }
    
    if attempts == 1 {
        achievements.push("🎯 Psychic - Got it on the first try!");
    }
    
    if stats.win_rate() >= 90.0 && stats.games_played >= 10 {
        achievements.push("🏆 Master Guesser - 90%+ win rate!");
    }
    
    for achievement in achievements {
        println!("🏆 ACHIEVEMENT UNLOCKED: {}", achievement);
    }
    
    if !achievements.is_empty() {
        println!();
    }
}

fn load_stats() -> Result<GameStats, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("game_stats.txt")?;
    let parts: Vec<&str> = data.trim().split(',').collect();
    
    if parts.len() != 6 {
        return Err("Invalid stats file format".into());
    }
    
    let best_attempts = if parts[3] == "None" {
        None
    } else {
        let clean_str = parts[3].replace("Some(", "").replace(")", "");
        Some(clean_str.parse()?)
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

fn ask_play_again() -> bool {
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

fn print_goodbye(stats: &GameStats) {
    println!("\n🎊 Final Session Statistics:");
    display_stats(stats);
    
    if stats.games_played > 0 {
        println!("🎮 Session Summary:");
        println!("   • Average attempts per win: {:.1}", stats.average_attempts());
        if stats.best_streak > 0 {
            println!("   • Best winning streak: {}", stats.best_streak);
        }
    }
    
    println!("\nThanks for playing the Enhanced Guessing Game! 👋");
    println!("Your statistics have been saved. See you next time! 🎲");
}