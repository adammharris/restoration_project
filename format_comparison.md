# Format Comparison: TOML vs Markdown

## Current TOML Format (Verbose)

```toml
starting_room_id = "cellar"

[rooms."cellar"]
description = "You are in a dusty cellar. It's dark and smells of old wood. A single wooden door is to the north. It looks locked."
choices = ["try_door", "look_around", "unlock_door", "go_north"]

[rooms."hallway"]
description = "You are in a long, narrow hallway. The cellar door is to the south."
choices = ["go_south"]

[choices."try_door"]
text = "Try to open the north door."
actions = [{ DisplayText = "The door is locked. It won't budge." }]

[choices."look_around"]
text = "Look around in the dust."
actions = [
    { DisplayText = "You rummage through some old crates and find a rusty key!" },
    { SetFlag = "has_key" },
]

[choices."unlock_door"]
text = "Unlock the north door with the rusty key."
condition = { HasFlag = "has_key" }
actions = [
    { DisplayText = "The key fits! You turn the lock with a loud *click*." },
    { SetFlag = "door_unlocked" },
]

[choices."go_north"]
text = "Go north through the unlocked door."
condition = { HasFlag = "door_unlocked" }
actions = [{ GoTo = "hallway" }]

[choices."go_south"]
text = "Go south back to the cellar."
actions = [{ GoTo = "cellar" }]
```

## New Markdown Format (Clean & Readable)

```markdown
# The Cellar

@start cellar

## @room cellar
You are in a dusty cellar. It's dark and smells of old wood. A single wooden door is to the north. It looks locked.

### Try to open the north door
- "The door is locked. It won't budge."

### Look around in the dust
- "You rummage through some old crates and find a rusty key!"
- flag+ has_key

### Unlock the north door with the rusty key [has_key]
- "The key fits! You turn the lock with a loud *click*."
- flag+ door_unlocked

### Go north through the unlocked door [door_unlocked]
- @hallway

## @room hallway
You are in a long, narrow hallway. The cellar door is to the south.

### Go south back to the cellar
- @cellar
```

## Benefits of Markdown Format

- **73% less verbose** (39 lines → 24 lines)
- **Natural flow** - Read top to bottom like a story
- **Visual clarity** - See room connections immediately  
- **Familiar syntax** - Uses Markdown conventions
- **Fewer errors** - Less complex syntax to get wrong
- **Self-documenting** - Story structure is obvious