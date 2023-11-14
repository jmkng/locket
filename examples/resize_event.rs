use locket::{command::Command, event::ResizeEvent, Message, Model};

struct ResizeModel {
    terminal_x: u16,
    terminal_y: u16,
    moved: bool,
}

impl Model for ResizeModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
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
            "Resize terminal".to_string()
        }
    }
}

fn main() {
    let model = ResizeModel {
        terminal_x: 0,
        terminal_y: 0,
        moved: false,
    };

    locket::execute(model).unwrap();
}
