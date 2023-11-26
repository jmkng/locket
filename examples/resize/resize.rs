use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::{event::ResizeEvent, Command, Message, Model};

/// Display the terminal dimensions as it is resized.
fn main() {
    locket::execute(ResizeModel::default()).unwrap();
}

struct ResizeModel {
    terminal_x: u16,
    terminal_y: u16,
    moved: bool,
}

impl Default for ResizeModel {
    fn default() -> Self {
        Self {
            terminal_x: 0,
            terminal_y: 0,
            moved: false,
        }
    }
}

impl Model for ResizeModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            locket::with_exit!(event);
        };
        if let Some(resize_event) = message.downcast_ref::<ResizeEvent>() {
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
