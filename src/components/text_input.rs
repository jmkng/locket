use std::fmt::Write;

use crate::crossterm::event::{KeyCode, KeyEvent};
use crate::{
    font::{fill_background, MAROON, WHITE},
    Command, Message, Model,
};

/// Text input component.
///
/// Displays a single line text field.
pub struct TextInput {
    /// Internal buffer containing the text in the field.
    buffer: String,
    /// Position of cursor within the buffer.
    position: usize,
    /// The font color of the character under the cursor.
    fill: u8,
    /// The background color of the cursor.
    background: u8,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new(WHITE, MAROON)
    }
}

impl Model for TextInput {
    fn update(&mut self, message: &Message) -> Option<Command> {
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            match event.code {
                // Movement.
                KeyCode::Left => self.handle_left(),
                KeyCode::Right => self.handle_right(),

                // Delete.
                KeyCode::Backspace => self.handle_backspace(),

                // Insert.
                KeyCode::Char(c) => self.handle_char(c),

                // No action.
                _ => {}
            }
        };

        None
    }

    fn view(&self) -> String {
        if self.buffer.is_empty() {
            return fill_background(" ", self.fill, self.background);
        }

        let mut buffer = String::with_capacity(self.buffer.len());
        buffer.write_str(&self.buffer[..self.position]).unwrap();
        if self.position < self.buffer.len() {
            let cursor_char: String = self.buffer[self.position..=self.position].to_string();
            buffer
                .write_str(&fill_background(cursor_char, self.fill, self.background))
                .unwrap();
            buffer.write_str(&self.buffer[self.position + 1..]).unwrap();
        } else {
            buffer
                .write_str(&fill_background(" ", self.fill, self.background))
                .unwrap();
        }

        buffer
    }
}

impl TextInput {
    /// Return a new instance of `Input`.
    pub fn new(fill: u8, background: u8) -> Self {
        Self {
            buffer: String::new(),
            position: 0,
            fill,
            background,
        }
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
    }

    /// Return the buffer.
    ///
    /// This action does not clear the buffer, to do that you should also
    /// call `.clear`.
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// Set the buffer to the given string.
    ///
    /// The cursor position is moved to the end of the buffer.
    pub fn set_buffer<T>(&mut self, buffer: T)
    where
        T: Into<String>,
    {
        let as_string = buffer.into();

        self.position = as_string.len();
        self.buffer = as_string;
    }

    /// Return the current position within the buffer.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the position.
    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    /// Delete one character behind the current position, and move the cursor back.
    fn handle_backspace(&mut self) {
        if self.position > 0 {
            self.buffer.remove(self.position - 1);
            self.position -= 1;
        }
    }

    /// Insert the character at the current position, and move the cursor forward.
    fn handle_char(&mut self, c: char) {
        self.buffer.insert(self.position, c);
        self.position += 1;
    }

    /// Move the cursor to the left, if possible.
    fn handle_left(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    /// Move the cursor to the right, if possible.
    fn handle_right(&mut self) {
        if self.position < self.buffer.len() {
            self.position += 1;
        }
    }
}
