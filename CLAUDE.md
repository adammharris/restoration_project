# AGENTS.md - Codebase Overview for AI Assistants

This document provides an overview of the `restoration_project` codebase, a modular text adventure game written in Rust. It is designed to help other AI assistants quickly understand the project's structure, design principles, and key components.

## 1. Project Purpose

The `restoration_project` is a simple, extensible text adventure game engine. Its primary goal is to allow story creators to define game worlds and narratives using external TOML files, separating game logic from story content. This design facilitates easy creation and modification of new stories without requiring code changes.

## 2. Key Architectural Decisions

*   **Modular Design:** The core game engine (handling game loop, state management, input/output) is strictly separated from the story content.
*   **TOML for Story Data:** All game world data (rooms, choices, actions, conditions, flags) is defined in TOML files, making stories human-readable and easily editable.
*   **String-based IDs:** Room IDs and Choice IDs are represented as `String`s, allowing for more descriptive and memorable identifiers (e.g., `"cellar"`, `"log_in_computer"`) instead of numerical IDs.
*   **Typewriter Effect:** Text output (room descriptions and display texts) is presented with a character-by-character typewriter effect for enhanced immersion.
*   **Command-Line Story Selection:** The game can load different stories by specifying the TOML file path as a command-line argument.

## 3. File Structure

*   `Cargo.toml`: Project manifest, defines dependencies (e.g., `toml`, `serde`).
*   `src/main.rs`: Contains the main game engine logic, data structures, game loop, and core functions.
*   `the_cellar.toml`: A sample story file demonstrating the TOML format.
*   `story_template.toml`: A commented template file explaining how to write new stories in TOML.

## 4. How to Add New Stories

To add a new story:
1.  Create a new `.toml` file (e.g., `my_new_story.toml`) in the project root.
2.  Define your rooms, choices, and a `starting_room_id` within this file, following the structure of `story_template.toml`.
3.  Run the game using `cargo run -- my_new_story.toml`.

## 5. How to Run the Game

*   **Default Story (the_cellar.toml):** `cargo run`
*   **Specific Story:** `cargo run -- <story_file_name>.toml` (e.g., `cargo run -- story_template.toml`)

## 6. Important Structs and Enums (`src/main.rs`)

*   `RoomId(pub String)`: Newtype for string-based room identifiers.
*   `ChoiceId(pub String)`: Newtype for string-based choice identifiers.
*   `FlagId(pub String)`: Newtype for string-based game state flags.
*   `World`: Represents the entire game world, containing `rooms`, `choices`, and the `starting_room_id`.
*   `Room`: Defines a location with a `description` and a list of `choices` available from that room.
*   `Choice`: Defines an action with `text`, an optional `condition`, and a list of `actions` it triggers.
*   `Condition`: Enum for conditions that must be met for a choice to be available (currently `HasFlag`).
*   `Action`: Enum for actions triggered by a choice (e.g., `GoTo`, `SetFlag`, `Quit`, `DisplayText`).
*   `GameState`: Tracks the player's dynamic progress (current room, active flags, quit status).
*   `TomlWorld`: An intermediate struct used for deserializing the raw TOML data before converting it into the `World` struct.

## 7. Key Functions (`src/main.rs`)

*   `load_world_from_toml(path: &str) -> Result<World, Box<dyn std::error::Error>>`: Reads and parses a TOML file into a `World` struct.
*   `check_condition(choice: &Choice, game_state: &GameState) -> bool`: Determines if a choice's conditions are met by the current game state.
*   `execute_actions(choice: &Choice, game_state: &mut GameState)`: Executes all actions associated with a chosen choice, modifying the game state.
*   `print_typewriter_effect(text: &str)`: Prints text character by character with a delay, used for both room descriptions and `DisplayText` actions.
