use crossterm::event::{KeyCode, KeyEvent};
use locket::{components::List, font::MAROON, Command, Message, Model};

/// Display a simple paginated list of items.
fn main() {
    locket::execute(ListModel::default()).unwrap();
}

pub struct ListModel {
    list: List,
}

impl Default for ListModel {
    fn default() -> Self {
        let mut items = Vec::new();
        for i in 0..15 {
            items.push(format!("{i}"))
        }
        assert_eq!(items.len(), 15);

        Self {
            list: List::new(items.into_iter(), 5, MAROON),
        }
    }
}

impl Model for ListModel {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            locket::with_exit!(event);
        };

        let command = self.list.update(message);

        command
    }

    fn view(&self) -> String {
        self.list.view()
    }
}
