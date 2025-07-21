use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crate::config::GameConfig;

pub fn print_typewriter_effect(text: &str, config: &GameConfig) {
    if config.enable_typewriter {
        for c in text.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(config.typewriter_speed_ms));
        }
        println!();
    } else {
        println!("{}", text);
    }
}

pub fn get_user_input() -> Result<String, io::Error> {
    print!("> ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn display_choices(choices: &[&crate::world::Choice], config: &GameConfig) {
    for (i, choice) in choices.iter().enumerate() {
        print!("{}: ", i + 1);
        print_typewriter_effect(&choice.text, config);
    }
}

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