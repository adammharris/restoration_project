use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GameError {
    IoError(std::io::Error),
    TomlParseError(toml::de::Error),
    MissingRoom(String),
    MissingChoice(String),
    InvalidStartingRoom(String),
    CircularReference(String),
    ValidationError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::IoError(err) => write!(f, "IO error: {}", err),
            GameError::TomlParseError(err) => write!(f, "TOML parsing error: {}", err),
            GameError::MissingRoom(room_id) => write!(f, "Room '{}' referenced but not defined", room_id),
            GameError::MissingChoice(choice_id) => write!(f, "Choice '{}' referenced but not defined", choice_id),
            GameError::InvalidStartingRoom(room_id) => write!(f, "Starting room '{}' does not exist", room_id),
            GameError::CircularReference(path) => write!(f, "Circular reference detected: {}", path),
            GameError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GameError::IoError(err) => Some(err),
            GameError::TomlParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> Self {
        GameError::IoError(err)
    }
}

impl From<toml::de::Error> for GameError {
    fn from(err: toml::de::Error) -> Self {
        GameError::TomlParseError(err)
    }
}

pub type GameResult<T> = Result<T, GameError>;