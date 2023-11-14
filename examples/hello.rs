use locket::{
    command::{self, Command},
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Message, Model,
};

struct HelloModel {
    last_key: Option<char>,
}

impl Model for HelloModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(command::quit)),
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
        format!("Hello! You pressed: {:?}", self.last_key)
    }
}

fn main() {
    let model = HelloModel { last_key: None };
    locket::execute(model).expect("failed to execute model");
}
