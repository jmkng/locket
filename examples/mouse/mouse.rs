use locket::crossterm::event::{MouseEvent, MouseEventKind};
use locket::{exit, Command, Message, Model};

/// Display the cursor position as it moves within the terminal.
fn main() {
    locket::with_mouse_capture!().expect("must be able to capture mouse");
    locket::execute(MouseModel::new(0, 0)).unwrap();
}

struct MouseModel {
    col: usize,
    row: usize,
}

impl MouseModel {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

impl Model for MouseModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(mouse_event) = message.downcast_ref::<MouseEvent>() {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                return Some(Box::new(exit));
            }
            self.col = mouse_event.column.into();
            self.row = mouse_event.row.into();
        }

        None
    }

    fn view(&self) -> String {
        format!("Click to exit. Row: {}, Column: {}", self.col, self.row)
    }
}
