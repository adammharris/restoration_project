use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

use crate::config::GameConfig;
use crate::world::{Choice, World};
use crate::game::GameState;

pub struct TerminalUi {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    all_text: Vec<String>,  // All accumulated text including story and responses
    current_choices: Vec<String>,
    selected_choice: usize,  // Currently selected choice index
    config: GameConfig,
    list_state: ListState,
    scroll_state: ScrollbarState,
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
            list_state: ListState::default(),
            scroll_state: ScrollbarState::default(),
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
                self.add_wrapped_text(text);
                self.scroll_to_bottom();
            }
        }
    }

    fn add_wrapped_text(&mut self, text: &str) {
        // Calculate available width (account for borders and margins)
        let available_width = 60; // Conservative estimate for content width
        let wrapped_lines = self.wrap_text(text, available_width);
        self.all_text.extend(wrapped_lines);
    }

    fn wrap_text(&self, text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        if lines.is_empty() {
            lines.push(text.to_string()); // Fallback for edge cases
        }
        
        lines
    }

    fn display_text_with_typewriter(&mut self, text: &str) {
        // For typewriter effect, we'll show character by character but still wrap properly
        let available_width = 60; // Conservative estimate for content width
        let wrapped_lines = self.wrap_text(text, available_width);
        
        for line in wrapped_lines {
            // Add a new line for this wrapped line
            self.all_text.push(String::new());
            let current_line_index = self.all_text.len() - 1;
            
            let mut current_text = String::new();
            
            for ch in line.chars() {
                current_text.push(ch);
                
                // Update the current line with the progress
                self.all_text[current_line_index] = current_text.clone();
                
                // Auto-scroll to bottom
                self.scroll_to_bottom();
                
                // Draw the updated display
                let _ = self.draw();
                
                // Sleep for typewriter effect
                std::thread::sleep(std::time::Duration::from_millis(self.config.typewriter_speed_ms));
            }
            
            // Ensure the final text is set for this line
            self.all_text[current_line_index] = line;
        }
        
        self.scroll_to_bottom();
    }

    fn scroll_to_bottom(&mut self) {
        let total_items = self.all_text.len();
        if total_items > 0 {
            self.list_state.select(Some(total_items - 1));
            self.scroll_state = self.scroll_state.content_length(total_items);
            self.scroll_state = self.scroll_state.position(total_items.saturating_sub(1));
        }
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

    pub fn get_selected_choice(&self) -> usize {
        self.selected_choice
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
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
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
                                self.move_selection_up();
                            }
                            KeyCode::Down => {
                                self.move_selection_down();
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
                }
            }
        }
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let main_layout = if !self.current_choices.is_empty() {
                // Split into story area and choices area when choices are available
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(10),        // Story area
                        Constraint::Length(self.current_choices.len() as u16 + 2), // Choices area (with borders)
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
            
            // Story area layout with margins and scrollbar
            let story_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(10),  // Left margin
                    Constraint::Percentage(78),  // Content (reduced for scrollbar)
                    Constraint::Percentage(2),   // Scrollbar
                    Constraint::Percentage(10),  // Right margin
                ])
                .split(story_area);

            let content_area = story_chunks[1];
            let scrollbar_area = story_chunks[2];

            // Convert text lines to ListItems
            let items: Vec<ListItem> = self.all_text
                .iter()
                .map(|line| ListItem::new(line.as_str()))
                .collect();

            // Create the scrollable story list
            let story_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Adventure"))
                .style(Style::default());

            // Render the story list with state for scrolling
            f.render_stateful_widget(story_list, content_area, &mut self.list_state);

            // Render scrollbar
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            f.render_stateful_widget(scrollbar, scrollbar_area, &mut self.scroll_state);

            // Render choices area if choices are available
            if !self.current_choices.is_empty() {
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
                let choice_items: Vec<ListItem> = self.current_choices
                    .iter()
                    .map(|choice| ListItem::new(choice.as_str()))
                    .collect();

                // Create the choices list with highlighting
                let choices_list = List::new(choice_items)
                    .block(Block::default().borders(Borders::ALL).title("Use ↑↓ arrows and Enter to select"))
                    .style(Style::default())
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

                // Render the choices list with state for selection
                f.render_stateful_widget(choices_list, choices_content_area, &mut self.choice_list_state);
            }
        })?;

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