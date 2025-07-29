use ratzilla::{DomBackend, WebRenderer};
use ratzilla::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::config::GameConfig;
use crate::world::{Choice, World};
use crate::game::{GameState, get_available_choices, get_room_description};
use crate::ui_trait::{GameUI, WaitForInput};
use std::error::Error;

// Global reference to the app for typewriter timer callbacks
static mut TYPEWRITER_APP: Option<Rc<App>> = None;

pub struct App {
    world: World,
    game_state: RefCell<GameState>,
    all_text: RefCell<Vec<String>>,
    current_choices: RefCell<Vec<String>>,
    available_choices: RefCell<Vec<Choice>>,
    selected_choice: RefCell<usize>,
    scroll_offset: RefCell<usize>,
    auto_scroll: RefCell<bool>,
    typewriter_text: RefCell<Vec<String>>,  // Text being typed with typewriter effect
    typewriter_complete: RefCell<bool>,     // Whether current typewriter effect is complete
    typewriter_current_line: RefCell<usize>, // Current line being typed
    typewriter_current_char: RefCell<usize>, // Current character position in line
    pending_texts: RefCell<Vec<String>>,    // Texts waiting to be displayed with pauses
    pending_text_index: RefCell<usize>,     // Current index in pending_texts
    waiting_for_continue: RefCell<bool>,    // Whether we're waiting for user to continue
    pending_room_change: RefCell<bool>,     // Whether there's a room change pending after text sequence
    room_text_sequence: RefCell<bool>,      // Whether current text sequence is for room description
    config: GameConfig,
}

impl App {
    pub fn new(world: World, game_state: GameState, config: GameConfig) -> Self {
        let app = App {
            world,
            game_state: RefCell::new(game_state),
            all_text: RefCell::new(Vec::new()),
            current_choices: RefCell::new(Vec::new()),
            available_choices: RefCell::new(Vec::new()),
            selected_choice: RefCell::new(0),
            scroll_offset: RefCell::new(0),
            auto_scroll: RefCell::new(true),
            typewriter_text: RefCell::new(Vec::new()),
            typewriter_complete: RefCell::new(true),
            typewriter_current_line: RefCell::new(0),
            typewriter_current_char: RefCell::new(0),
            pending_texts: RefCell::new(Vec::new()),
            pending_text_index: RefCell::new(0),
            waiting_for_continue: RefCell::new(false),
            pending_room_change: RefCell::new(false),
            room_text_sequence: RefCell::new(false),
            config,
        };

        // Initialize the game with welcome message and first room
        app.initialize_game();
        app
    }

    fn initialize_game(&self) {
        let game_state = self.game_state.borrow();
        let current_room = match self.world.rooms.get(&game_state.current_room_id) {
            Some(room) => room,
            None => {
                self.all_text.borrow_mut().push(format!("Error: Starting room '{}' not found!", game_state.current_room_id));
                return;
            }
        };

        let room_desc = get_room_description(current_room, &game_state);
        let mut all_text = self.all_text.borrow_mut();
        all_text.push("".to_string());
        all_text.push(room_desc);
        all_text.push("".to_string());

        drop(all_text);
        drop(game_state);
        
        // Load initial choices
        self.update_choices();
    }

    fn update_choices(&self) {
        let game_state = self.game_state.borrow();
        match get_available_choices(&self.world, &game_state) {
            Ok(choices) => {
                let mut current_choices = self.current_choices.borrow_mut();
                let mut available_choices = self.available_choices.borrow_mut();
                
                current_choices.clear();
                available_choices.clear();
                *self.selected_choice.borrow_mut() = 0;

                if choices.is_empty() {
                    self.all_text.borrow_mut().push("There is nothing you can do here.".to_string());
                    self.all_text.borrow_mut().push("🎉 Game Over!".to_string());
                    return;
                }

                for (i, choice) in choices.iter().enumerate() {
                    let choice_text = format!("{}: {}", i + 1, choice.text);
                    current_choices.push(choice_text);
                    available_choices.push((*choice).clone());
                }
            }
            Err(e) => {
                self.all_text.borrow_mut().push(format!("Error getting choices: {}", e));
            }
        }
    }

    pub fn handle_events(&self) -> Box<dyn Fn(KeyEvent)> {
        Box::new(move |_event| {
            // This will be handled by the terminal.on_key_event
        })
    }

    fn move_selection_up(&self) {
        let current_choices = self.current_choices.borrow();
        let mut selected_choice = self.selected_choice.borrow_mut();
        
        if !current_choices.is_empty() && *selected_choice > 0 {
            *selected_choice -= 1;
        }
    }

    fn move_selection_down(&self) {
        let current_choices = self.current_choices.borrow();
        let mut selected_choice = self.selected_choice.borrow_mut();
        
        if !current_choices.is_empty() && *selected_choice < current_choices.len() - 1 {
            *selected_choice += 1;
        }
    }

    fn select_choice(&self) {
        let selected_index = *self.selected_choice.borrow();
        let available_choices = self.available_choices.borrow();
        
        if selected_index < available_choices.len() {
            let choice = available_choices[selected_index].clone();
            drop(available_choices);
            
            // Store the current room ID before executing actions
            let previous_room_id = self.game_state.borrow().current_room_id.clone();
            
            // Prepare initial text to display
            let initial_text = vec![
                format!("> {}", choice.text),
                "".to_string(),
            ];
            
            // Enable auto-scroll when new content is added
            *self.auto_scroll.borrow_mut() = true;
            
            // Execute the choice actions using the unified approach
            let mut game_state = self.game_state.borrow_mut();
            let has_quit = game_state.has_quit;
            
            // Use a simplified version of execute_choice for web that handles text sequencing
            let action_texts = self.execute_actions_for_web(&choice, &mut game_state);
            let current_room_id = game_state.current_room_id.clone(); // Get room ID AFTER actions
            drop(game_state);
            
            // Add initial text to display
            self.all_text.borrow_mut().extend(initial_text);
            
            // Check if game has ended
            if has_quit {
                self.all_text.borrow_mut().push("🎉 Thank you for playing!".to_string());
                self.current_choices.borrow_mut().clear();
                self.available_choices.borrow_mut().clear();
                return;
            }
            
            // Check if we have text actions
            let has_text_actions = !action_texts.is_empty();
            
            // Start the text sequence if there are texts to display
            if has_text_actions {
                *self.room_text_sequence.borrow_mut() = false; // This is a choice action text sequence
                self.start_text_sequence(action_texts);
            }
            
            // Handle room change and choice updates
            if previous_room_id != current_room_id {
                *self.pending_room_change.borrow_mut() = true;
                // If we're not in a text sequence, handle room change immediately
                if !has_text_actions {
                    self.handle_room_change(current_room_id);
                    *self.pending_room_change.borrow_mut() = false;
                } 
                // If we have text actions, room change will be handled after text sequence completes
            } else {
                *self.pending_room_change.borrow_mut() = false;
                // No room change, update choices if not in text sequence
                if !has_text_actions {
                    self.update_choices();
                }
            }
        }
    }
    
    fn execute_actions_for_web(&self, choice: &Choice, game_state: &mut GameState) -> Vec<String> {
        use crate::world::Action;
        use crate::game::check_single_condition;
        
        let mut text_actions = Vec::new();
        
        // Execute immediate actions first
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
                Action::IncrementCounter(counter) => {
                    let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                    *game_state.counters.entry(counter.clone()).or_insert(0) += 1;
                    let new_value = *game_state.counters.get(counter).unwrap();
                    text_actions.push(format!("[{}: {} → {}]", counter, old_value, new_value));
                }
                Action::DecrementCounter(counter) => {
                    let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                    *game_state.counters.entry(counter.clone()).or_insert(0) -= 1;
                    let new_value = *game_state.counters.get(counter).unwrap();
                    text_actions.push(format!("[{}: {} → {}]", counter, old_value, new_value));
                }
                Action::SetCounter(counter, value) => {
                    let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                    game_state.counters.insert(counter.clone(), *value);
                    text_actions.push(format!("[{}: {} → {}]", counter, old_value, value));
                }
                Action::DisplayText(text) => {
                    text_actions.push(text.clone());
                }
                Action::DisplayTextConditional { condition, text_if_true, text_if_false } => {
                    let text = if check_single_condition(condition, game_state) {
                        text_if_true.clone()
                    } else {
                        text_if_false.clone()
                    };
                    text_actions.push(text);
                }
            }
        }
        
        text_actions
    }
    
    fn handle_room_change(&self, room_id: String) {
        let current_room = match self.world.rooms.get(&room_id) {
            Some(room) => room,
            None => {
                self.all_text.borrow_mut().push(format!("Error: Room '{}' not found!", room_id));
                return;
            }
        };
        
        let game_state = self.game_state.borrow();
        let room_desc = get_room_description(current_room, &game_state);
        drop(game_state);
        
        if room_desc.is_empty() {
            // Skip empty room descriptions
            return;
        }
        
        // Add separator first (immediately, no typewriter)
        let mut all_text = self.all_text.borrow_mut();
        all_text.push("".to_string());
        all_text.push("─".repeat(60));
        all_text.push("".to_string());
        drop(all_text);
        
        // Display room description with typewriter effect
        let room_texts = vec![room_desc];
        *self.room_text_sequence.borrow_mut() = true;
        self.start_text_sequence(room_texts);
    }

    fn select_by_number(&self, number: usize) {
        let available_choices = self.available_choices.borrow();
        if number > 0 && number <= available_choices.len() {
            drop(available_choices);
            *self.selected_choice.borrow_mut() = number - 1;
            self.select_choice();
        }
    }

    fn scroll_up(&self) {
        let mut auto_scroll = self.auto_scroll.borrow_mut();
        let mut scroll_offset = self.scroll_offset.borrow_mut();
        
        *auto_scroll = false; // Disable auto scroll when manually scrolling
        if *scroll_offset > 0 {
            *scroll_offset -= 1;
        }
    }

    fn scroll_down(&self) {
        let mut auto_scroll = self.auto_scroll.borrow_mut();
        let mut scroll_offset = self.scroll_offset.borrow_mut();
        let all_text = self.all_text.borrow();
        
        *auto_scroll = false; // Disable auto scroll when manually scrolling
        
        // Calculate visual lines to determine max scroll
        // Use an estimated width for bounds checking (actual width will be used in render)
        let estimated_width = 80; // Conservative estimate
        let mut total_visual_lines = 0;
        for text_line in all_text.iter() {
            if text_line.is_empty() {
                total_visual_lines += 1;
            } else {
                let wrapped_lines = (text_line.len() + estimated_width - 1) / estimated_width;
                total_visual_lines += wrapped_lines.max(1);
            }
        }
        
        // Estimate available height (will be refined in render)
        let current_choices = self.current_choices.borrow();
        let estimated_total_height: usize = 30; // Conservative terminal height
        let choices_height: usize = if current_choices.is_empty() { 0 } else { current_choices.len() + 2 };
        let estimated_content_height = estimated_total_height.saturating_sub(choices_height).saturating_sub(2);
        
        // Only scroll if there's more content than fits on screen
        if total_visual_lines > estimated_content_height {
            let max_scroll = total_visual_lines - estimated_content_height;
            if *scroll_offset < max_scroll {
                *scroll_offset += 1;
            }
        }
    }

    fn enable_auto_scroll(&self) {
        *self.auto_scroll.borrow_mut() = true;
        *self.scroll_offset.borrow_mut() = 0;
    }

    fn add_text_with_typewriter(&self, new_text: Vec<String>) {
        if self.config.enable_typewriter && !new_text.is_empty() {
            // Start typewriter effect for new text
            *self.typewriter_complete.borrow_mut() = false;
            *self.typewriter_text.borrow_mut() = new_text;
            *self.typewriter_current_line.borrow_mut() = 0;
            *self.typewriter_current_char.borrow_mut() = 0;
            self.start_typewriter_effect();
        } else {
            // Add text immediately if typewriter is disabled or no text
            self.all_text.borrow_mut().extend(new_text);
        }
    }

    fn start_typewriter_effect(&self) {
        // Set up the timer for typewriter effect
        let window = web_sys::window().unwrap();
        let speed_ms = if self.config.typewriter_speed_ms == 0 { 1 } else { self.config.typewriter_speed_ms };
        
        let closure = Closure::wrap(Box::new(move || {
            typewriter_tick();
        }) as Box<dyn FnMut()>);
        
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            speed_ms as i32,
        ).unwrap();
        
        closure.forget(); // Prevent cleanup
    }

    fn is_typewriter_complete(&self) -> bool {
        !self.config.enable_typewriter || *self.typewriter_complete.borrow()
    }

    fn skip_typewriter(&self) {
        if self.config.enable_typewriter && !*self.typewriter_complete.borrow() {
            // Complete the typewriter effect immediately by finishing all remaining text
            let typewriter_text = self.typewriter_text.borrow().clone();
            let current_line = *self.typewriter_current_line.borrow();
            let mut all_text = self.all_text.borrow_mut();
            
            // Complete the current partial line if it exists
            if current_line < typewriter_text.len() {
                // If we're in the middle of typing a line, complete it
                if let Some(last_line) = all_text.last_mut() {
                    *last_line = typewriter_text[current_line].clone();
                }
                
                // Add any remaining complete lines
                for i in (current_line + 1)..typewriter_text.len() {
                    all_text.push(typewriter_text[i].clone());
                }
            }
            
            drop(all_text);
            *self.typewriter_complete.borrow_mut() = true;
        }
    }

    fn typewriter_tick(&self) {
        if *self.typewriter_complete.borrow() {
            return; // Already complete
        }

        let typewriter_text = self.typewriter_text.borrow();
        let mut current_line = self.typewriter_current_line.borrow_mut();
        let mut current_char = self.typewriter_current_char.borrow_mut();

        if *current_line >= typewriter_text.len() {
            // All lines complete
            *self.typewriter_complete.borrow_mut() = true;
            return;
        }

        let line = &typewriter_text[*current_line];
        let line_chars: Vec<char> = line.chars().collect();
        
        if *current_char == 0 {
            // Starting a new line - add it to all_text as empty initially
            self.all_text.borrow_mut().push(String::new());
        }

        if *current_char < line_chars.len() {
            // Add next character to the current line
            let all_text_len = self.all_text.borrow().len();
            if all_text_len > 0 {
                let mut all_text = self.all_text.borrow_mut();
                let last_line_idx = all_text_len - 1;
                all_text[last_line_idx] = line_chars[..*current_char + 1].iter().collect();
            }
            *current_char += 1;
        } else {
            // Current line complete, move to next
            *current_line += 1;
            *current_char = 0;
        }

        // Continue typewriter effect if not complete
        if *current_line < typewriter_text.len() {
            self.start_typewriter_effect();
        } else {
            *self.typewriter_complete.borrow_mut() = true;
        }
    }

    fn start_text_sequence(&self, texts: Vec<String>) {
        if texts.is_empty() {
            return;
        }
        
        *self.pending_texts.borrow_mut() = texts;
        *self.pending_text_index.borrow_mut() = 0;
        *self.waiting_for_continue.borrow_mut() = false;
        // Note: room_text_sequence flag is set by the caller if needed
        
        // Start displaying the first text
        self.display_next_pending_text();
    }
    
    fn display_next_pending_text(&self) {
        let pending_texts = self.pending_texts.borrow();
        let mut pending_index = self.pending_text_index.borrow_mut();
        let total_texts = pending_texts.len();
        
        if *pending_index < total_texts {
            let text = pending_texts[*pending_index].clone();
            drop(pending_texts);
            
            // Display this text with typewriter effect
            self.add_text_with_typewriter(vec![text]);
            
            *pending_index += 1;
            
            // Check if there are more texts to display
            if *pending_index < total_texts {
                // Set waiting state for continue prompt
                *self.waiting_for_continue.borrow_mut() = true;
            } else {
                // Text sequence is complete
                drop(pending_index); // Drop the borrow before calling complete_text_sequence
                self.complete_text_sequence();
            }
        }
    }
    
    fn complete_text_sequence(&self) {
        // Clear the text sequence
        self.pending_texts.borrow_mut().clear();
        *self.pending_text_index.borrow_mut() = 0;
        
        // Check if this was a room text sequence
        let was_room_sequence = *self.room_text_sequence.borrow();
        *self.room_text_sequence.borrow_mut() = false;
        
        if was_room_sequence {
            // Room description sequence complete, update choices
            self.update_choices();
        } else if *self.pending_room_change.borrow() {
            // Handle any pending room change from choice actions
            let game_state = self.game_state.borrow();
            let current_room_id = game_state.current_room_id.clone();
            drop(game_state);
            
            self.handle_room_change(current_room_id);
            *self.pending_room_change.borrow_mut() = false;
        } else {
            // No room change, just update choices to reflect any state changes
            self.update_choices();
        }
    }
    
    fn continue_text_sequence(&self) {
        if *self.waiting_for_continue.borrow() {
            *self.waiting_for_continue.borrow_mut() = false;
            self.display_next_pending_text();
        }
    }
    
    fn is_waiting_for_continue(&self) -> bool {
        *self.waiting_for_continue.borrow()
    }
    

    pub fn render(&self, f: &mut Frame) {
        let all_text = self.all_text.borrow();
        let current_choices = self.current_choices.borrow();
        let selected_choice = *self.selected_choice.borrow();

        let main_layout = if !current_choices.is_empty() {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),        // Story area
                    Constraint::Length(current_choices.len() as u16 + 2), // Choices area
                ])
                .split(f.area())
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(f.area())
        };

        let story_area = main_layout[0];
        
        // Story area with margins
        let story_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(5),   // Left margin
                Constraint::Percentage(90),  // Content
                Constraint::Percentage(5),   // Right margin
            ])
            .split(story_area);

        let content_area = story_chunks[1];

        // Calculate how many lines can fit in the content area (accounting for borders and padding)
        let content_height = content_area.height.saturating_sub(2) as usize; // Subtract 2 for borders
        let content_width = content_area.width.saturating_sub(2) as usize; // Subtract 2 for borders
        let auto_scroll = *self.auto_scroll.borrow();
        let scroll_offset = *self.scroll_offset.borrow();
        
        // Calculate visual lines for each text line (accounting for wrapping)
        let mut visual_lines = Vec::new();
        for text_line in all_text.iter() {
            if text_line.is_empty() {
                visual_lines.push(1); // Empty lines still take 1 visual line
            } else {
                let wrapped_lines = (text_line.len() + content_width - 1) / content_width; // Ceiling division
                visual_lines.push(wrapped_lines.max(1)); // At least 1 line
            }
        }
        
        // Calculate cumulative visual lines for proper scrolling
        let total_visual_lines: usize = visual_lines.iter().sum();
        
        // Determine which lines to show based on scroll mode
        let content_text = if auto_scroll {
            // Auto-scroll: show the most recent text that fits
            if total_visual_lines > content_height {
                // Find the starting text line that fits in the viewport
                let mut cumulative_lines = 0;
                let mut start_idx = all_text.len();
                
                for (i, &lines) in visual_lines.iter().enumerate().rev() {
                    if cumulative_lines + lines <= content_height {
                        cumulative_lines += lines;
                        start_idx = i;
                    } else {
                        break;
                    }
                }
                
                all_text[start_idx..].join("\n")
            } else {
                // All text fits, show everything
                all_text.join("\n")
            }
        } else {
            // Manual scroll: show text based on scroll_offset
            if total_visual_lines <= content_height {
                // All text fits, ignore scroll offset
                all_text.join("\n")
            } else {
                // Clamp scroll offset to valid range
                let max_scroll = total_visual_lines.saturating_sub(content_height);
                let clamped_offset = scroll_offset.min(max_scroll);
                
                // Find text lines that correspond to the scroll offset
                let mut cumulative_lines = 0;
                let mut start_idx = 0;
                let mut end_idx = all_text.len();
                
                // Find start index based on clamped scroll offset
                for (i, &lines) in visual_lines.iter().enumerate() {
                    if cumulative_lines >= clamped_offset {
                        start_idx = i;
                        break;
                    }
                    cumulative_lines += lines;
                }
                
                // Find end index that fits in viewport
                cumulative_lines = 0;
                for (i, &lines) in visual_lines[start_idx..].iter().enumerate() {
                    if cumulative_lines + lines <= content_height {
                        cumulative_lines += lines;
                        end_idx = start_idx + i + 1;
                    } else {
                        break;
                    }
                }
                
                all_text[start_idx..end_idx].join("\n")
            }
        };

        let paragraph = Paragraph::new(content_text)
            .block(Block::default().borders(Borders::ALL).title("Restoration Project"))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));

        f.render_widget(paragraph, content_area);

        // Render choices if available, or prompts if waiting/typing
        let waiting = self.is_waiting_for_continue();
        let typewriter_active = !self.is_typewriter_complete();
        if !current_choices.is_empty() && !waiting && !typewriter_active {
            let choices_area = main_layout[1];
            
            let choices_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(5),   // Left margin
                    Constraint::Percentage(90),  // Choices content
                    Constraint::Percentage(5),   // Right margin
                ])
                .split(choices_area);

            let choices_content_area = choices_chunks[1];

            // Convert choices to ListItems
            let choice_items: Vec<ListItem> = current_choices
                .iter()
                .enumerate()
                .map(|(i, choice)| {
                    let style = if i == selected_choice {
                        Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(choice.as_str()).style(style)
                })
                .collect();

            let choices_list = List::new(choice_items)
                .block(Block::default().borders(Borders::ALL).title("Use ↑↓ arrow keys + Enter, or press 1-9"))
                .style(Style::default().fg(Color::White));

            f.render_widget(choices_list, choices_content_area);
        } else if waiting {
            // Show continue prompt
            let choices_area = main_layout[1];
            
            let continue_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(5),   // Left margin
                    Constraint::Percentage(90),  // Continue content
                    Constraint::Percentage(5),   // Right margin
                ])
                .split(choices_area);

            let continue_content_area = continue_chunks[1];

            let continue_prompt = Paragraph::new("Press Enter to continue...")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));

            f.render_widget(continue_prompt, continue_content_area);
        } else if typewriter_active {
            // Show skip typewriter prompt
            let choices_area = main_layout[1];
            
            let skip_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(5),   // Left margin
                    Constraint::Percentage(90),  // Skip content
                    Constraint::Percentage(5),   // Right margin
                ])
                .split(choices_area);

            let skip_content_area = skip_chunks[1];

            let skip_prompt = Paragraph::new("Press any key to skip...")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));

            f.render_widget(skip_prompt, skip_content_area);
        }
    }
}

impl GameUI for App {
    fn display_texts(&mut self, texts: &[String]) -> Result<(), Box<dyn Error>> {
        self.start_text_sequence(texts.to_vec());
        Ok(())
    }
    
    fn display_text(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        self.all_text.borrow_mut().push(text.to_string());
        Ok(())
    }
    
    fn display_choices(&mut self, choices: &[&Choice]) {
        let mut current_choices = self.current_choices.borrow_mut();
        let mut available_choices = self.available_choices.borrow_mut();
        
        current_choices.clear();
        available_choices.clear();
        *self.selected_choice.borrow_mut() = 0;

        if choices.is_empty() {
            self.all_text.borrow_mut().push("There is nothing you can do here.".to_string());
            self.all_text.borrow_mut().push("🎉 Game Over!".to_string());
            return;
        }

        for (i, choice) in choices.iter().enumerate() {
            let choice_text = format!("{}: {}", i + 1, choice.text);
            current_choices.push(choice_text);
            available_choices.push((*choice).clone());
        }
    }
    
    fn get_user_choice(&mut self) -> Result<usize, Box<dyn Error>> {
        // This method doesn't make sense for the web UI since it's event-driven
        // Instead, the web UI handles choices through the event system
        // We'll return the currently selected choice
        Ok(*self.selected_choice.borrow())
    }
    
    fn clear_choices(&mut self) {
        self.current_choices.borrow_mut().clear();
        self.available_choices.borrow_mut().clear();
        *self.selected_choice.borrow_mut() = 0;
    }
    
    fn add_separator(&mut self) {
        let mut all_text = self.all_text.borrow_mut();
        all_text.push("".to_string());
        all_text.push("─".repeat(60));
        all_text.push("".to_string());
    }
    
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        // No cleanup needed for web UI
        Ok(())
    }
}

impl WaitForInput for App {
    fn wait_for_continue(&mut self) -> Result<(), Box<dyn Error>> {
        // For web UI, we set the waiting state and return immediately
        // The actual waiting happens through the event system
        *self.waiting_for_continue.borrow_mut() = true;
        Ok(())
    }
}

// Global typewriter tick function that can be called from JavaScript timers
fn typewriter_tick() {
    unsafe {
        if let Some(app) = &TYPEWRITER_APP {
            app.typewriter_tick();
        }
    }
}

pub fn run_web_game(world: World, game_state: GameState, config: GameConfig) {
    let backend = DomBackend::new().expect("Failed to create DOM backend");
    let terminal = Terminal::new(backend).expect("Failed to create terminal");
    
    let app = Rc::new(App::new(world, game_state, config));
    let app_clone = app.clone();
    
    // Set global reference for typewriter callbacks
    unsafe {
        TYPEWRITER_APP = Some(app.clone());
    }
    
    // Set up event handling
    terminal.on_key_event(move |event| {
        match event.code {
            KeyCode::Up => {
                if app_clone.is_typewriter_complete() {
                    app_clone.move_selection_up();
                } else {
                    app_clone.skip_typewriter();
                }
            }
            KeyCode::Down => {
                if app_clone.is_typewriter_complete() {
                    app_clone.move_selection_down();
                } else {
                    app_clone.skip_typewriter();
                }
            }
            KeyCode::Enter => {
                if app_clone.is_typewriter_complete() {
                    if app_clone.is_waiting_for_continue() {
                        app_clone.continue_text_sequence();
                    } else {
                        app_clone.select_choice();
                    }
                } else {
                    app_clone.skip_typewriter();
                }
            }
            KeyCode::PageUp => {
                if app_clone.is_typewriter_complete() {
                    app_clone.scroll_up();
                } else {
                    app_clone.skip_typewriter();
                }
            }
            KeyCode::PageDown => {
                if app_clone.is_typewriter_complete() {
                    app_clone.scroll_down();
                } else {
                    app_clone.skip_typewriter();
                }
            }
            KeyCode::End => {
                if app_clone.is_typewriter_complete() {
                    app_clone.enable_auto_scroll();
                } else {
                    app_clone.skip_typewriter();
                }
            }
            KeyCode::Char(c) => {
                if app_clone.is_typewriter_complete() {
                    match c {
                        c if c.is_ascii_digit() => {
                            if let Some(digit) = c.to_digit(10) {
                                app_clone.select_by_number(digit as usize);
                            }
                        }
                        'k' | 'K' => {
                            app_clone.scroll_up();
                        }
                        'j' | 'J' => {
                            app_clone.scroll_down();
                        }
                        ' ' => {
                            app_clone.enable_auto_scroll();
                        }
                        _ => {}
                    }
                } else {
                    // Any key press skips typewriter effect
                    app_clone.skip_typewriter();
                }
            }
            _ => {
                // Any other key skips typewriter effect
                if !app_clone.is_typewriter_complete() {
                    app_clone.skip_typewriter();
                }
            }
        }
    });
    
    // Main render loop
    let app_render = app.clone();
    terminal.draw_web(move |f| {
        app_render.render(f);
    });
}