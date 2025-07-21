use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::errors::{GameError, GameResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub typewriter_speed_ms: u64,
    pub enable_typewriter: bool,
    pub allow_text_commands: bool,
    pub auto_save: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            typewriter_speed_ms: 30,
            enable_typewriter: true,
            allow_text_commands: true,
            auto_save: false,
        }
    }
}

impl GameConfig {
    pub fn load_or_create() -> GameResult<Self> {
        let config_path = "restoration_config.json";
        
        if Path::new(config_path).exists() {
            let content = fs::read_to_string(config_path)?;
            let config: GameConfig = serde_json::from_str(&content)
                .map_err(|e| GameError::ValidationError(format!("Failed to parse config: {}", e)))?;
            Ok(config)
        } else {
            let default_config = GameConfig::default();
            default_config.save()?;
            Ok(default_config)
        }
    }
    
    pub fn save(&self) -> GameResult<()> {
        let config_str = serde_json::to_string_pretty(self)
            .map_err(|e| GameError::ValidationError(format!("Failed to serialize config: {}", e)))?;
        fs::write("restoration_config.json", config_str)?;
        Ok(())
    }
}