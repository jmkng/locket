use std::fmt::Write;

use crate::crossterm::event::{KeyCode, KeyEvent};
use crate::font::fill;
use crate::{Command, Message, Model};

/// List component.
///
/// Displays a vertical list of items.
pub struct List {
    /// List items.
    items: Vec<String>,
    /// Index of the selected list item.
    position: Option<usize>,
    /// The font color of the selected item.
    fill: u8,
}

impl Model for List {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            match event.code {
                // Movement.
                KeyCode::Up => self.handle_up(),
                KeyCode::Down => self.handle_down(),

                // No action.
                _ => {}
            }
        };

        None
    }

    fn view(&self) -> String {
        let mut buffer = String::new();

        for (item_index, item) in self.items.iter().enumerate() {
            let string = if self.position.map_or(false, |x| x == item_index) {
                format!("{}", fill(item, self.fill))
            } else {
                format!("{item}")
            };

            write!(buffer, "{string}").unwrap();
            if item_index != self.items.len() - 1 {
                write!(buffer, "\n").unwrap();
            }
        }

        buffer
    }
}

impl List {
    /// Return a new instance of `Self`.
    pub fn new(items: Vec<String>, fill: u8) -> Self {
        Self {
            items,
            position: None,
            fill,
        }
    }

    /// Deselect the selected item, if any.
    pub fn deselect(&mut self) {
        self.position = None;
    }

    /// Return the selected item as a string.
    pub fn selected(&self) -> Option<String> {
        if let Some(index) = self.position {
            self.items.get(index).cloned()
        } else {
            None
        }
    }

    /// Return the index of the selected option.
    pub fn index(&self) -> Option<usize> {
        self.position
    }

    /// Move the cursor up.
    fn handle_up(&mut self) {
        if let Some(index) = self.position {
            if index > 0 {
                self.position = Some(index - 1);
            }
        } else {
            // Start from the bottom.
            self.position = Some(self.items.len() - 1);
        }
    }

    /// Move the cursor down.
    fn handle_down(&mut self) {
        if let Some(index) = self.position {
            if index < self.items.len() - 1 {
                self.position = Some(index + 1)
            }
        } else {
            // Start from the top.
            self.position = Some(0);
        }
    }
}
