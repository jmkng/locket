use locket::{
    command::{self, Command},
    components::Input,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Message, Model,
};

struct InputModel {
    prompt: String,
    input: Input,
    name: Option<String>,
}

impl Model for InputModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        // TODO: Extract to helper function.
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(command::quit)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Enter => {
                    let buffer_text = self.input.buffer();
                    if !buffer_text.is_empty() {
                        self.name = Some(buffer_text);
                        self.input.clear();
                    }

                    // return Some(Box::new(|| command::quit()));
                    return None;
                }
                // Pass any other keystrokes through to the input component.
                _ => self.input.on_key_event(*key_event),
            }
        };

        None
    }

    fn view(&self) -> String {
        format!(
            "{}\n\n{}\n\n{}",
            self.prompt,
            self.input.view(),
            self.name
                .as_ref()
                .map_or(String::new(), |n| format!("Hello, {}!", n))
        )
    }
}

fn main() {
    let model = InputModel {
        prompt: "What is your name?".to_string(),
        input: Input::new(),
        name: None,
    };

    locket::execute(model).unwrap();
}
