# The Cellar

@start cellar

## @room cellar
[!door_unlocked]
You are in a dusty cellar. It's dark and smells of old wood. A single wooden door is to the north. It looks locked.

[door_unlocked]
You are in a dusty cellar. It's dark and smells of old wood. A single wooden door is to the north. It is unlocked.

### Try to open the north door [!has_key]
- "The door is locked. It won't budge."

### Look around in the dust [!has_key]
- "You rummage through some old crates and find a rusty key!"
- flag+ has_key

### Unlock the north door with the rusty key [has_key & !door_unlocked]
- "The key fits! You turn the lock with a loud *click*."
- flag+ door_unlocked

### Go north through the unlocked door [door_unlocked]
- @hallway

## @room hallway
You are in a long, narrow hallway. The cellar door is to the south.

### Go south back to the cellar
- @cellar

### Go down the hallway
- end