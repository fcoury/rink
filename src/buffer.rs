use crate::style::Style;

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

pub struct Buffer {
    pub cells: Vec<Cell>,
    pub width: u16,
    pub height: u16,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        let cells = vec![
            Cell {
                ch: ' ',
                style: Style::default(),
            };
            (width * height) as usize
        ];
        Buffer {
            cells,
            width,
            height,
        }
    }

    pub fn diff(&self, other: &Buffer) -> Vec<(u16, u16, Cell)> {
        let mut changes = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                if self.cells[idx] != other.cells[idx] {
                    changes.push((x, y, other.cells[idx].clone()));
                }
            }
        }
        changes
    }

    pub fn resize(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
        self.cells.resize(
            (new_width * new_height) as usize,
            Cell {
                ch: ' ',
                style: Style::default(),
            },
        );
    }

    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) -> anyhow::Result<()> {
        if x < self.width && y < self.height {
            self.cells[(y * self.width + x) as usize] = cell;
            Ok(())
        } else {
            anyhow::bail!("Index out of bounds: ({}, {})", x, y);
        }
    }
}
