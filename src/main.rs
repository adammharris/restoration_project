mod world;
mod game;
mod ui;
mod errors;
mod config;
mod markdown_parser;

use clap::{Parser, Subcommand};
use world::load_world_from_markdown;
use game::{GameState, get_available_choices, execute_actions};
use ui::{print_typewriter_effect, get_user_input, display_choices, parse_user_choice};
use config::GameConfig;

#[derive(Parser)]
#[command(name = "restoration")]
#[command(about = "A text adventure game engine with Markdown story format")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Play a story (default command)
    Play {
        /// Story file to play
        #[arg(value_name = "STORY", env = "RESTORATION_STORY")]
        story: Option<String>,
        
        /// Skip typewriter effect (fast text)
        #[arg(long, env = "RESTORATION_FAST")]
        fast: bool,
        
        /// Disable text commands (numbers only)
        #[arg(long, env = "RESTORATION_NO_TEXT")]
        no_text_commands: bool,
    },
    
    /// Validate a story file
    Validate {
        /// Story file to validate
        #[arg(value_name = "STORY")]
        story: String,
    },
    
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,
    /// Reset configuration to defaults
    Reset,
    /// Set typewriter speed (0 = instant, higher = slower)
    SetSpeed { speed: u64 },
    /// Enable/disable typewriter effect
    SetTypewriter { enabled: bool },
    /// Enable/disable text commands
    SetTextCommands { enabled: bool },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Play { story, fast, no_text_commands }) => {
            let story_file = story.as_deref().unwrap_or("the_cellar.md");
            play_story(story_file, fast, no_text_commands);
        }
        Some(Commands::Validate { story }) => {
            validate_story(&story);
        }
        Some(Commands::Config { action }) => {
            handle_config(action);
        }
        None => {
            // Default: play with first argument as story file
            let story_arg = std::env::args().nth(1);
            let story_file = story_arg.as_deref().unwrap_or("the_cellar.md");
            play_story(story_file, false, false);
        }
    }
}

fn play_story(story_file: &str, fast: bool, no_text_commands: bool) {
    let mut config = match GameConfig::load_or_create() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };
    
    // Override config with command line options
    if fast {
        config.enable_typewriter = false;
    }
    if no_text_commands {
        config.allow_text_commands = false;
    }

    let world = match load_world_from_markdown(story_file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error loading story: {}", e);
            return;
        }
    };
    let save_filename = format!("{}.save", story_file.replace(".md", ""));
    
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

fn validate_story(story_file: &str) {
    match load_world_from_markdown(story_file) {
        Ok(_) => {
            println!("✅ Story '{}' is valid!", story_file);
            // Run the detailed validation from story_validator
            std::process::Command::new("cargo")
                .args(&["run", "--bin", "story_validator", story_file])
                .status()
                .expect("Failed to run story validator");
        }
        Err(e) => {
            eprintln!("❌ Story '{}' is invalid: {}", story_file, e);
            std::process::exit(1);
        }
    }
}

fn handle_config(action: ConfigAction) {
    match action {
        ConfigAction::Show => {
            match GameConfig::load_or_create() {
                Ok(config) => {
                    println!("Current Configuration:");
                    println!("  Typewriter enabled: {}", config.enable_typewriter);
                    println!("  Typewriter speed: {} ms", config.typewriter_speed_ms);
                    println!("  Text commands: {}", config.allow_text_commands);
                    println!("  Auto save: {}", config.auto_save);
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::Reset => {
            let default_config = GameConfig::default();
            match default_config.save() {
                Ok(()) => println!("✅ Configuration reset to defaults"),
                Err(e) => eprintln!("❌ Failed to reset config: {}", e),
            }
        }
        ConfigAction::SetSpeed { speed } => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.typewriter_speed_ms = speed;
                    match config.save() {
                        Ok(()) => println!("✅ Typewriter speed set to {} ms", speed),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::SetTypewriter { enabled } => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.enable_typewriter = enabled;
                    match config.save() {
                        Ok(()) => println!("✅ Typewriter effect {}", if enabled { "enabled" } else { "disabled" }),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::SetTextCommands { enabled } => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.allow_text_commands = enabled;
                    match config.save() {
                        Ok(()) => println!("✅ Text commands {}", if enabled { "enabled" } else { "disabled" }),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
    }
}

