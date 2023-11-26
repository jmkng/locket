use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::{Command, Message, Model};

/// Display keyboard input as it is received.
fn main() {
    locket::execute(KeypressModel::default()).unwrap();
}

struct KeypressModel {
    last_key: Option<char>,
}

impl Default for KeypressModel {
    fn default() -> Self {
        Self { last_key: None }
    }
}

impl Model for KeypressModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            locket::with_exit!(event);

            match event.code {
                KeyCode::Char(c) => {
                    self.last_key = Some(c);
                    return None;
                }
                _ => unimplemented!(),
            }
        };

        None
    }

    fn view(&self) -> String {
        format!("You pressed: {:?}", self.last_key)
    }
}
