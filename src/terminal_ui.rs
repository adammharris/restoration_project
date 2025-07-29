use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect, Size},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};
use std::io;
use std::time::Duration;

use crate::config::GameConfig;
use crate::world::Choice;
use crate::ui_trait::{GameUI, WaitForInput};
use std::error::Error;

pub struct TerminalUi {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    all_text: Vec<String>,  // All accumulated text including story and responses
    current_choices: Vec<String>,
    selected_choice: usize,  // Currently selected choice index
    config: GameConfig,
    scroll_view_state: ScrollViewState,
    choice_list_state: ListState, // Separate state for choices
}

impl TerminalUi {
    pub fn new(config: GameConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(TerminalUi {
            terminal,
            all_text: Vec::new(),
            current_choices: Vec::new(),
            selected_choice: 0,
            config,
            scroll_view_state: ScrollViewState::default(),
            choice_list_state: ListState::default(),
        })
    }

    pub fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn display_text(&mut self, text: &str) {
        // Add text to the scrolling history
        if !text.trim().is_empty() {
            if self.config.enable_typewriter {
                self.display_text_with_typewriter(text);
            } else {
                self.all_text.push(text.to_string());
                self.scroll_to_bottom();
            }
        }
    }

    fn display_text_with_typewriter(&mut self, text: &str) {
        // Add a new line for this text
        self.all_text.push(String::new());
        let current_line_index = self.all_text.len() - 1;
        
        let mut current_text = String::new();
        
        for ch in text.chars() {
            current_text.push(ch);
            
            // Update the current line with the progress
            self.all_text[current_line_index] = current_text.clone();
            
            // Capture values needed for drawing
            let current_choices = self.current_choices.clone();
            let mut scroll_view_state = self.scroll_view_state.clone();
            let mut choice_list_state = self.choice_list_state.clone();
            
            // Force a complete redraw for each character
            if let Err(e) = self.terminal.draw(|f| {
                draw_terminal_content_with_scrollview(f, &self.all_text, &current_choices, &mut scroll_view_state, &mut choice_list_state);
            }) {
                eprintln!("Error drawing during typewriter: {}", e);
                break;
            }
            
            // Update states back
            self.scroll_view_state = scroll_view_state;
            self.choice_list_state = choice_list_state;
            
            // Sleep for typewriter effect
            std::thread::sleep(std::time::Duration::from_millis(self.config.typewriter_speed_ms));
        }
        
        // Ensure the final text is set
        self.all_text[current_line_index] = text.to_string();
        
        // Auto-scroll to bottom after completion
        self.scroll_view_state.scroll_to_bottom();
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll_view_state.scroll_to_bottom();
    }

    pub fn scroll_up(&mut self) {
        self.scroll_view_state.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_view_state.scroll_down();
    }

    pub fn display_choices(&mut self, choices: &[&Choice]) {
        // Clear previous choices and store new ones
        self.current_choices.clear();
        self.selected_choice = 0; // Reset selection
        
        if !choices.is_empty() {
            for (i, choice) in choices.iter().enumerate() {
                let choice_text = format!("{}: {}", i + 1, choice.text);
                self.current_choices.push(choice_text);
            }
            
            // Update choice selection state
            self.choice_list_state.select(Some(0));
        }
    }

    pub fn move_selection_up(&mut self) {
        if !self.current_choices.is_empty() && self.selected_choice > 0 {
            self.selected_choice -= 1;
            self.choice_list_state.select(Some(self.selected_choice));
        }
    }

    pub fn move_selection_down(&mut self) {
        if !self.current_choices.is_empty() && self.selected_choice < self.current_choices.len() - 1 {
            self.selected_choice += 1;
            self.choice_list_state.select(Some(self.selected_choice));
        }
    }

    pub fn add_user_input(&mut self, input: &str) {
        // Show what the user typed
        self.all_text.push(format!("> {}", input));
        self.all_text.push("".to_string()); // Add spacing after input
        self.scroll_to_bottom();
    }

    pub fn get_input(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        loop {
            self.draw()?;
            
            if event::poll(Duration::from_millis(16))? {
                let event = event::read()?;
                match event {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        match key.code {
                            KeyCode::Enter => {
                                if !self.current_choices.is_empty() {
                                    let choice_number = self.selected_choice + 1;
                                    let input = choice_number.to_string();
                                    self.add_user_input(&format!("Selected: {}", self.current_choices[self.selected_choice]));
                                    return Ok(input);
                                }
                            }
                            KeyCode::Up => {
                                if !self.current_choices.is_empty() {
                                    self.move_selection_up();
                                } else {
                                    self.scroll_up();
                                }
                            }
                            KeyCode::Down => {
                                if !self.current_choices.is_empty() {
                                    self.move_selection_down();
                                } else {
                                    self.scroll_down();
                                }
                            }
                            // Add Ctrl+ combinations for scrolling even with choices present
                            KeyCode::Char('k') => {
                                self.scroll_up();
                            }
                            KeyCode::Char('j') => {
                                self.scroll_down();
                            }
                            KeyCode::PageUp => {
                                for _ in 0..5 { self.scroll_up(); }
                            }
                            KeyCode::PageDown => {
                                for _ in 0..5 { self.scroll_down(); }
                            }
                            KeyCode::Home => {
                                self.scroll_view_state.scroll_to_top();
                            }
                            KeyCode::End => {
                                self.scroll_view_state.scroll_to_bottom();
                            }
                            KeyCode::Esc => {
                                return Ok("quit".to_string());
                            }
                            // Allow number keys as shortcuts
                            KeyCode::Char(c) if c.is_ascii_digit() => {
                                if let Some(digit) = c.to_digit(10) {
                                    let choice_index = (digit as usize).saturating_sub(1);
                                    if choice_index < self.current_choices.len() {
                                        self.selected_choice = choice_index;
                                        self.choice_list_state.select(Some(choice_index));
                                        let input = digit.to_string();
                                        self.add_user_input(&format!("Selected: {}", self.current_choices[choice_index]));
                                        return Ok(input);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::Mouse(mouse) => {
                        match mouse.kind {
                            MouseEventKind::ScrollUp => {
                                self.scroll_up();
                            }
                            MouseEventKind::ScrollDown => {
                                self.scroll_down();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let current_choices = self.current_choices.clone();
        let mut scroll_view_state = self.scroll_view_state.clone();
        let mut choice_list_state = self.choice_list_state.clone();
        
        self.terminal.draw(|f| {
            draw_terminal_content_with_scrollview(f, &self.all_text, &current_choices, &mut scroll_view_state, &mut choice_list_state);
        })?;

        // Update states back
        self.scroll_view_state = scroll_view_state;
        self.choice_list_state = choice_list_state;

        Ok(())
    }



    pub fn clear_text(&mut self) {
        // Don't clear all text, just add a separator for new rooms
        self.all_text.push("".to_string());
        self.all_text.push("─".repeat(60));
        self.all_text.push("".to_string());
        self.scroll_to_bottom();
    }
}

impl GameUI for TerminalUi {
    fn display_texts(&mut self, texts: &[String]) -> Result<(), Box<dyn Error>> {
        for (i, text) in texts.iter().enumerate() {
            TerminalUi::display_text(self, text);
            
            // Wait for user input between texts (except for the last one)
            if i < texts.len() - 1 {
                self.wait_for_continue()?;
            }
        }
        Ok(())
    }
    
    fn display_text(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        // Call the existing display_text method from TerminalUi
        TerminalUi::display_text(self, text);
        Ok(())
    }
    
    fn display_choices(&mut self, choices: &[&Choice]) {
        TerminalUi::display_choices(self, choices);
    }
    
    fn get_user_choice(&mut self) -> Result<usize, Box<dyn Error>> {
        let input = self.get_input()?;
        
        // Parse the input as a number (1-based) and convert to 0-based index
        if let Ok(choice_num) = input.parse::<usize>() {
            if choice_num > 0 && choice_num <= self.current_choices.len() {
                return Ok(choice_num - 1);
            }
        }
        
        // If parsing failed, return error
        Err("Invalid choice".into())
    }
    
    fn clear_choices(&mut self) {
        self.current_choices.clear();
        self.selected_choice = 0;
        self.choice_list_state.select(None);
    }
    
    fn add_separator(&mut self) {
        self.clear_text();
    }
    
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        TerminalUi::cleanup(self)
    }
}

impl WaitForInput for TerminalUi {
    fn wait_for_continue(&mut self) -> Result<(), Box<dyn Error>> {
        // Display "Press Enter to continue..." and wait for input
        self.all_text.push("".to_string());
        self.all_text.push("Press Enter to continue...".to_string());
        self.scroll_to_bottom();
        
        // Draw the current state
        self.draw()?;
        
        // Wait for any key press
        loop {
            if event::poll(Duration::from_millis(16))? {
                let event = event::read()?;
                if let Event::Key(key) = event {
                    if key.kind == KeyEventKind::Press {
                        // Any key press continues
                        break;
                    }
                }
            }
        }
        
        // Remove the "Press Enter to continue..." message
        if let Some(last) = self.all_text.last() {
            if last == "Press Enter to continue..." {
                self.all_text.pop();
                self.all_text.pop(); // Also remove the empty line
            }
        }
        
        Ok(())
    }
}

// Standalone function for drawing terminal content with ScrollView to avoid borrowing issues
fn draw_terminal_content_with_scrollview(
    f: &mut Frame, 
    all_text: &[String], 
    current_choices: &[String], 
    scroll_view_state: &mut ScrollViewState,
    choice_list_state: &mut ListState
) {
    let main_layout = if !current_choices.is_empty() {
        // Split into story area and choices area when choices are available
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),        // Story area
                Constraint::Length(current_choices.len() as u16 + 2), // Choices area (with borders)
            ])
            .split(f.area())
    } else {
        // Full screen for story when no choices
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(f.area())
    };

    let story_area = main_layout[0];
    
    // Story area layout with margins
    let story_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),  // Left margin
            Constraint::Percentage(80),  // Content
            Constraint::Percentage(10),  // Right margin
        ])
        .split(story_area);

    let content_area = story_chunks[1];

    // Create the content text from all text lines
    let raw_content = all_text.join("\n");
    
    // Pre-wrap text to fit the content area width
    let content_width = content_area.width.saturating_sub(2); // Account for borders
    let wrapped_lines = wrap_text_to_width(&raw_content, content_width as usize);
    let content_text = wrapped_lines.join("\n");
    let content_height = wrapped_lines.len() as u16;
    
    let content_size = Size::new(content_width, content_height);
    
    // Create the scrollable content with ScrollView
    let title = "Restoration Project";

    let mut scroll_view = ScrollView::new(content_size)
        .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);
    
    // Create a paragraph widget to render inside the scroll view (no wrap needed since we pre-wrapped)
    let paragraph = Paragraph::new(content_text);
    
    // Render the paragraph inside the scroll view
    let scroll_area = Rect::new(0, 0, content_width, content_height);
    scroll_view.render_widget(paragraph, scroll_area);
    
    // Create a bordered area for the scroll view
    let block = Block::default().borders(Borders::ALL).title(title);
    let inner_area = block.inner(content_area);
    
    // Render the block first, then the scroll view inside
    f.render_widget(block, content_area);
    f.render_stateful_widget(scroll_view, inner_area, scroll_view_state);

    // Render choices area if choices are available
    if !current_choices.is_empty() {
        let choices_area = main_layout[1];
        
        // Choices area with margins
        let choices_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),  // Left margin
                Constraint::Percentage(80),  // Choices content
                Constraint::Percentage(10),  // Right margin
            ])
            .split(choices_area);

        let choices_content_area = choices_chunks[1];

        // Convert choices to ListItems
        let choice_items: Vec<ListItem> = current_choices
            .iter()
            .map(|choice| ListItem::new(choice.as_str()))
            .collect();

        // Create the choices list with highlighting
        let choices_list = List::new(choice_items)
            .block(Block::default().borders(Borders::ALL).title("↑↓ arrows + Enter to select | j/k or mouse wheel to scroll"))
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        // Render the choices list with state for selection
        f.render_stateful_widget(choices_list, choices_content_area, choice_list_state);
    }
}

// Helper function to wrap text to a specific width
fn wrap_text_to_width(text: &str, width: usize) -> Vec<String> {
    let mut wrapped_lines = Vec::new();
    
    for line in text.lines() {
        if line.is_empty() {
            wrapped_lines.push(String::new());
            continue;
        }
        
        let mut current_line = String::new();
        
        for word in line.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                wrapped_lines.push(current_line);
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            wrapped_lines.push(current_line);
        }
    }
    
    wrapped_lines
}