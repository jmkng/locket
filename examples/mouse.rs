use locket::crossterm::event::{MouseEvent, MouseEventKind};
use locket::{exit, Command, Message, Model};

/// Display the cursor position as it moves within the terminal.
fn main() {
    let model = MouseModel { col: 0, row: 0 };

    locket::with_mouse_capture!().unwrap();
    locket::execute(model).unwrap();
}

struct MouseModel {
    col: u16,
    row: u16,
}

impl Model for MouseModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(mouse_event) = msg.downcast::<MouseEvent>() {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                return Some(Box::new(exit));
            }
            self.col = mouse_event.column;
            self.row = mouse_event.row;
        }

        None
    }

    fn view(&self) -> String {
        format!("Click to exit. Row: {}, Column: {}", self.col, self.row)
    }
}
