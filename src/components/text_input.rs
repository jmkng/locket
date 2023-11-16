use std::fmt::Write;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{Command, Message, Model};

/// Text input component.
///
/// Displays a single line text field.
pub struct TextInput {
    /// Internal buffer containing the text in the field.
    buffer: String,
    /// Position of cursor within the buffer.
    pos: usize,
    /// The color of the cursor.
    cursor_color: u8,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new(29)
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
            return self.stroke_as_cursor(" ");
        }
        let mut buffer = String::with_capacity(self.buffer.len());
        buffer.write_str(&self.buffer[..self.pos]).unwrap();
        if self.pos < self.buffer.len() {
            let cursor_char: String = self.buffer[self.pos..=self.pos].to_string();
            buffer
                .write_str(&self.stroke_as_cursor(&cursor_char))
                .unwrap();
            buffer.write_str(&self.buffer[self.pos + 1..]).unwrap();
        } else {
            buffer.write_str(&self.stroke_as_cursor(" ")).unwrap();
        }

        buffer
    }
}

impl TextInput {
    /// Return a new instance of `Input`.
    pub fn new(cursor_color: u8) -> Self {
        Self {
            buffer: String::new(),
            pos: 0,
            cursor_color,
        }
    }

    /// Return a string highlighted with the color in `self.cursor_color`.
    pub fn stroke_as_cursor<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("\x1B[48;5;{}m{}\x1B[0m", self.cursor_color, text.as_ref())
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
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
        self.pos = buffer.len();
        self.buffer = buffer;
    }

    /// Return the current position within the buffer.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Set the position.
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
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
        if self.pos > 0 {
            self.buffer.remove(self.pos - 1);
            self.pos -= 1;
        }
    }

    /// Insert the character the current position, and move the cursor forward.
    ///
    /// Response to a `KeyEvent::Char`
    fn handle_char(&mut self, c: char) {
        self.buffer.insert(self.pos, c);
        self.pos += 1;
    }

    /// Move the cursor to the left, if possible.
    ///
    /// Response to a `KeyEvent::Left`
    fn handle_left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    /// Move the cursor to the right, if possible.
    ///
    /// Response to a `KeyEvent::Right`
    fn handle_right(&mut self) {
        if self.pos < self.buffer.len() {
            self.pos += 1;
        }
    }
}
