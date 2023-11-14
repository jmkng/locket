use locket::{
    command::{self, Command},
    components::Input,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Message, Model,
};

struct InputModel {
    input: Input,
    name: Option<String>,
}

impl Model for InputModel {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(command::quit)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Enter => {
                    self.name = Some(self.input.buffer());
                    self.input.clear();
                    // return Some(Box::new(|| command::quit()));
                    return None;
                }
                _ => self.input.on_key_event(*key_event),
            }
        };

        None
    }

    fn view(&self) -> String {
        let prompt = "Enter your name: ";
        let input_buffer = self.input.buffer();
        let cursor_position = self.input.pos();

        // Adjust the view to ensure both sides of the bar are visible when moving left/right.
        let (visible_before, visible_after) = if cursor_position == 0 {
            (String::new(), input_buffer.clone())
        } else if cursor_position < input_buffer.len() {
            (
                input_buffer[..cursor_position].to_string(),
                input_buffer[cursor_position..].to_string(),
            )
        } else {
            (input_buffer.clone(), String::new())
        };

        // Insert a dynamic vertical bar at the cursor position,
        // this must include the rightmost position.
        let output = format!(
            "{}{}{}{}",
            prompt,
            visible_before,
            if cursor_position <= input_buffer.len() {
                "|"
            } else {
                ""
            },
            visible_after,
        );

        if let Some(name) = &self.name {
            format!("{}\nHello, {}!", output, name)
        } else {
            output
        }
    }
}

fn main() {
    let model = InputModel {
        input: Input::new(),
        name: None,
    };

    locket::execute(model).unwrap();
}
