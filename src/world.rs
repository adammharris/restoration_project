use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::{GameError, GameResult};

// --- ID Types for Type Safety ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RoomId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ChoiceId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
    NotHasFlag(FlagId),
    HasAllFlags(Vec<FlagId>),
    HasAnyFlags(Vec<FlagId>),
    CounterGreaterThan(String, i32),
    CounterLessThan(String, i32),
    CounterEquals(String, i32),
}

#[derive(Debug, Deserialize)]
pub enum Action {
    GoTo(String),
    SetFlag(FlagId),
    RemoveFlag(FlagId),
    Quit,
    DisplayText(String),
    DisplayTextConditional {
        condition: Condition,
        text_if_true: String,
        text_if_false: String,
    },
    IncrementCounter(String),
    DecrementCounter(String),
    SetCounter(String, i32),
}

// --- TOML Loading ---
#[derive(Debug, Deserialize)]
pub struct TomlWorld {
    pub rooms: HashMap<String, Room>,
    pub choices: HashMap<String, Choice>,
    pub starting_room_id: String,
}

pub fn load_world_from_toml(path: &str) -> GameResult<World> {
    let toml_str = std::fs::read_to_string(path)?;
    let toml_world: TomlWorld = toml::from_str(&toml_str)?;

    let world = World { 
        rooms: toml_world.rooms, 
        choices: toml_world.choices, 
        starting_room_id: toml_world.starting_room_id 
    };

    validate_world(&world)?;
    Ok(world)
}

fn validate_world(world: &World) -> GameResult<()> {
    // Check if starting room exists
    if !world.rooms.contains_key(&world.starting_room_id) {
        return Err(GameError::InvalidStartingRoom(world.starting_room_id.clone()));
    }

    // Check all room references in choices
    for (choice_id, choice) in &world.choices {
        for action in &choice.actions {
            if let Action::GoTo(room_id) = action {
                if !world.rooms.contains_key(room_id) {
                    return Err(GameError::MissingRoom(format!("Choice '{}' references missing room '{}'", choice_id, room_id)));
                }
            }
        }
    }

    // Check all choice references in rooms
    for (room_id, room) in &world.rooms {
        for choice_id in &room.choices {
            if !world.choices.contains_key(choice_id) {
                return Err(GameError::MissingChoice(format!("Room '{}' references missing choice '{}'", room_id, choice_id)));
            }
        }
    }

    // Note: We don't check for circular references as they're valid in adventure games
    // Players should be able to move back and forth between rooms

    Ok(())
}

