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
        all_text.push("🏰 Welcome to the Restoration Project".to_string());
        all_text.push("".to_string());
        all_text.push(room_desc);
        all_text.push("".to_string());
        all_text.push("🎮 Instructions:".to_string());
        all_text.push("• Use arrow keys ↑↓ to navigate choices".to_string());
        all_text.push("• Press Enter to select a choice".to_string());
        all_text.push("• Press number keys (1-9) for quick selection".to_string());
        all_text.push("• Use PageUp/PageDown or j/k to scroll text".to_string());
        all_text.push("• Press End or Spacebar to return to auto-scroll".to_string());
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
            
            // Prepare text to display
            let mut new_text = vec![
                format!("> {}", choice.text),
                "".to_string(),
            ];
            
            // Enable auto-scroll when new content is added
            *self.auto_scroll.borrow_mut() = true;
            
            // Execute the choice actions and collect any text output
            let mut game_state = self.game_state.borrow_mut();
            let action_text = self.execute_actions_web(&choice, &mut game_state);
            let has_quit = game_state.has_quit;
            let current_room_id = game_state.current_room_id.clone();
            drop(game_state);
            
            // Add any text from actions to the new text
            new_text.extend(action_text);
            
            // Check if game has ended
            if has_quit {
                self.all_text.borrow_mut().push("🎉 Thank you for playing!".to_string());
                self.current_choices.borrow_mut().clear();
                self.available_choices.borrow_mut().clear();
                return;
            }
            
            // Only display new room description if room actually changed
            if previous_room_id != current_room_id {
                let current_room = match self.world.rooms.get(&current_room_id) {
                    Some(room) => room,
                    None => {
                        new_text.push(format!("Error: Room '{}' not found!", current_room_id));
                        self.add_text_with_typewriter(new_text);
                        return;
                    }
                };
                
                let game_state = self.game_state.borrow();
                let room_desc = get_room_description(current_room, &game_state);
                drop(game_state);
                
                new_text.push("".to_string());
                new_text.push("─".repeat(60));
                new_text.push("".to_string());
                new_text.push(room_desc);
            }
            
            // Display all the new text with typewriter effect
            self.add_text_with_typewriter(new_text);
            
            // Update choices (this will happen regardless of room change)
            self.update_choices();
        }
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
            // Complete the typewriter effect immediately
            let typewriter_text = self.typewriter_text.borrow().clone();
            self.all_text.borrow_mut().extend(typewriter_text);
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

    fn execute_actions_web(&self, choice: &Choice, game_state: &mut GameState) -> Vec<String> {
        use crate::world::Action;
        use crate::game::check_single_condition;
        
        let mut text_output = Vec::new();
        
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
                Action::DisplayText(text) => {
                    text_output.push(text.clone());
                }
                Action::DisplayTextConditional { condition, text_if_true, text_if_false } => {
                    let text = if check_single_condition(condition, game_state) {
                        text_if_true
                    } else {
                        text_if_false
                    };
                    text_output.push(text.clone());
                }
                Action::IncrementCounter(counter) => {
                    let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                    *game_state.counters.entry(counter.clone()).or_insert(0) += 1;
                    let new_value = *game_state.counters.get(counter).unwrap();
                    text_output.push(format!("[{}: {} → {}]", counter, old_value, new_value));
                }
                Action::DecrementCounter(counter) => {
                    let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                    *game_state.counters.entry(counter.clone()).or_insert(0) -= 1;
                    let new_value = *game_state.counters.get(counter).unwrap();
                    text_output.push(format!("[{}: {} → {}]", counter, old_value, new_value));
                }
                Action::SetCounter(counter, value) => {
                    let old_value = *game_state.counters.get(counter).unwrap_or(&0);
                    game_state.counters.insert(counter.clone(), *value);
                    text_output.push(format!("[{}: {} → {}]", counter, old_value, value));
                }
            }
        }
        
        text_output
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

        // Render choices if available
        if !current_choices.is_empty() {
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
        }
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
                    app_clone.select_choice();
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