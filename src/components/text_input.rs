use std::fmt::Write;

use crate::crossterm::event::{KeyCode, KeyEvent};
use crate::{
    font::{fill_background, WHITE},
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
        Self::new(WHITE, 29)
    }
}

impl Model for TextInput {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(event) = msg.downcast::<KeyEvent>() {
            match event.code {
                KeyCode::Enter => {
                    self.clear();
                    return None;
                }
                _ => self.handle_key(*event),
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
    pub fn buffer(&self) -> String {
        self.buffer.clone()
    }

    /// Set the buffer to the given string.
    ///
    /// The cursor position is moved to the end of the buffer.
    pub fn set_buffer(&mut self, buffer: String) {
        self.position = buffer.len();
        self.buffer = buffer;
    }

    /// Return the current position within the buffer.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the position.
    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    /// Respond to a `KeyEvent`.
    ///
    /// This is a dispatch function that will call some `handle_*` function
    /// based on the key code.
    pub fn handle_key(&mut self, event: KeyEvent) {
        match event.code {
            // Movement keys.
            KeyCode::Left => self.handle_left(),
            KeyCode::Right => self.handle_right(),

            // Delete.
            KeyCode::Backspace => self.handle_backspace(),

            // Insert.
            KeyCode::Char(c) => self.handle_char(c),

            // No response.
            _ => {}
        }
    }

    /// Delete one character behind the current position, and move the cursor back.
    ///
    /// Response to a `KeyEvent::Backspace`.
    fn handle_backspace(&mut self) {
        if self.position > 0 {
            self.buffer.remove(self.position - 1);
            self.position -= 1;
        }
    }

    /// Insert the character the current position, and move the cursor forward.
    ///
    /// Response to a `KeyEvent::Char`
    fn handle_char(&mut self, c: char) {
        self.buffer.insert(self.position, c);
        self.position += 1;
    }

    /// Move the cursor to the left, if possible.
    ///
    /// Response to a `KeyEvent::Left`
    fn handle_left(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    /// Move the cursor to the right, if possible.
    ///
    /// Response to a `KeyEvent::Right`
    fn handle_right(&mut self) {
        if self.position < self.buffer.len() {
            self.position += 1;
        }
    }
}
