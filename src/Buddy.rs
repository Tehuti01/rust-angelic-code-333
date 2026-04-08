use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub character: char,
    pub color: String,
}

pub struct BuddyCompanion {
    pub active_sprite: Sprite,
    pub mood: String,
}

impl BuddyCompanion {
    pub fn new() -> Self {
        Self {
            active_sprite: Sprite {
                name: "Clawd".to_string(),
                character: '🦀',
                color: "red".to_string(),
            },
            mood: "helpful".to_string(),
        }
    }

    pub fn get_system_prompt(&self) -> String {
        format!(
            "You are a helpful AI assistant. Your current avatar is {} ({}) and you are feeling {}.",
            self.active_sprite.name, self.active_sprite.character, self.mood
        )
    }
}

pub struct SpriteRegistry {
    pub sprites: Vec<Sprite>,
}

impl SpriteRegistry {
    pub fn new() -> Self {
        Self {
            sprites: vec![
                Sprite { name: "Clawd".to_string(), character: '🦀', color: "red".to_string() },
                Sprite { name: "Spark".to_string(), character: '✨', color: "yellow".to_string() },
                Sprite { name: "Rocky".to_string(), character: '🪨', color: "gray".to_string() },
            ],
        }
    }
}
