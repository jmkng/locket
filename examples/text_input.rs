use locket::{
    components::TextInput,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    quit, Command, Message, Model,
};

/// Display an input field to collect a name, and display a greeting.
fn main() {
    let model = InputModel {
        prompt: "What is your name?".to_string(),
        input: TextInput::new(29),
        name: None,
    };

    locket::execute(model).unwrap();
}

struct InputModel {
    prompt: String,
    input: TextInput,
    name: Option<String>,
}

impl Model for InputModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(quit)),
                    _ => return None,
                }
            }

            // TODO: ^ check will be pretty common, might need a macro to help with that.

            match key_event.code {
                // `Enter` will mean the user is done typing.
                KeyCode::Enter => {
                    let buffer_text = self.input.buffer();
                    if !buffer_text.is_empty() {
                        self.name = Some(buffer_text);
                        self.input.clear();
                    }
                    return None;
                }
                // Pass any other keystrokes through to the input component.
                _ => self.input.handle_key(*key_event),
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
