# Story Template

<!-- This is a template showing how to write interactive stories using Markdown format -->
<!-- Comments are ignored by the parser -->

@start office

## @room office
You are in a small, windowless office cubicle. Your computer screen glows softly. There's a door to the hallway to the north.

### Look at the computer screen
- "The screen shows a login prompt. You'll need a password."

### Open the desk drawer
- "You find a sticky note inside. It reads: 'password123'"
- flag+ has_password

### Check the sad-looking office plant [knows_about_keycard]
- "Tucked into the soil, you find a plastic keycard!"
- flag+ has_keycard

### Log into the computer [has_password]
- "You log in successfully! An email from HR says 'The keycard for the main door has been placed in the pot of the ficus plant for safekeeping.'"
- flag+ knows_about_keycard

### Go north to the hallway
- @hallway

## @room hallway
You are in a sterile-looking office hallway. The exit is to the north, and your cubicle is to the south.

### Try to open the exit door
- "It's locked. A card reader is blinking next to it."

### Use the keycard on the exit door [has_keycard]
- "The card reader beeps and the lock clicks open."
- @outside

### Go south back to your cubicle
- @office

## @room outside
You've made it outside! The city air feels fresh. You are free.

### Celebrate your escape
- "You do a little dance. You've earned it!"

### Go home
- "Time to head home after a successful escape!"
- quit

<!-- 
SYNTAX REFERENCE:

Story Structure:
  # Story Title                     - Optional title
  @start room_id                    - Starting room (required)
  
Rooms:  
  ## @room room_id                  - Room definition
  Description text here.            - Room description (can be multiple paragraphs)
  
Choices:
  ### Choice text [condition]       - Choice with optional condition
- action                            - Actions (multiple tab-indented lines)

Actions (tab-indented):
- "Display text"                  - Show text to player
- say: Alternative text syntax    - Alternative display syntax
- @room_id                        - Go to room (shorthand)
- goto: room_id                   - Go to room (explicit)
- flag+ flag_name                 - Set flag
- flag- flag_name                 - Remove flag  
- count+ counter_name             - Increment counter
- count- counter_name             - Decrement counter
- count= counter_name 5           - Set counter value
- quit                            - End game
- end                             - End game (alternative)

Conditions:
  [flag_name]                       - Has flag
  [!flag_name]                      - Does NOT have flag
  [flag1 & flag2]                   - Has ALL flags (and)
  [flag1 | flag2]                   - Has ANY flags (or)
  [counter > 5]                     - Counter greater than
  [counter < 3]                     - Counter less than
  [counter = 10]                    - Counter equals
  [counter >= 5]                    - Greater than or equal
  [counter <= 3]                    - Less than or equal
  [counter != 0]                    - Not equal
  
Complex conditions:
  [has_wand & (wisdom > 5 | experience = 10)]

Comments:
  <!-- This is a comment -->
- "Action"  # This is also a comment

-->