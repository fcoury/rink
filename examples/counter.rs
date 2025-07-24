use rink::prelude::*;

struct CounterApp {
    count: i32,
}

impl Component for CounterApp {
    fn render(&self) -> RenderTree {
        RenderTree {
            elements: vec![
                Element {
                    x: 5,
                    y: 2,
                    width: 20,
                    height: 1,
                    content: format!("Count: {}", self.count),
                    style: Style::default(),
                },
                Element {
                    x: 5,
                    y: 4,
                    width: 30,
                    height: 1,
                    content: "Press 'j' to increment, 'k' to decrement, 'q' to quit".to_string(),
                    style: Style {
                        fg: Some(Color::DarkGrey),
                        ..Style::default()
                    },
                },
            ],
        }
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<EventResult> {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char('j') => {
                    self.count += 1;
                    Ok(EventResult::Handled)
                }
                KeyCode::Char('k') => {
                    self.count -= 1;
                    Ok(EventResult::Handled)
                }
                KeyCode::Char('q') => {
                    return Ok(EventResult::Quit);
                }
                _ => Ok(EventResult::NotHandled),
            }
        } else {
            Ok(EventResult::NotHandled)
        }
    }

    fn get_size(&self) -> (u16, u16) {
        (40, 10)
    }
}

fn main() -> anyhow::Result<()> {
    let counter_app = CounterApp { count: 0 };

    let mut app = App::new(Box::new(counter_app))?;
    app.run()
}
