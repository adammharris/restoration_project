#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use crate::config::GameConfig;

const IDEAL_LINE_LENGTH: usize = 80;
const MIN_MARGIN: usize = 4;

#[cfg(not(target_arch = "wasm32"))]
fn wrap_and_center_text(text: &str) -> Vec<String> {
    if let Some((width, _)) = terminal_size::terminal_size() {
        let terminal_width = width.0 as usize;
        let content_width = std::cmp::min(IDEAL_LINE_LENGTH, terminal_width.saturating_sub(MIN_MARGIN * 2));
        let margin = (terminal_width.saturating_sub(content_width)) / 2;
        
        let wrapped_lines = wrap_text(text, content_width);
        wrapped_lines.iter()
            .map(|line| format!("{}{}", " ".repeat(margin), line))
            .collect()
    } else {
        // Fallback without terminal size
        wrap_text(text, IDEAL_LINE_LENGTH)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print_game_text(text: &str, config: &GameConfig) {
    use crate::config::UiMode;
    use std::io::{self, Write};
    if config.ui_mode == UiMode::Centered {
        let lines = wrap_and_center_text(text);
        for line in lines {
            print_line(&line, config);
        }
    } else {
        print_line(text, config);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn print_line(text: &str, config: &GameConfig) {
    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;
    if config.enable_typewriter {
        // Print leading whitespace instantly, then animate the content
        let leading_spaces = text.len() - text.trim_start().len();
        if leading_spaces > 0 {
            print!("{}", &text[..leading_spaces]);
            io::stdout().flush().unwrap();
        }
        
        // Animate the actual content
        for c in text.trim_start().chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(config.typewriter_speed_ms));
        }
        println!();
    } else {
        println!("{}", text);
    }
}

// Keep the old function for backwards compatibility
#[cfg(not(target_arch = "wasm32"))]
pub fn print_typewriter_effect(text: &str, config: &GameConfig) {
    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;
    print_game_text(text, config);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_user_input(config: &GameConfig) -> Result<String, io::Error> {
    use std::io;
    use crate::config::UiMode;
    if config.ui_mode == UiMode::Centered {
        // Align with the choice margin
        let margin = get_content_margin();
        print!("{}{}", " ".repeat(margin), "> ");
    } else {
        print!("> ");
    }
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn wait_for_enter() {
    use std::io::{self, Write};
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn get_content_margin() -> usize {
    if let Some((width, _)) = terminal_size::terminal_size() {
        let terminal_width = width.0 as usize;
        let content_width = std::cmp::min(IDEAL_LINE_LENGTH, terminal_width.saturating_sub(MIN_MARGIN * 2));
        (terminal_width.saturating_sub(content_width)) / 2
    } else {
        0
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn display_choices(choices: &[&crate::world::Choice], config: &GameConfig) {
    for (i, choice) in choices.iter().enumerate() {
        let full_choice = format!("{}: {}", i + 1, choice.text);
        print_game_text(&full_choice, config);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_user_choice(input: &str, choices: &[&crate::world::Choice], config: &GameConfig) -> Option<usize> {
    let input = input.trim();
    
    // Try parsing as number first
    if let Ok(num) = input.parse::<usize>() {
        if num > 0 && num <= choices.len() {
            return Some(num - 1);
        }
    }
    
    // If text commands are enabled, try matching text
    if config.allow_text_commands {
        let input_lower = input.to_lowercase();
        
        for (i, choice) in choices.iter().enumerate() {
            let choice_text_lower = choice.text.to_lowercase();
            
            // Check for exact matches or partial matches
            if choice_text_lower.contains(&input_lower) || 
               input_lower.contains(&choice_text_lower) {
                return Some(i);
            }
            
            // Check for keyword matches (simple heuristic)
            let choice_words: Vec<&str> = choice_text_lower.split_whitespace().collect();
            let input_words: Vec<&str> = input_lower.split_whitespace().collect();
            
            for input_word in &input_words {
                if input_word.len() > 3 && choice_words.contains(input_word) {
                    return Some(i);
                }
            }
        }
        
        // Common command aliases
        match input_lower.as_str() {
            "quit" | "exit" | "q" => {
                for (i, choice) in choices.iter().enumerate() {
                    if choice.text.to_lowercase().contains("quit") || 
                       choice.actions.iter().any(|a| matches!(a, crate::world::Action::Quit)) {
                        return Some(i);
                    }
                }
            }
            "look" | "examine" | "search" => {
                for (i, choice) in choices.iter().enumerate() {
                    let text = choice.text.to_lowercase();
                    if text.contains("look") || text.contains("examine") || text.contains("search") {
                        return Some(i);
                    }
                }
            }
            "go" | "move" | "walk" => {
                for (i, choice) in choices.iter().enumerate() {
                    let text = choice.text.to_lowercase();
                    if text.contains("go") || text.contains("move") || text.contains("walk") {
                        return Some(i);
                    }
                }
            }
            _ => {}
        }
    }
    
    None
}