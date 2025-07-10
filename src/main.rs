use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// --- ID Types for Type Safety ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RoomId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ChoiceId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct FlagId(pub String);

// --- Static World Data ---
#[derive(Debug, Deserialize)]
pub struct World {
    pub rooms: HashMap<String, Room>,
    pub choices: HashMap<String, Choice>,
    pub starting_room_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub description: String,
    pub choices: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub text: String,
    pub condition: Option<Condition>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub enum Condition {
    HasFlag(FlagId),
}

#[derive(Debug, Deserialize)]
pub enum Action {
    GoTo(String),
    SetFlag(FlagId),
    Quit,
    DisplayText(String),
}

// --- Dynamic Game State ---
pub struct GameState {
    pub current_room_id: String,
    pub flags: HashSet<FlagId>,
    pub has_quit: bool,
}

impl GameState {
    pub fn new(starting_room_id: String) -> Self {
        GameState {
            current_room_id: starting_room_id,
            flags: HashSet::new(),
            has_quit: false,
        }
    }
}

// --- Game Logic ---

#[derive(Debug, Deserialize)]
pub struct TomlWorld {
    pub rooms: HashMap<String, Room>,
    pub choices: HashMap<String, Choice>,
    pub starting_room_id: String,
}

fn load_world_from_toml(path: &str) -> Result<World, Box<dyn std::error::Error>> {
    let toml_str = std::fs::read_to_string(path)?;
    let toml_world: TomlWorld = toml::from_str(&toml_str)?;

    let mut rooms = HashMap::new();
    for (key, value) in toml_world.rooms {
        rooms.insert(key, value);
    }

    let mut choices = HashMap::new();
    for (key, value) in toml_world.choices {
        choices.insert(key, value);
    }

    Ok(World { rooms, choices, starting_room_id: toml_world.starting_room_id })
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let story_file = if args.len() > 1 { &args[1] } else { "the_cellar.toml" };

    let world = match load_world_from_toml(story_file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error loading story: {}", e);
            return;
        }
    };
    let mut game_state = GameState::new(world.starting_room_id.clone());
    let mut last_room_id = game_state.current_room_id.clone(); // Track the last room

    println!("--- Welcome to the Restoration Project ---");

    // Initial room description
    let current_room = world
        .rooms
        .get(&game_state.current_room_id)
        .expect("Room not found!");
    println!("\n{}", current_room.description);

    while !game_state.has_quit {
        // Only print room description if we have changed rooms
        if last_room_id != game_state.current_room_id {
            let current_room = world
                .rooms
                .get(&game_state.current_room_id)
                .expect("Room not found!");
            println!("\n{}", current_room.description);
            last_room_id = game_state.current_room_id.clone();
        }

        let current_room = world
            .rooms
            .get(&game_state.current_room_id)
            .expect("Room not found!");

        let available_choices: Vec<&Choice> = current_room
            .choices
            .iter()
            .map(|id| world.choices.get(id).expect("Choice not found!"))
            .filter(|choice| check_condition(choice, &game_state))
            .collect();

        if available_choices.is_empty() {
            println!("There is nothing you can do here.");
            game_state.has_quit = true;
            continue;
        }

        for (i, choice) in available_choices.iter().enumerate() {
            println!("{}: {}", i + 1, choice.text);
        }

        print!("> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }

        let choice_index: Result<usize, _> = input.trim().parse();

        match choice_index {
            Ok(idx) if idx > 0 && idx <= available_choices.len() => {
                let choice = available_choices[idx - 1];
                execute_actions(choice, &mut game_state);
            }
            _ => {
                println!("That's not a valid choice.");
            }
        }
    }

    println!("\nThank you for playing!");
}

fn check_condition(choice: &Choice, game_state: &GameState) -> bool {
    match &choice.condition {
        None => true,
        Some(condition) => match condition {
            Condition::HasFlag(flag_id) => game_state.flags.contains(flag_id),
        },
    }
}

fn execute_actions(choice: &Choice, game_state: &mut GameState) {
    for action in &choice.actions {
        match action {
            Action::GoTo(room_id) => game_state.current_room_id = room_id.clone(),
            Action::SetFlag(flag_id) => {
                game_state.flags.insert(flag_id.clone());
            }
            Action::Quit => game_state.has_quit = true,
            Action::DisplayText(text) => {
                print!("\n");
                for c in text.chars() {
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_millis(30)); // Adjust delay as needed
                }
                println!();
            },
        }
    }
}