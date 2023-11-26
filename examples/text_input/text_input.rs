use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::{components::TextInput, Command, Message, Model};

/// Display an input field to collect a name, and display a greeting.
fn main() {
    locket::execute(InputModel::new("What is your name?".to_string())).unwrap();
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

impl InputModel {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            name: None,
            input: TextInput::default(),
        }
    }
}

impl Model for InputModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            locket::with_exit!(event);

            match event.code {
                // Display the greeting.
                KeyCode::Enter => {
                    if !self.input.buffer().is_empty() {
                        self.name = Some(self.input.buffer().to_string());
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
