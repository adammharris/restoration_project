use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GameError {
    IoError(std::io::Error),
    MissingRoom(String),
    MissingChoice(String),
    InvalidStartingRoom(String),
    ValidationError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::IoError(err) => write!(f, "IO error: {}", err),
            GameError::MissingRoom(room_id) => write!(f, "Room '{}' referenced but not defined", room_id),
            GameError::MissingChoice(choice_id) => write!(f, "Choice '{}' referenced but not defined", choice_id),
            GameError::InvalidStartingRoom(room_id) => write!(f, "Starting room '{}' does not exist", room_id),
            GameError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GameError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> Self {
        GameError::IoError(err)
    }
}

pub type GameResult<T> = Result<T, GameError>;