use std::io::{self, Write as _};

use crossterm::event;

use crate::{
    buffer::{Buffer, Cell},
    component::{Component, RenderTree},
    prelude::EventResult,
    style::Style,
    terminal::Terminal,
};

pub struct App {
    terminal: Terminal,
    current_buffer: Buffer,
    root_component: Box<dyn Component>,
    should_quit: bool,
}

impl App {
    pub fn new(root_component: Box<dyn Component>) -> anyhow::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        let terminal = Terminal::new(width, height);
        let current_buffer = Buffer::new(width, height);

        Ok(App {
            terminal,
            current_buffer,
            root_component,
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.terminal.prepare()?;

        loop {
            if self.should_quit {
                break;
            }

            // Handle events
            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;
                match self.root_component.handle_event(event)? {
                    EventResult::Quit => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }

            // Render
            let render_tree = self.root_component.render();
            let next_buffer = self.render_tree_to_buffer(render_tree);

            // Diff and update
            let changes = self.current_buffer.diff(&next_buffer);
            self.apply_changes(changes)?;
            self.current_buffer = next_buffer;
        }

        self.terminal.cleanup()?;
        Ok(())
    }

    pub fn render_tree_to_buffer(&self, render_tree: RenderTree) -> Buffer {
        let mut buffer = Buffer::new(self.terminal.dimensions().0, self.terminal.dimensions().1);

        for element in render_tree.elements {
            for y in 0..element.height {
                for x in 0..element.width {
                    let cell = Cell {
                        ch: element
                            .content
                            .chars()
                            .nth((y * element.width + x) as usize)
                            .unwrap_or(' '),
                        style: element.style,
                    };
                    buffer.set_cell(element.x + x, element.y + y, cell).unwrap();
                }
            }
        }

        buffer
    }

    fn apply_changes(&mut self, changes: Vec<(u16, u16, Cell)>) -> anyhow::Result<()> {
        if changes.is_empty() {
            return Ok(());
        }

        let mut output = String::new();
        let mut current_style = Style::default();

        for (x, y, cell) in changes {
            // Add cursor movement
            output.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));

            // Add style changes if needed
            if current_style != cell.style {
                output.push_str(&cell.style.to_ansi_codes());
                current_style = cell.style;
            }

            // Add the character
            output.push(cell.ch);
        }

        // Write everything at once
        write!(io::stdout(), "{}", output)?;
        io::stdout().flush()?;
        Ok(())
    }
}
