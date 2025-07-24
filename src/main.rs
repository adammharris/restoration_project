mod world;
mod game;
mod ui;
mod errors;
mod config;
mod markdown_parser;
mod terminal_ui;

use clap::{Parser, Subcommand};
use world::load_world_from_markdown;
use game::{GameState, get_available_choices, execute_actions, get_room_description};
use ui::{print_typewriter_effect, get_user_input, display_choices, parse_user_choice, print_game_text};
use terminal_ui::TerminalUi;
use config::{GameConfig, UiMode};

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
    /// Enable typewriter effect
    EnableTypewriter,
    /// Disable typewriter effect  
    DisableTypewriter,
    /// Enable text commands
    EnableTextCommands,
    /// Disable text commands
    DisableTextCommands,
    /// Enable centered UI mode
    EnableUiMode,
    /// Disable centered UI mode (plain mode)
    DisableUiMode,
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

    match config.ui_mode {
        UiMode::Centered => play_story_terminal_ui(story_file, world, config),
        UiMode::Plain => play_story_plain(story_file, world, config),
    }
}

fn play_story_plain(story_file: &str, world: world::World, config: GameConfig) {
    let save_filename = format!("{}.save", story_file.replace(".md", ""));
    
    let mut game_state = if GameState::has_save_file(&save_filename) {
        print_game_text("Found save file. Load it? (y/n)", &config);
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() && input.trim().eq_ignore_ascii_case("y") {
            match GameState::load_from_file(&save_filename) {
                Ok(state) => {
                    print_game_text("Game loaded successfully!", &config);
                    state
                }
                Err(e) => {
                    eprintln!("Failed to load save file: {}", e);
                    print_game_text("Starting new game...", &config);
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

    //print_game_line("--- Welcome to the Restoration Project ---", &config);
    //print_game_text("Special commands: 'save' to save game, 'load' to load game, 'quit' to exit", &config);

    // Initial room description
    let current_room = match world.rooms.get(&game_state.current_room_id) {
        Some(room) => room,
        None => {
            eprintln!("Error: Starting room '{}' not found!", game_state.current_room_id);
            return;
        }
    };
    let room_desc = get_room_description(current_room, &game_state);
    print_typewriter_effect(&format!("\n{}", room_desc), &config);

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
            let room_desc = get_room_description(current_room, &game_state);
            print_typewriter_effect(&format!("\n{}", room_desc), &config);
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
            print_game_text("There is nothing you can do here.", &config);
            game_state.has_quit = true;
            continue;
        }

        display_choices(&available_choices, &config);

        let input = match get_user_input(&config) {
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
                    Ok(()) => print_game_text("Game saved successfully!", &config),
                    Err(e) => eprintln!("Failed to save game: {}", e),
                }
                continue;
            }
            "load" => {
                match GameState::load_from_file(&save_filename) {
                    Ok(loaded_state) => {
                        game_state = loaded_state;
                        last_room_id = "".to_string(); // Force room description to show
                        print_game_text("Game loaded successfully!", &config);
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
                    print_game_text(&format!("I don't understand '{}'. Try typing a number or describing your action.", input), &config);
                } else {
                    print_game_text("That's not a valid choice number.", &config);
                }
            }
        }
    }

    print_game_text("\nThank you for playing!", &config);
}

fn play_story_terminal_ui(story_file: &str, world: world::World, config: GameConfig) {
    let mut terminal_ui = match TerminalUi::new(config.clone()) {
        Ok(ui) => ui,
        Err(e) => {
            eprintln!("Failed to initialize terminal UI: {}", e);
            return;
        }
    };

    let save_filename = format!("{}.save", story_file.replace(".md", ""));
    
    let mut game_state = if GameState::has_save_file(&save_filename) {
        terminal_ui.display_text("Found save file. Load it? (y/n)");
        match terminal_ui.get_input() {
            Ok(input) if input.trim().eq_ignore_ascii_case("y") => {
                match GameState::load_from_file(&save_filename) {
                    Ok(state) => {
                        terminal_ui.display_text("Game loaded successfully!");
                        state
                    }
                    Err(e) => {
                        terminal_ui.display_text(&format!("Failed to load save file: {}", e));
                        terminal_ui.display_text("Starting new game...");
                        GameState::new(world.starting_room_id.clone())
                    }
                }
            }
            _ => GameState::new(world.starting_room_id.clone()),
        }
    } else {
        GameState::new(world.starting_room_id.clone())
    };
    
    let mut last_room_id = game_state.current_room_id.clone();

    terminal_ui.display_text("--- Welcome to the Restoration Project ---");
    terminal_ui.display_text("Special commands: 'save' to save game, 'load' to load game, 'quit' to exit");

    // Initial room description
    let current_room = match world.rooms.get(&game_state.current_room_id) {
        Some(room) => room,
        None => {
            terminal_ui.display_text(&format!("Error: Starting room '{}' not found!", game_state.current_room_id));
            let _ = terminal_ui.cleanup();
            return;
        }
    };
    let room_desc = get_room_description(current_room, &game_state);
    terminal_ui.display_text(&format!("\n{}", room_desc));

    while !game_state.has_quit {
        // Only display room description if we have changed rooms
        if last_room_id != game_state.current_room_id {
            let current_room = match world.rooms.get(&game_state.current_room_id) {
                Some(room) => room,
                None => {
                    terminal_ui.display_text(&format!("Error: Room '{}' not found!", game_state.current_room_id));
                    break;
                }
            };
            let room_desc = get_room_description(current_room, &game_state);
            terminal_ui.clear_text();
            terminal_ui.display_text(&format!("\n{}", room_desc));
            last_room_id = game_state.current_room_id.clone();
        }

        let available_choices = match get_available_choices(&world, &game_state) {
            Ok(choices) => choices,
            Err(e) => {
                terminal_ui.display_text(&format!("Error getting choices: {}", e));
                break;
            }
        };

        if available_choices.is_empty() {
            terminal_ui.display_text("There is nothing you can do here.");
            game_state.has_quit = true;
            continue;
        }

        terminal_ui.display_choices(&available_choices);

        let input = match terminal_ui.get_input() {
            Ok(input) => input,
            Err(_) => {
                terminal_ui.display_text("Error reading input.");
                continue;
            }
        };

        // Handle special commands first
        match input.trim().to_lowercase().as_str() {
            "save" => {
                match game_state.save_to_file(&save_filename) {
                    Ok(()) => terminal_ui.display_text("Game saved successfully!"),
                    Err(e) => terminal_ui.display_text(&format!("Failed to save game: {}", e)),
                }
                continue;
            }
            "load" => {
                match GameState::load_from_file(&save_filename) {
                    Ok(loaded_state) => {
                        game_state = loaded_state;
                        last_room_id = "".to_string(); // Force room description to show
                        terminal_ui.display_text("Game loaded successfully!");
                    }
                    Err(e) => terminal_ui.display_text(&format!("Failed to load game: {}", e)),
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
                // Execute actions and capture any text output for terminal UI
                execute_actions_terminal_ui(choice, &mut game_state, &mut terminal_ui);
            }
            None => {
                if config.allow_text_commands {
                    terminal_ui.display_text(&format!("I don't understand '{}'. Try typing a number or describing your action.", input));
                } else {
                    terminal_ui.display_text("That's not a valid choice number.");
                }
            }
        }
    }

    terminal_ui.display_text("\nThank you for playing!");
    let _ = terminal_ui.cleanup();
}

fn execute_actions_terminal_ui(choice: &world::Choice, game_state: &mut game::GameState, terminal_ui: &mut terminal_ui::TerminalUi) {
    use crate::world::Action;
    use crate::game::check_single_condition;
    
    for action in &choice.actions {
        match action {
            Action::GoTo(room_id) => game_state.current_room_id = room_id.clone(),
            Action::SetFlag(flag_id) => {
                game_state.flags.insert(flag_id.clone());
            }
            Action::RemoveFlag(flag_id) => {
                game_state.flags.remove(flag_id);
            }
            Action::Quit => game_state.has_quit = true,
            Action::DisplayText(text) => {
                terminal_ui.display_text(text);
            }
            Action::DisplayTextConditional { condition, text_if_true, text_if_false } => {
                let text = if check_single_condition(condition, game_state) {
                    text_if_true
                } else {
                    text_if_false
                };
                terminal_ui.display_text(text);
            }
            Action::IncrementCounter(counter) => {
                let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                *game_state.counters.entry(counter.clone()).or_insert(0) += 1;
                let new_value = *game_state.counters.get(counter).unwrap();
                terminal_ui.display_text(&format!("[{}: {} → {}]", counter, old_value, new_value));
            }
            Action::DecrementCounter(counter) => {
                let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                *game_state.counters.entry(counter.clone()).or_insert(0) -= 1;
                let new_value = *game_state.counters.get(counter).unwrap();
                terminal_ui.display_text(&format!("[{}: {} → {}]", counter, old_value, new_value));
            }
            Action::SetCounter(counter, value) => {
                let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                game_state.counters.insert(counter.clone(), *value);
                terminal_ui.display_text(&format!("[{}: {} → {}]", counter, old_value, value));
            }
        }
    }
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
                    println!("  UI mode: {:?}", config.ui_mode);
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
        ConfigAction::EnableTypewriter => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.enable_typewriter = true;
                    match config.save() {
                        Ok(()) => println!("✅ Typewriter effect enabled"),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::DisableTypewriter => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.enable_typewriter = false;
                    match config.save() {
                        Ok(()) => println!("✅ Typewriter effect disabled"),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::EnableTextCommands => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.allow_text_commands = true;
                    match config.save() {
                        Ok(()) => println!("✅ Text commands enabled"),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::DisableTextCommands => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.allow_text_commands = false;
                    match config.save() {
                        Ok(()) => println!("✅ Text commands disabled"),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::EnableUiMode => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.ui_mode = UiMode::Centered;
                    match config.save() {
                        Ok(()) => println!("✅ Centered UI mode enabled"),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
        ConfigAction::DisableUiMode => {
            match GameConfig::load_or_create() {
                Ok(mut config) => {
                    config.ui_mode = UiMode::Plain;
                    match config.save() {
                        Ok(()) => println!("✅ Plain UI mode enabled"),
                        Err(e) => eprintln!("❌ Failed to save config: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading config: {}", e),
            }
        }
    }
}

