# CLAUDE.md - AI Assistant Guide for Restoration Project

This document provides a comprehensive overview of the `restoration_project` codebase for AI assistants. The project has evolved significantly from its original TOML-based design to a modern, Markdown-based text adventure game engine.

## 1. Project Overview

**Restoration Project** is a sophisticated text adventure game engine written in Rust that uses **Markdown files** for story definition. It features a modern CLI, comprehensive error handling, save/load functionality, and advanced game mechanics like flags, counters, and conditional text.

### Key Features
- 🎮 **Markdown-based stories** - Natural, writer-friendly format
- 🖥️ **Modern CLI** with subcommands and configuration
- 💾 **Save/load system** with JSON persistence  
- 🎛️ **Advanced game mechanics** - flags, counters, conditional logic
- ✅ **Story validation** and statistics tools
- 🔧 **Configurable experience** - typewriter effects, text commands
- 📝 **Enhanced input parsing** - natural language commands

## 2. Current Architecture

### Modular Structure
```
src/
├── main.rs              # CLI interface and game orchestration
├── world.rs             # Story data structures and Markdown loading
├── game.rs              # Game state management and logic
├── ui.rs                # User interface and input handling
├── config.rs            # Configuration management
├── errors.rs            # Custom error types and handling
├── markdown_parser.rs   # Markdown story format parser
└── bin/
    └── story_validator.rs # Story validation tool
```

### Key Technologies
- **clap** - Modern CLI argument parsing
- **serde + serde_json** - Configuration and save data serialization
- **Custom Markdown parser** - Story format processing

## 3. Story Format (Markdown)

**IMPORTANT:** Stories are now written in **Markdown**, NOT TOML. The format is much more natural:

```markdown
# Story Title

@start room_id

## @room room_id
Room description text here.

### Choice text [optional_condition]
- "Display this text"
- flag+ some_flag
- @next_room
```

### Action Syntax
```markdown
- "Display text to player"
- @room_id              # Go to room (shorthand)
- goto: room_id         # Go to room (explicit)  
- flag+ flag_name       # Set flag
- flag- flag_name       # Remove flag
- count+ counter_name   # Increment counter
- count- counter_name   # Decrement counter
- count= counter_name 5 # Set counter value
- quit                  # End game
```

### Condition Syntax
```markdown
[flag_name]             # Has flag
[!flag_name]            # Does NOT have flag
[flag1 & flag2]         # Has ALL flags
[flag1 | flag2]         # Has ANY flags  
[counter > 5]           # Counter comparisons
[wisdom > 3 & has_wand] # Complex conditions
```

## 4. CLI Commands

The project now has a professional CLI:

```bash
# Play stories
cargo run                           # Default story
cargo run -- play story.md         # Specific story
cargo run -- play --fast story.md  # Skip typewriter effect

# Validate stories  
cargo run -- validate story.md

# Configuration management
cargo run -- config show
cargo run -- config set-speed 10
cargo run -- config set-typewriter false
```

## 5. Key Data Structures

### Core Types (src/world.rs)
```rust
pub struct World {
    pub rooms: HashMap<String, Room>,
    pub choices: HashMap<String, Choice>, 
    pub starting_room_id: String,
}

pub struct Room {
    pub description: String,
    pub choices: Vec<String>,
}

pub struct Choice {
    pub text: String,
    pub condition: Option<Condition>,
    pub actions: Vec<Action>,
}

pub enum Action {
    GoTo(String),
    SetFlag(FlagId),
    RemoveFlag(FlagId),
    Quit,
    DisplayText(String),
    IncrementCounter(String),
    DecrementCounter(String), 
    SetCounter(String, i32),
}

pub enum Condition {
    HasFlag(FlagId),
    NotHasFlag(FlagId),
    HasAllFlags(Vec<FlagId>),
    HasAnyFlags(Vec<FlagId>),
    CounterGreaterThan(String, i32),
    CounterLessThan(String, i32),
    CounterEquals(String, i32),
}
```

### Game State (src/game.rs)
```rust
pub struct GameState {
    pub current_room_id: String,
    pub flags: HashSet<FlagId>,
    pub counters: HashMap<String, i32>,
    pub has_quit: bool,
}
```

## 6. Important Functions

### Story Loading (src/world.rs)
- `load_world_from_markdown(path: &str) -> GameResult<World>` - Loads and validates Markdown stories

### Game Logic (src/game.rs)  
- `execute_actions(choice: &Choice, game_state: &mut GameState, config: &GameConfig)` - Executes choice actions
- `check_condition(choice: &Choice, game_state: &GameState) -> bool` - Evaluates choice conditions
- `get_available_choices(world: &World, game_state: &GameState) -> GameResult<Vec<&Choice>>` - Gets valid choices

### UI (src/ui.rs)
- `parse_user_choice(input: &str, choices: &[&Choice], config: &GameConfig) -> Option<usize>` - Smart input parsing
- `print_typewriter_effect(text: &str, config: &GameConfig)` - Configurable text display

## 7. How to Add New Features

### Adding New Actions
1. Add variant to `Action` enum in `src/world.rs`
2. Add parsing logic in `src/markdown_parser.rs` `parse_action()` function  
3. Add execution logic in `src/game.rs` `execute_actions()` function

### Adding New Conditions  
1. Add variant to `Condition` enum in `src/world.rs`
2. Add parsing logic in `src/markdown_parser.rs` `parse_condition()` function
3. Add evaluation logic in `src/game.rs` `check_single_condition()` function

## 8. Development Workflow

### Testing Stories
```bash
# Validate story structure
cargo run -- validate story.md

# Quick test with fast text
cargo run -- play --fast story.md  

# Full experience test
cargo run -- play story.md
```

### Building and Running
```bash
cargo build                    # Build project
cargo run -- --help          # See CLI help
cargo run -- play story.md   # Play a story
```

## 9. File Locations

### Stories
- `the_cellar.md` - Example story demonstrating features
- `story_template.md` - Template with syntax guide

### Configuration  
- `restoration_config.json` - User preferences (auto-created)

### Documentation
- `STORY_FORMAT_SPEC.md` - Complete format specification
- `CLI_USAGE.md` - CLI command guide
- `format_comparison.md` - Old vs new format comparison

## 10. Common Tasks

### Creating a New Story
1. Copy `story_template.md` as starting point
2. Validate with `cargo run -- validate your_story.md`
3. Test with `cargo run -- play --fast your_story.md`

### Debugging Story Issues  
1. Check validation output for missing references
2. Use `--fast` mode to quickly test changes
3. Check that actions use `- ` prefix (dash + space)

### Modifying Game Engine
- Core game logic is in `src/game.rs`
- Markdown parsing is in `src/markdown_parser.rs` 
- CLI interface is in `src/main.rs`
- Always run `cargo run -- validate story.md` after parser changes

## 11. Important Notes

⚠️ **CRITICAL:** Stories now use **Markdown format with dash-prefixed actions**, NOT TOML
⚠️ **Actions must start with `- ` (dash + space)** or they won't be parsed
⚠️ **Always validate stories** after editing to catch reference errors
⚠️ **Use `--fast` mode** during development to skip typewriter delays

The project has evolved into a professional-grade text adventure engine with modern tooling and excellent user experience.