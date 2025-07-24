use std::io;

use crossterm::{cursor, execute, style::Print};

pub struct Terminal {
    width: u16,
    height: u16,
}

impl Terminal {
    pub fn new(width: u16, height: u16) -> Self {
        Terminal { width, height }
    }

    pub fn write_at(&self, x: u16, y: u16, text: &str) -> anyhow::Result<()> {
        execute!(io::stdout(), cursor::MoveTo(x, y), Print(text))
            .map_err(|e| anyhow::anyhow!("Failed to write at position ({}, {}): {}", x, y, e))
    }

    pub fn resize(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
