use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
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
    input_buffer: String,
    config: GameConfig,
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
            input_buffer: String::new(),
            config,
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
            self.all_text.push(text.to_string());
        }
    }

    pub fn display_choices(&mut self, choices: &[&Choice]) {
        // Clear previous choices and add new ones to the main text
        self.current_choices.clear();
        if !choices.is_empty() {
            self.all_text.push("".to_string()); // Add spacing
            for (i, choice) in choices.iter().enumerate() {
                let choice_text = format!("{}: {}", i + 1, choice.text);
                self.current_choices.push(choice_text.clone());
                self.all_text.push(choice_text);
            }
        }
    }

    pub fn add_user_input(&mut self, input: &str) {
        // Show what the user typed
        self.all_text.push(format!("> {}", input));
        self.all_text.push("".to_string()); // Add spacing after input
    }

    pub fn get_input(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.input_buffer.clear();
        
        loop {
            self.draw()?;
            
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Enter => {
                                let input = self.input_buffer.clone();
                                self.add_user_input(&input);
                                self.input_buffer.clear();
                                return Ok(input);
                            }
                            KeyCode::Char(c) => {
                                self.input_buffer.push(c);
                            }
                            KeyCode::Backspace => {
                                self.input_buffer.pop();
                            }
                            KeyCode::Esc => {
                                return Ok("quit".to_string());
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
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(5),        // Story text area (scrollable)
                    Constraint::Length(3),     // Input area
                ])
                .split(f.area());

            // Story area with centered content
            let story_area = chunks[0];
            let centered_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(10),  // Left margin
                    Constraint::Percentage(80),  // Content
                    Constraint::Percentage(10),  // Right margin
                ])
                .split(story_area)[1];

            // Calculate how many lines we can display
            let available_height = (centered_area.height as usize).saturating_sub(2); // Account for borders
            let total_lines = self.all_text.len();
            
            // Show the most recent text (scroll to bottom)
            let start_index = if total_lines > available_height {
                total_lines - available_height
            } else {
                0
            };
            
            let visible_text = self.all_text[start_index..].join("\n");

            let story_paragraph = Paragraph::new(visible_text)
                .block(Block::default().borders(Borders::ALL).title("Adventure"))
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Left);

            f.render_widget(story_paragraph, centered_area);

            // Input area
            let input_paragraph = Paragraph::new(format!("> {}", self.input_buffer))
                .block(Block::default().borders(Borders::ALL).title("Input"));

            f.render_widget(input_paragraph, chunks[1]);
        })?;

        Ok(())
    }

    pub fn clear_text(&mut self) {
        // Don't clear all text, just add a separator for new rooms
        self.all_text.push("".to_string());
        self.all_text.push("─".repeat(60));
        self.all_text.push("".to_string());
    }
}