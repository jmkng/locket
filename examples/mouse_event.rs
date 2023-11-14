use locket::{
    command::{self, Command},
    crossterm::event::{MouseEvent, MouseEventKind},
    Message, Model,
};

struct MouseModel {
    col: u16,
    row: u16,
}

impl Model for MouseModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(mouse_event) = msg.downcast::<MouseEvent>() {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                return Some(Box::new(command::quit));
            }
            self.col = mouse_event.column;
            self.row = mouse_event.row;
        }

        None
    }

    fn view(&self) -> String {
        format!(
            "Click to terminate. Mouse row: {}, col: {}",
            self.col, self.row
        )
    }
}

fn main() {
    let model = MouseModel { col: 0, row: 0 };

    locket::enable_mouse_capture().unwrap();
    locket::execute(model).expect("failed to execute model");
}
