use crate::world::Choice;
use std::error::Error;

/// Trait for abstracting text adventure game UI interactions.
/// This allows the same game logic to work with both terminal and web UIs.
pub trait GameUI {
    /// Display multiple texts with user-controlled pacing.
    /// Each text is shown one at a time, with the user pressing a key/button to continue.
    /// The last text does not wait for user input.
    fn display_texts(&mut self, texts: &[String]) -> Result<(), Box<dyn Error>>;
    
    /// Display a single text immediately without waiting.
    fn display_text(&mut self, text: &str) -> Result<(), Box<dyn Error>>;
    
    /// Show available choices to the user.
    fn display_choices(&mut self, choices: &[&Choice]);
    
    /// Get the user's choice selection (returns choice index).
    fn get_user_choice(&mut self) -> Result<usize, Box<dyn Error>>;
    
    /// Clear currently displayed choices.
    fn clear_choices(&mut self);
    
    /// Add a visual separator (for room transitions).
    fn add_separator(&mut self);
    
    /// Clean up resources (terminal mode, etc.).
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Helper trait for UIs that support waiting for user input.
pub trait WaitForInput {
    /// Wait for the user to press Enter/continue button.
    fn wait_for_continue(&mut self) -> Result<(), Box<dyn Error>>;
}