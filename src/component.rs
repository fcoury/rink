use crossterm::event::Event;

use crate::style::Style;

pub enum EventResult {
    Handled,
    NotHandled,
    Quit,
}

pub trait Component {
    fn render(&self) -> RenderTree;
    fn handle_event(&mut self, event: Event) -> anyhow::Result<EventResult>;
    fn get_size(&self) -> (u16, u16);
}

pub struct RenderTree {
    pub elements: Vec<Element>,
}

pub struct Element {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub content: String,
    pub style: Style,
}
