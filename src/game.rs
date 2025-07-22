use std::collections::{HashSet, HashMap};
use std::fs;
use serde::{Serialize, Deserialize};
use crate::world::{FlagId, Choice, Action, Condition, World, Room};
use crate::ui::print_typewriter_effect;
use crate::errors::{GameError, GameResult};
use crate::config::GameConfig;

// --- Dynamic Game State ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_room_id: String,
    pub flags: HashSet<FlagId>,
    pub counters: HashMap<String, i32>,
    pub has_quit: bool,
}

impl GameState {
    pub fn new(starting_room_id: String) -> Self {
        GameState {
            current_room_id: starting_room_id,
            flags: HashSet::new(),
            counters: HashMap::new(),
            has_quit: false,
        }
    }
    
    pub fn save_to_file(&self, filename: &str) -> GameResult<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| GameError::ValidationError(format!("Failed to serialize save data: {}", e)))?;
        fs::write(filename, json)?;
        Ok(())
    }
    
    pub fn load_from_file(filename: &str) -> GameResult<Self> {
        let content = fs::read_to_string(filename)?;
        let game_state: GameState = serde_json::from_str(&content)
            .map_err(|e| GameError::ValidationError(format!("Failed to deserialize save data: {}", e)))?;
        Ok(game_state)
    }
    
    pub fn has_save_file(filename: &str) -> bool {
        std::path::Path::new(filename).exists()
    }
}

pub fn check_condition(choice: &Choice, game_state: &GameState) -> bool {
    match &choice.condition {
        None => true,
        Some(condition) => check_single_condition(condition, game_state),
    }
}

fn check_single_condition(condition: &Condition, game_state: &GameState) -> bool {
    match condition {
        Condition::HasFlag(flag_id) => game_state.flags.contains(flag_id),
        Condition::NotHasFlag(flag_id) => !game_state.flags.contains(flag_id),
        Condition::HasAllFlags(flags) => flags.iter().all(|flag| game_state.flags.contains(flag)),
        Condition::HasAnyFlags(flags) => flags.iter().any(|flag| game_state.flags.contains(flag)),
        Condition::CounterGreaterThan(counter, value) => {
            game_state.counters.get(counter).unwrap_or(&0) > value
        }
        Condition::CounterLessThan(counter, value) => {
            game_state.counters.get(counter).unwrap_or(&0) < value
        }
        Condition::CounterEquals(counter, value) => {
            game_state.counters.get(counter).unwrap_or(&0) == value
        }
        Condition::And(left, right) => {
            check_single_condition(left, game_state) && check_single_condition(right, game_state)
        }
        Condition::Or(left, right) => {
            check_single_condition(left, game_state) || check_single_condition(right, game_state)
        }
    }
}

pub fn execute_actions(choice: &Choice, game_state: &mut GameState, config: &GameConfig) {
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
                print_typewriter_effect(&format!("\n{}", text), config);
            }
            Action::DisplayTextConditional { condition, text_if_true, text_if_false } => {
                let text = if check_single_condition(condition, game_state) {
                    text_if_true
                } else {
                    text_if_false
                };
                print_typewriter_effect(&format!("\n{}", text), config);
            }
            Action::IncrementCounter(counter) => {
                let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                *game_state.counters.entry(counter.clone()).or_insert(0) += 1;
                let new_value = *game_state.counters.get(counter).unwrap();
                print_typewriter_effect(&format!("\n[{}: {} → {}]", counter, old_value, new_value), config);
            }
            Action::DecrementCounter(counter) => {
                let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                *game_state.counters.entry(counter.clone()).or_insert(0) -= 1;
                let new_value = *game_state.counters.get(counter).unwrap();
                print_typewriter_effect(&format!("\n[{}: {} → {}]", counter, old_value, new_value), config);
            }
            Action::SetCounter(counter, value) => {
                let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                game_state.counters.insert(counter.clone(), *value);
                print_typewriter_effect(&format!("\n[{}: {} → {}]", counter, old_value, value), config);
            }
        }
    }
}

pub fn get_available_choices<'a>(world: &'a World, game_state: &GameState) -> GameResult<Vec<&'a Choice>> {
    let current_room = world
        .rooms
        .get(&game_state.current_room_id)
        .ok_or_else(|| GameError::MissingRoom(game_state.current_room_id.clone()))?;

    let mut choices = Vec::new();
    for choice_id in &current_room.choices {
        let choice = world
            .choices
            .get(choice_id)
            .ok_or_else(|| GameError::MissingChoice(choice_id.clone()))?;
        
        if check_condition(choice, game_state) {
            choices.push(choice);
        }
    }
    
    Ok(choices)
}

pub fn get_room_description(room: &Room, game_state: &GameState) -> String {
    // Find the first matching conditional description
    for description in &room.descriptions {
        if let Some(condition) = &description.condition {
            if check_single_condition(condition, game_state) {
                return description.text.clone();
            }
        } else {
            // Unconditional description - use as fallback
            return description.text.clone();
        }
    }
    
    // No matching description found
    String::new()
}