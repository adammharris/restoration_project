use std::collections::HashMap;
use crate::world::{World, Room, Choice, Action, Condition, FlagId, ConditionalDescription};
use crate::errors::{GameError, GameResult};

#[derive(Debug)]
struct MarkdownStory {
    title: Option<String>,
    starting_room_id: String,
    rooms: HashMap<String, MarkdownRoom>,
}

#[derive(Debug)]
struct MarkdownRoom {
    id: String,
    descriptions: Vec<MarkdownDescription>,
    choices: Vec<MarkdownChoice>,
}

#[derive(Debug)]
struct MarkdownDescription {
    condition: Option<Condition>,
    text: String,
}

#[derive(Debug)]
struct MarkdownChoice {
    text: String,
    condition: Option<Condition>,
    actions: Vec<Action>,
}

pub fn parse_markdown_story(content: &str) -> GameResult<World> {
    let story = parse_markdown(content)?;
    convert_to_world(story)
}

fn parse_markdown(content: &str) -> GameResult<MarkdownStory> {
    let lines: Vec<&str> = content.lines().collect();
    let mut story = MarkdownStory {
        title: None,
        starting_room_id: String::new(),
        rooms: HashMap::new(),
    };
    
    let mut current_room: Option<MarkdownRoom> = None;
    let mut current_choice: Option<MarkdownChoice> = None;
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("<!--") {
            i += 1;
            continue;
        }
        
        // Story title: # Title
        if let Some(title) = parse_title(line) {
            story.title = Some(title);
        }
        // Starting room: @start room_id
        else if let Some(start_id) = parse_start_directive(line) {
            story.starting_room_id = start_id;
        }
        // Room definition: ## @room room_id
        else if let Some(room_id) = parse_room_header(line) {
            // Save previous room if exists
            if let Some(room) = current_room.take() {
                // Save current choice if exists
                if let Some(choice) = current_choice.take() {
                    let mut room = room;
                    room.choices.push(choice);
                    story.rooms.insert(room.id.clone(), room);
                } else {
                    story.rooms.insert(room.id.clone(), room);
                }
            }
            
            // Start new room - parse conditional descriptions
            i += 1;
            let descriptions = parse_room_descriptions(&lines, &mut i)?;
            
            current_room = Some(MarkdownRoom {
                id: room_id,
                descriptions,
                choices: Vec::new(),
            });
            continue; // Don't increment i again
        }
        // Choice definition: ### Choice text [condition]
        else if let Some((choice_text, condition)) = parse_choice_header(line) {
            // Save previous choice if exists
            if let Some(choice) = current_choice.take() {
                if let Some(ref mut room) = current_room {
                    room.choices.push(choice);
                }
            }
            
            current_choice = Some(MarkdownChoice {
                text: choice_text,
                condition,
                actions: Vec::new(),
            });
        }
        // Action line: tab-indented action
        else if let Some(action_str) = parse_action_line(line) {
            if let Some(ref mut choice) = current_choice {
                match parse_action(&action_str) {
                    Ok(action) => choice.actions.push(action),
                    Err(e) => eprintln!("Warning: Failed to parse action '{}': {}", action_str, e),
                }
            }
        }
        
        i += 1;
    }
    
    // Save final room and choice
    if let Some(choice) = current_choice {
        if let Some(ref mut room) = current_room {
            room.choices.push(choice);
        }
    }
    if let Some(room) = current_room {
        story.rooms.insert(room.id.clone(), room);
    }
    
    // Validate required fields
    if story.starting_room_id.is_empty() {
        return Err(GameError::ValidationError("No starting room specified. Use @start room_id".to_string()));
    }
    
    Ok(story)
}

fn parse_room_descriptions(lines: &[&str], i: &mut usize) -> GameResult<Vec<MarkdownDescription>> {
    let mut descriptions = Vec::new();
    let mut current_condition: Option<Condition> = None;
    let mut current_text_lines = Vec::new();
    
    while *i < lines.len() {
        let line = lines[*i].trim();
        
        // Empty line
        if line.is_empty() {
            *i += 1;
            continue;
        }
        
        // Stop at headers or choices
        if line.starts_with("###") || line.starts_with("##") || line.starts_with("#") {
            break;
        }
        
        // Check for conditional block: [condition]
        if line.starts_with('[') && line.ends_with(']') {
            // Save previous description if exists
            if !current_text_lines.is_empty() {
                descriptions.push(MarkdownDescription {
                    condition: current_condition.take(),
                    text: current_text_lines.join(" "),
                });
                current_text_lines.clear();
            }
            
            // Parse new condition from line like "[condition]"
            let condition_str = &line[1..line.len()-1];
            current_condition = parse_condition(condition_str).ok();
        } else {
            // Regular description text
            current_text_lines.push(line);
        }
        
        *i += 1;
    }
    
    // Save final description
    if !current_text_lines.is_empty() {
        descriptions.push(MarkdownDescription {
            condition: current_condition,
            text: current_text_lines.join(" "),
        });
    }
    
    // If no conditional descriptions found, treat as single unconditional description
    if descriptions.is_empty() {
        return Ok(vec![MarkdownDescription {
            condition: None,
            text: String::new(),
        }]);
    }
    
    Ok(descriptions)
}

fn parse_title(line: &str) -> Option<String> {
    if line.starts_with("# ") {
        Some(line[2..].trim().to_string())
    } else {
        None
    }
}

fn parse_start_directive(line: &str) -> Option<String> {
    if line.starts_with("@start ") {
        Some(line[7..].trim().to_string())
    } else {
        None
    }
}

fn parse_room_header(line: &str) -> Option<String> {
    if line.starts_with("## @room ") {
        Some(line[9..].trim().to_string())
    } else {
        None
    }
}

fn parse_choice_header(line: &str) -> Option<(String, Option<Condition>)> {
    if !line.starts_with("### ") {
        return None;
    }
    
    let content = &line[4..];
    
    // Check for condition in brackets at the end
    if let Some(bracket_start) = content.rfind('[') {
        if content.ends_with(']') {
            let choice_text = content[..bracket_start].trim().to_string();
            let condition_str = &content[bracket_start + 1..content.len() - 1];
            let condition = parse_condition(condition_str).ok();
            return Some((choice_text, condition));
        }
    }
    
    // No condition
    Some((content.trim().to_string(), None))
}

fn parse_action_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    // Check for dash-prefixed action (Markdown list style)
    if trimmed.starts_with("- ") {
        let action = trimmed[2..].trim().to_string();
        Some(action)
    } else {
        None
    }
}

fn parse_action(action_str: &str) -> GameResult<Action> {
    let action_str = action_str.trim();
    
    // Display text with say: prefix
    if action_str.starts_with("say: ") {
        return Ok(Action::DisplayText(action_str[5..].to_string()));
    }
    
    // Movement: goto: room_id or @room_id
    if action_str.starts_with("goto: ") {
        return Ok(Action::GoTo(action_str[6..].to_string()));
    }
    
    if action_str.starts_with('@') {
        return Ok(Action::GoTo(action_str[1..].to_string()));
    }
    
    // Flags: flag+ name, flag- name
    if action_str.starts_with("flag+ ") {
        return Ok(Action::SetFlag(FlagId(action_str[6..].to_string())));
    }
    
    if action_str.starts_with("flag- ") {
        return Ok(Action::RemoveFlag(FlagId(action_str[6..].to_string())));
    }
    
    // Counters: count+ name, count- name, count= name value
    if action_str.starts_with("count+ ") {
        return Ok(Action::IncrementCounter(action_str[7..].to_string()));
    }
    
    if action_str.starts_with("count- ") {
        return Ok(Action::DecrementCounter(action_str[7..].to_string()));
    }
    
    if action_str.starts_with("count= ") {
        let parts: Vec<&str> = action_str[7..].split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(value) = parts[1].parse::<i32>() {
                return Ok(Action::SetCounter(parts[0].to_string(), value));
            }
        }
    }
    
    // Quit: quit, end
    if action_str == "quit" || action_str == "end" {
        return Ok(Action::Quit);
    }
    
    // Default: treat as display text (no quotes needed)
    Ok(Action::DisplayText(action_str.to_string()))
}

fn parse_condition(condition_str: &str) -> GameResult<Condition> {
    let condition_str = condition_str.trim();
    
    // Check for logical AND (&)
    if let Some(pos) = condition_str.find(" & ") {
        let left = condition_str[..pos].trim();
        let right = condition_str[pos + 3..].trim();
        
        let left_condition = parse_single_condition(left)?;
        let right_condition = parse_single_condition(right)?;
        
        return Ok(Condition::And(Box::new(left_condition), Box::new(right_condition)));
    }
    
    // Check for logical OR (|)
    if let Some(pos) = condition_str.find(" | ") {
        let left = condition_str[..pos].trim();
        let right = condition_str[pos + 3..].trim();
        
        let left_condition = parse_single_condition(left)?;
        let right_condition = parse_single_condition(right)?;
        
        return Ok(Condition::Or(Box::new(left_condition), Box::new(right_condition)));
    }
    
    // Single condition
    parse_single_condition(condition_str)
}


fn parse_single_condition(condition_str: &str) -> GameResult<Condition> {
    let condition_str = condition_str.trim();
    
    // Simple flag conditions
    if condition_str.starts_with('!') {
        return Ok(Condition::NotHasFlag(FlagId(condition_str[1..].to_string())));
    }
    
    // Check for counter conditions
    for op in &[" >= ", " <= ", " > ", " < ", " = ", " != "] {
        if let Some(pos) = condition_str.find(op) {
            let counter = condition_str[..pos].trim();
            let value_str = condition_str[pos + op.len()..].trim();
            if let Ok(value) = value_str.parse::<i32>() {
                return match op.trim() {
                    ">" => Ok(Condition::CounterGreaterThan(counter.to_string(), value)),
                    "<" => Ok(Condition::CounterLessThan(counter.to_string(), value)),
                    "=" => Ok(Condition::CounterEquals(counter.to_string(), value)),
                    _ => Err(GameError::ValidationError(format!("Unsupported operator: {}", op))),
                };
            }
        }
    }
    
    // Simple flag condition (no prefix)
    Ok(Condition::HasFlag(FlagId(condition_str.to_string())))
}

fn convert_to_world(story: MarkdownStory) -> GameResult<World> {
    let mut rooms = HashMap::new();
    let mut choices = HashMap::new();
    let mut choice_counter = 0;
    
    for (room_id, md_room) in story.rooms {
        let mut room_choices = Vec::new();
        
        for md_choice in md_room.choices {
            let choice_id = format!("choice_{}", choice_counter);
            choice_counter += 1;
            
            let choice = Choice {
                text: md_choice.text,
                condition: md_choice.condition,
                actions: md_choice.actions,
            };
            
            room_choices.push(choice_id.clone());
            choices.insert(choice_id, choice);
        }
        
        let descriptions = md_room.descriptions.into_iter().map(|md_desc| {
            ConditionalDescription {
                condition: md_desc.condition,
                text: md_desc.text,
            }
        }).collect();
        
        let room = Room {
            descriptions,
            choices: room_choices,
        };
        
        rooms.insert(room_id, room);
    }
    
    Ok(World {
        rooms,
        choices,
        starting_room_id: story.starting_room_id,
    })
}