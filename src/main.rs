mod world;
mod game;
mod ui;
mod errors;
mod config;

use std::env;
use world::load_world_from_toml;
use game::{GameState, get_available_choices, execute_actions};
use ui::{print_typewriter_effect, get_user_input, display_choices, parse_user_choice};
use config::GameConfig;

fn main() {
    let args: Vec<String> = env::args().collect();
    let story_file = if args.len() > 1 { &args[1] } else { "the_cellar.toml" };

    let config = match GameConfig::load_or_create() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    let world = match load_world_from_toml(story_file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error loading story: {}", e);
            return;
        }
    };
    let save_filename = format!("{}.save", story_file.replace(".toml", ""));
    
    let mut game_state = if GameState::has_save_file(&save_filename) {
        println!("Found save file. Load it? (y/n)");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() && input.trim().eq_ignore_ascii_case("y") {
            match GameState::load_from_file(&save_filename) {
                Ok(state) => {
                    println!("Game loaded successfully!");
                    state
                }
                Err(e) => {
                    eprintln!("Failed to load save file: {}", e);
                    println!("Starting new game...");
                    GameState::new(world.starting_room_id.clone())
                }
            }
        } else {
            GameState::new(world.starting_room_id.clone())
        }
    } else {
        GameState::new(world.starting_room_id.clone())
    };
    
    let mut last_room_id = game_state.current_room_id.clone(); // Track the last room

    println!("--- Welcome to the Restoration Project ---");
    println!("Special commands: 'save' to save game, 'load' to load game, 'quit' to exit");

    // Initial room description
    let current_room = match world.rooms.get(&game_state.current_room_id) {
        Some(room) => room,
        None => {
            eprintln!("Error: Starting room '{}' not found!", game_state.current_room_id);
            return;
        }
    };
    print_typewriter_effect(&format!("\n{}", current_room.description), &config);

    while !game_state.has_quit {
        // Only print room description if we have changed rooms
        if last_room_id != game_state.current_room_id {
            let current_room = match world.rooms.get(&game_state.current_room_id) {
                Some(room) => room,
                None => {
                    eprintln!("Error: Room '{}' not found!", game_state.current_room_id);
                    return;
                }
            };
            print_typewriter_effect(&format!("\n{}", current_room.description), &config);
            last_room_id = game_state.current_room_id.clone();
        }

        let available_choices = match get_available_choices(&world, &game_state) {
            Ok(choices) => choices,
            Err(e) => {
                eprintln!("Error getting choices: {}", e);
                return;
            }
        };

        if available_choices.is_empty() {
            println!("There is nothing you can do here.");
            game_state.has_quit = true;
            continue;
        }

        display_choices(&available_choices, &config);

        let input = match get_user_input() {
            Ok(input) => input,
            Err(_) => {
                println!("Error reading input.");
                continue;
            }
        };

        // Handle special commands first
        match input.trim().to_lowercase().as_str() {
            "save" => {
                match game_state.save_to_file(&save_filename) {
                    Ok(()) => println!("Game saved successfully!"),
                    Err(e) => eprintln!("Failed to save game: {}", e),
                }
                continue;
            }
            "load" => {
                match GameState::load_from_file(&save_filename) {
                    Ok(loaded_state) => {
                        game_state = loaded_state;
                        last_room_id = "".to_string(); // Force room description to show
                        println!("Game loaded successfully!");
                    }
                    Err(e) => eprintln!("Failed to load game: {}", e),
                }
                continue;
            }
            "quit" | "exit" => {
                game_state.has_quit = true;
                continue;
            }
            _ => {}
        }

        match parse_user_choice(&input, &available_choices, &config) {
            Some(choice_index) => {
                let choice = available_choices[choice_index];
                execute_actions(choice, &mut game_state, &config);
            }
            None => {
                if config.allow_text_commands {
                    println!("I don't understand '{}'. Try typing a number or describing your action.", input);
                } else {
                    println!("That's not a valid choice number.");
                }
            }
        }
    }

    println!("\nThank you for playing!");
}

