# Restoration Project - Markdown Story Format Specification

## Overview

This specification defines a Markdown-like format for writing interactive stories that compiles to the native TOML format. The goal is to make story writing as natural as possible while supporting all engine features.

## File Structure

```
# Story Title

@start room_id

## @room room_id
Room description text here.

### Choice text [condition]
	Action 1
	Action 2
	Action 3

### Another choice
	Single action
```

## Basic Elements

### 1. Story Header
```markdown
# The Cellar Adventure
@start cellar
```
- `# Title` - Story title (optional, for documentation)
- `@start room_id` - Defines the starting room

### 2. Room Definitions
```markdown
## @room cellar
You are in a dusty cellar. It's dark and smells of old wood.
```
- `## @room room_id` - Room header with unique ID
- Following paragraph(s) - Room description text
- Room IDs must be valid identifiers (letters, numbers, underscores)

### 3. Choice Definitions
```markdown
### Try to open the door
- "The door is locked solid."

### Look around the room
- "You find an old key!"
- flag+ has_key
```
- `### Choice text` - What the player sees in the choice menu
- `	action` - Actions that happen when choice is selected (dash-prefixed (- ))
- Multiple actions can be listed with multiple dash-prefixed (- ) lines

## Actions

### Display Text
```markdown
- "Text to display to the player"
- say: Alternative syntax for display text
```

### Movement
```markdown
	goto: room_id
	@room_id            # Shorthand syntax
```

### Flags
```markdown
	flag+ flag_name     # Set flag
	flag- flag_name     # Remove flag
```

### Counters
```markdown
	count+ counter_name      # Increment counter
	count- counter_name      # Decrement counter  
	count= counter_name 5    # Set counter to specific value
```

### Conditional Text
```markdown
	if has_key: "You have the key!" else: "You need a key."
	if wisdom > 3: "You are wise!" else: "Study more."
```

### Quit Game
```markdown
	quit
	end
```

## Conditions

Conditions are specified in square brackets after choice text:

### Flag Conditions
```markdown
### Unlock the door [has_key]                    # HasFlag
### Try without key [!has_key]                   # NotHasFlag  
### Use magic [has_wand & knows_spell]           # HasAllFlags
### Cast spell [fire_magic | ice_magic]          # HasAnyFlags
```

### Counter Conditions
```markdown
### Advanced training [wisdom > 2]               # CounterGreaterThan
### Basic training [wisdom < 5]                  # CounterLessThan
### Perfect score [score = 100]                  # CounterEquals
```

### Complex Conditions
```markdown
### Master spell [has_wand & (wisdom > 5 | experience = 10)]
```

## Operators

### Flag Operators
- `&` or `and` - All flags must be present (HasAllFlags)
- `|` or `or` - Any flag must be present (HasAnyFlags)  
- `!` - Flag must NOT be present (NotHasFlag)

### Counter Operators
- `>` - Greater than
- `<` - Less than
- `=` - Equals
- `>=` - Greater than or equal
- `<=` - Less than or equal
- `!=` - Not equal

### Logical Operators
- `()` - Grouping for complex conditions
- `&` - AND operation
- `|` - OR operation

## Complete Example

```markdown
# The Magic Academy

@start entrance

## @room entrance
You stand before a mystical academy. A wise instructor greets you with a knowing smile.
Your magical training begins here!

### Begin basic training [!basic_complete]
	"You practice basic spells and feel your power growing!"
	flag+ basic_complete
	count+ magic_power

### Check your progress
	if magic_power = 0: "You haven't started training yet." else: "You're making progress!"

### Attempt advanced training [basic_complete]
	"You delve into complex magical techniques!"
	count+ magic_power
	flag+ advanced_ready

### Take graduation exam [advanced_ready & magic_power > 2]
	"You demonstrate mastery of the magical arts!"
	goto: graduation

## @room graduation
Congratulations! You have become a true wizard!

### Celebrate your achievement
	"You've mastered the arcane arts. Your journey is complete!"
	quit
```

## Comments

```markdown
<!-- This is a comment and will be ignored -->

### Choice text
	"Action"  # Inline comments are also supported
```

## Validation Rules

1. **Room IDs** must be unique and valid identifiers
2. **Starting room** must exist in the story
3. **Room references** in `goto:` actions must exist
4. **Flag names** must be valid identifiers
5. **Counter names** must be valid identifiers
6. **Conditions** must have valid syntax
7. **Actions** must have valid syntax

## Error Handling

The compiler will provide helpful error messages:
- Line numbers for syntax errors
- Missing room references
- Invalid condition syntax
- Malformed actions
- Circular reference warnings (optional)

## Compilation

```bash
# Compile markdown story to TOML
cargo run --bin story_compiler story.md --output story.toml

# Validate markdown story  
cargo run --bin story_compiler story.md --validate-only

# Compile and run immediately
cargo run --bin story_compiler story.md --run
```

## Benefits of This Format

1. **Natural writing flow** - Stories read like documentation
2. **Visual clarity** - Easy to see story structure and flow
3. **Familiar syntax** - Uses Markdown conventions writers know
4. **Compact** - Much less verbose than raw TOML
5. **Error-resistant** - Harder to make syntax mistakes
6. **Self-documenting** - Comments and structure make intent clear
7. **Version control friendly** - Diffs are meaningful and readable

## Migration Path

Existing TOML stories remain fully supported. The Markdown format compiles to the same TOML structure, so both formats can coexist.