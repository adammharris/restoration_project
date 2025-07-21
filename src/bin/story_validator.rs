use restoration_project::world::{load_world_from_markdown, World, Condition};
use restoration_project::errors::GameError;
use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: story_validator <story.md>");
        eprintln!("Example: cargo run --bin story_validator the_cellar.md");
        std::process::exit(1);
    }

    let story_file = &args[1];
    
    match validate_story(story_file) {
        Ok(stats) => {
            println!("✅ Story file '{}' is valid!", story_file);
            println!("\n📊 Story Statistics:");
            print_story_stats(&stats);
        }
        Err(e) => {
            eprintln!("❌ Validation failed for '{}': {}", story_file, e);
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
pub struct StoryStats {
    pub total_rooms: usize,
    pub total_choices: usize,
    pub unreachable_rooms: Vec<String>,
    pub dead_end_rooms: Vec<String>,
    pub rooms_with_no_exit: Vec<String>,
    pub max_depth: usize,
    pub total_flags: HashSet<String>,
}

fn validate_story(path: &str) -> Result<StoryStats, GameError> {
    let world = load_world_from_markdown(path)?;
    
    // Additional validation checks
    let stats = analyze_story(&world)?;
    
    // Check for unreachable rooms
    if !stats.unreachable_rooms.is_empty() {
        eprintln!("⚠️  Warning: Found {} unreachable room(s): {:?}", 
                 stats.unreachable_rooms.len(), stats.unreachable_rooms);
    }
    
    // Check for dead ends
    if !stats.dead_end_rooms.is_empty() {
        eprintln!("⚠️  Warning: Found {} dead-end room(s): {:?}", 
                 stats.dead_end_rooms.len(), stats.dead_end_rooms);
    }
    
    Ok(stats)
}

fn analyze_story(world: &World) -> Result<StoryStats, GameError> {
    let mut reachable_rooms = HashSet::new();
    let mut flags = HashSet::new();
    
    // Find all reachable rooms starting from the starting room
    find_reachable_rooms(world, &world.starting_room_id, &mut reachable_rooms, &mut HashSet::new(), 0);
    
    // Collect all flags used in the story
    for choice in world.choices.values() {
        if let Some(condition) = &choice.condition {
            match condition {
                restoration_project::world::Condition::HasFlag(flag) => {
                    flags.insert(flag.0.clone());
                }
                restoration_project::world::Condition::NotHasFlag(flag) => {
                    flags.insert(flag.0.clone());
                }
                restoration_project::world::Condition::HasAllFlags(flag_list) => {
                    for flag in flag_list {
                        flags.insert(flag.0.clone());
                    }
                }
                restoration_project::world::Condition::HasAnyFlags(flag_list) => {
                    for flag in flag_list {
                        flags.insert(flag.0.clone());
                    }
                }
                restoration_project::world::Condition::CounterGreaterThan(_, _) |
                restoration_project::world::Condition::CounterLessThan(_, _) |
                restoration_project::world::Condition::CounterEquals(_, _) => {
                    // Counters don't use flags
                }
                restoration_project::world::Condition::And(left, right) => {
                    collect_flags_from_condition(left, &mut flags);
                    collect_flags_from_condition(right, &mut flags);
                }
                restoration_project::world::Condition::Or(left, right) => {
                    collect_flags_from_condition(left, &mut flags);
                    collect_flags_from_condition(right, &mut flags);
                }
            }
        }
        
        for action in &choice.actions {
            match action {
                restoration_project::world::Action::SetFlag(flag) => {
                    flags.insert(flag.0.clone());
                }
                restoration_project::world::Action::RemoveFlag(flag) => {
                    flags.insert(flag.0.clone());
                }
                restoration_project::world::Action::DisplayTextConditional { condition, .. } => {
                    // Handle flags in conditional display text
                    match condition {
                        restoration_project::world::Condition::HasFlag(flag) |
                        restoration_project::world::Condition::NotHasFlag(flag) => {
                            flags.insert(flag.0.clone());
                        }
                        restoration_project::world::Condition::HasAllFlags(flag_list) |
                        restoration_project::world::Condition::HasAnyFlags(flag_list) => {
                            for flag in flag_list {
                                flags.insert(flag.0.clone());
                            }
                        }
                        _ => {}
                    }
                }
                _ => {} // Other actions don't affect flags
            }
        }
    }
    
    // Find unreachable rooms
    let unreachable_rooms: Vec<String> = world.rooms.keys()
        .filter(|room_id| !reachable_rooms.contains(*room_id))
        .cloned()
        .collect();
    
    // Find dead end rooms (rooms with no valid choices that lead anywhere)
    let mut dead_end_rooms = Vec::new();
    let mut rooms_with_no_exit = Vec::new();
    
    for (room_id, room) in &world.rooms {
        let mut has_exit = false;
        let mut has_any_choice = false;
        
        for choice_id in &room.choices {
            if let Some(choice) = world.choices.get(choice_id) {
                has_any_choice = true;
                for action in &choice.actions {
                    match action {
                        restoration_project::world::Action::GoTo(_) => {
                            has_exit = true;
                            break;
                        }
                        restoration_project::world::Action::Quit => {
                            has_exit = true;
                            break;
                        }
                        _ => {}
                    }
                }
                if has_exit {
                    break;
                }
            }
        }
        
        if !has_any_choice {
            rooms_with_no_exit.push(room_id.clone());
        } else if !has_exit {
            dead_end_rooms.push(room_id.clone());
        }
    }
    
    Ok(StoryStats {
        total_rooms: world.rooms.len(),
        total_choices: world.choices.len(),
        unreachable_rooms,
        dead_end_rooms,
        rooms_with_no_exit,
        max_depth: calculate_max_depth(world),
        total_flags: flags,
    })
}

fn find_reachable_rooms(
    world: &World,
    room_id: &str,
    reachable: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
    depth: usize,
) {
    if reachable.contains(room_id) || visiting.contains(room_id) {
        return;
    }
    
    visiting.insert(room_id.to_string());
    reachable.insert(room_id.to_string());
    
    if let Some(room) = world.rooms.get(room_id) {
        for choice_id in &room.choices {
            if let Some(choice) = world.choices.get(choice_id) {
                for action in &choice.actions {
                    if let restoration_project::world::Action::GoTo(next_room) = action {
                        find_reachable_rooms(world, next_room, reachable, visiting, depth + 1);
                    }
                }
            }
        }
    }
    
    visiting.remove(room_id);
}

fn calculate_max_depth(world: &World) -> usize {
    fn dfs(world: &World, room_id: &str, visited: &mut HashSet<String>, depth: usize) -> usize {
        if visited.contains(room_id) {
            return depth;
        }
        
        visited.insert(room_id.to_string());
        let mut max_depth = depth;
        
        if let Some(room) = world.rooms.get(room_id) {
            for choice_id in &room.choices {
                if let Some(choice) = world.choices.get(choice_id) {
                    for action in &choice.actions {
                        if let restoration_project::world::Action::GoTo(next_room) = action {
                            let new_depth = dfs(world, next_room, visited, depth + 1);
                            max_depth = max_depth.max(new_depth);
                        }
                    }
                }
            }
        }
        
        visited.remove(room_id);
        max_depth
    }
    
    dfs(world, &world.starting_room_id, &mut HashSet::new(), 0)
}

fn collect_flags_from_condition(condition: &Condition, flags: &mut HashSet<String>) {
    match condition {
        Condition::HasFlag(flag) => {
            flags.insert(flag.0.clone());
        }
        Condition::NotHasFlag(flag) => {
            flags.insert(flag.0.clone());
        }
        Condition::HasAllFlags(flag_list) => {
            for flag in flag_list {
                flags.insert(flag.0.clone());
            }
        }
        Condition::HasAnyFlags(flag_list) => {
            for flag in flag_list {
                flags.insert(flag.0.clone());
            }
        }
        Condition::CounterGreaterThan(_, _) |
        Condition::CounterLessThan(_, _) |
        Condition::CounterEquals(_, _) => {
            // Counters don't use flags
        }
        Condition::And(left, right) => {
            collect_flags_from_condition(left, flags);
            collect_flags_from_condition(right, flags);
        }
        Condition::Or(left, right) => {
            collect_flags_from_condition(left, flags);
            collect_flags_from_condition(right, flags);
        }
    }
}

fn print_story_stats(stats: &StoryStats) {
    println!("  📚 Total rooms: {}", stats.total_rooms);
    println!("  🔀 Total choices: {}", stats.total_choices);
    println!("  🏁 Total flags: {}", stats.total_flags.len());
    println!("  📏 Maximum depth: {}", stats.max_depth);
    
    if !stats.total_flags.is_empty() {
        println!("  🎌 Flags used: {:?}", stats.total_flags.iter().collect::<Vec<_>>());
    }
    
    if !stats.unreachable_rooms.is_empty() {
        println!("  🚫 Unreachable rooms: {:?}", stats.unreachable_rooms);
    }
    
    if !stats.dead_end_rooms.is_empty() {
        println!("  ⚠️  Dead-end rooms: {:?}", stats.dead_end_rooms);
    }
    
    if !stats.rooms_with_no_exit.is_empty() {
        println!("  🚪 Rooms with no choices: {:?}", stats.rooms_with_no_exit);
    }
}