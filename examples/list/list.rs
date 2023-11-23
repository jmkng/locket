use locket::crossterm::event::{KeyCode, KeyEvent};
use locket::font::FontBuilder;
use locket::{components::List, font::MAROON, Command, Message, Model};

/// Display an input field to collect a name, and display a greeting.
fn main() {
    let model = ListModel {
        list: List::new(
            vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
            ]
            .into_iter()
            .map(|n| n.to_string()),
            5,
            1,
            MAROON,
        ),
        choice: None,
    };

    locket::execute(model).unwrap();
}

struct ListModel {
    choice: Option<String>,

    // Nested `List` component will handle the list.
    list: List,
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
