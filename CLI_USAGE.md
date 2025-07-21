# Restoration Project - CLI Usage Guide

The Restoration Project now has a modern, intuitive command-line interface powered by `clap`. Here's how to use it:

## Basic Usage

```bash
# Play the default story (the_cellar.md)
cargo run

# Play a specific story
cargo run story.md
cargo run -- play story.md

# Get help
cargo run -- --help
cargo run -- play --help
```

## Commands

### 🎮 Playing Stories

```bash
# Play a story (default command)
cargo run -- play story.md

# Play with instant text (no typewriter effect)
cargo run -- play --fast story.md

# Play with number-only commands (disable text parsing)
cargo run -- play --no-text-commands story.md

# Combine options
cargo run -- play --fast --no-text-commands story.md
```

### ✅ Validating Stories

```bash
# Validate a story file
cargo run -- validate story.md

# This checks:
# - Markdown syntax correctness
# - All room references exist
# - All choice references exist  
# - Starting room is valid
# - Story statistics and warnings
```

### ⚙️ Configuration Management

```bash
# Show current configuration
cargo run -- config show

# Reset to defaults
cargo run -- config reset

# Set typewriter speed (milliseconds per character)
cargo run -- config set-speed 50    # Slower
cargo run -- config set-speed 10    # Faster
cargo run -- config set-speed 0     # Instant

# Enable/disable typewriter effect
cargo run -- config set-typewriter true
cargo run -- config set-typewriter false

# Enable/disable text commands
cargo run -- config set-text-commands true   # Allow "look around"
cargo run -- config set-text-commands false  # Numbers only
```

## Examples

### Quick Play Session
```bash
# Fast play for testing
cargo run -- play --fast my_story.md
```

### Story Development Workflow
```bash
# 1. Validate your story
cargo run -- validate my_story.md

# 2. Play test with fast text
cargo run -- play --fast my_story.md

# 3. Final test with normal settings
cargo run -- play my_story.md
```

### Custom Configuration
```bash
# Set up for fast development
cargo run -- config set-typewriter false
cargo run -- config set-text-commands true

# Set up for immersive play
cargo run -- config set-typewriter true
cargo run -- config set-speed 20
```

## Configuration File

Settings are stored in `restoration_config.json`:
```json
{
  "typewriter_speed_ms": 30,
  "enable_typewriter": true,
  "allow_text_commands": true,  
  "auto_save": false
}
```

## In-Game Commands

Once playing, these special commands work in any room:
- `save` - Save your progress
- `load` - Load saved progress  
- `quit` / `exit` - Exit the game

## Command-Line Options Override Config

Options like `--fast` and `--no-text-commands` temporarily override your saved configuration for that session only.

## Help System

Every command has built-in help:
```bash
cargo run -- --help                    # Main help
cargo run -- play --help               # Play command help
cargo run -- validate --help           # Validate command help  
cargo run -- config --help             # Config command help
cargo run -- config set-speed --help   # Specific config help
```

## Tips

1. **Story Development**: Use `--fast` during development to quickly test changes
2. **Text Commands**: Try typing "look around" instead of numbers for a more immersive experience
3. **Validation**: Always validate your stories before sharing them
4. **Configuration**: Set your preferred defaults with the config commands
5. **Help**: Use `--help` on any command when you're unsure of the syntax