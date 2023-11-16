use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::{components::TextInput, Command, Message, Model};

/// Display an input field to collect a name, and display a greeting.
fn main() {
    let model = InputModel {
        prompt: "What is your name?".to_string(),
        name: None,
        input: TextInput::default(), // Equivalent to: TextInput::new(15, 29)
    };

    locket::execute(model).unwrap();
}

struct InputModel {
    // A prompt question to ask the user for their name.
    prompt: String,
    // The name given by the user.
    //
    // When populated, a greeting is displayed beneath the input.
    name: Option<String>,

    // Nested `TextInput` component will handle the input field.
    input: TextInput,
}

impl Model for InputModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
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

                _ => {}
            }
        };

        // Propagate message to nested components.
        // ↓
        self.input.update(message);

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
