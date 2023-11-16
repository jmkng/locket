use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::{components::TextInput, Command, Message, Model};

/// Display an input field to collect a name, and display a greeting.
fn main() {
    let model = InputModel {
        prompt: "What is your name?".to_string(),
        name: None,
        input: TextInput::default(), // Equivalent to: TextInput::new(29, 231)
    };

    locket::execute(model).unwrap();
}

struct InputModel {
    prompt: String,
    name: Option<String>,

    input: TextInput,
}

impl Model for InputModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(event) = msg.downcast::<KeyEvent>() {
            locket::with_exit!(event);

            match event.code {
                // `Enter` means the user is done typing.
                KeyCode::Enter => {
                    let buffer_text = self.input.buffer();
                    if !buffer_text.is_empty() {
                        self.name = Some(buffer_text);
                        self.input.clear();
                    }
                    return None;
                }
                // Other keystrokes go to the nested TextInput.
                _ => self.input.handle_key(*event),
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
