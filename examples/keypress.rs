use locket::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    quit, Command, Message, Model,
};

/// Display keyboard input as it is received.
fn main() {
    let model = KeypressModel { last_key: None };

    locket::execute(model).unwrap();
}

struct KeypressModel {
    last_key: Option<char>,
}

impl Model for KeypressModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(quit)),
                    _ => return None,
                }
            }

            match key_event.code {
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
