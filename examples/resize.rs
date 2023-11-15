use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use locket::{event::ResizeEvent, quit, Command, Message, Model};

/// Display the terminal dimensions as it is resized.
fn main() {
    let model = ResizeModel {
        terminal_x: 0,
        terminal_y: 0,
        moved: false,
    };

    locket::execute(model).unwrap();
}

struct ResizeModel {
    terminal_x: u16,
    terminal_y: u16,
    moved: bool,
}

impl Model for ResizeModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(quit)),
                    _ => return None,
                }
            }
        };

        if let Ok(resize_event) = msg.downcast::<ResizeEvent>() {
            self.moved = true;
            self.terminal_x = resize_event.0;
            self.terminal_y = resize_event.1;
        }

        None
    }

    fn view(&self) -> String {
        if self.moved {
            format!(
                "Terminal size: (x: {}, y: {})",
                self.terminal_x, self.terminal_y
            )
        } else {
            "Resize the terminal to begin.".to_string()
        }
    }
}
