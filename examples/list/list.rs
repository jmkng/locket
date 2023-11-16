use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::font::FontBuilder;
use locket::{components::List, font::MAROON, Command, Message, Model};

/// Display an input field to collect a name, and display a greeting.
fn main() {
    let model = ListModel {
        list: List::new(vec!["One".into(), "Two".into(), "Three".into()], MAROON),
        choice: None,
    };

    locket::execute(model).unwrap();
}

struct ListModel {
    // Nested `List` component will handle the list.
    list: List,

    choice: Option<String>,
}

impl Model for ListModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            locket::with_exit!(event);

            match event.code {
                // Update the selected item.
                KeyCode::Enter => {
                    self.choice = self.list.selected();
                }

                _ => {}
            }
        };

        // Propagate message to nested components.
        // ↓
        self.list.update(message);

        None
    }

    fn view(&self) -> String {
        format!(
            "{}\n\n{}\n\n{}",
            "Select one:",
            self.list.view(),
            self.choice.as_ref().map_or(String::new(), |n| format!(
                "You selected: {}",
                FontBuilder::new(n).underline().bold().to_string()
            ))
        )
    }
}
