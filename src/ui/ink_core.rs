use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutNode {
    pub width: u16,
    pub height: u16,
    pub flex_direction: String,
}

pub struct InkRenderer {
    pub last_frame: String,
}

impl InkRenderer {
    pub fn new() -> Self {
        Self { last_frame: String::new() }
    }

    pub fn render(&mut self, content: &str) {
        // Ported from ink/renderer.ts
        self.last_frame = content.to_string();
    }
}

pub struct TerminalManager {
    pub width: u16,
    pub height: u16,
}

impl TerminalManager {
    pub fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
        Self { width: w, height: h }
    }

    pub fn clear(&self) {
        // Ported from ink/clearTerminal.ts
        println!("\x1B[2J\x1B[1;1H");
    }
}

pub struct EventDispatcher {
    // Ported from ink/events/dispatcher.ts
}

impl EventDispatcher {
    pub fn new() -> Self { Self {} }
    pub fn dispatch(&self, _event: String) {
        // Event handling logic
    }
}

// Hooks (Simplified Rust versions)
pub struct InkHooks;
impl InkHooks {
    pub fn use_input<F>(mut handler: F) where F: FnMut(char) {
        // Logic similar to use-input.ts
    }
}
