use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    command::{self, Command},
    Message, Model,
};

/// A helper struct for creating a user input field.
///
/// It is very minimal and leaves all control of rendering to the user.
/// You are able to access the buffer and caret pos, and render the caret however you please.
pub struct Input {
    buffer: String,
    pos: usize,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Model for Input {
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
                    // self.name = Some(self.buffer()); // TODO: Handle in outer implementation.
                    self.clear();
                    // return Some(Box::new(|| command::quit()));
                    return None;
                }
                _ => self.on_key_event(*key_event),
            }
        };

        None
    }

    fn view(&self) -> String {
        // let prompt = "Enter your name: "; // TODO: Handle in outer implementation.
        let input_buffer = self.buffer();
        let cursor_position = self.pos();

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
        format!(
            "{}{}{}",
            visible_before,
            if cursor_position <= input_buffer.len() {
                "|"
            } else {
                ""
            },
            visible_after,
        )
    }
}

impl Input {
    /// Simple contructor. Starts with an empty buffer.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            pos: 0,
        }
    }

    /// Emptys the buffer and resets the pos.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
    }

    /// Returns the current buffer.
    pub fn buffer(&self) -> String {
        self.buffer.clone()
    }

    /// Overwrites the current buffer with the given string,
    /// and sets the pos to the end of it.
    pub fn set_buffer(&mut self, buffer: String) {
        self.pos = buffer.len();
        self.buffer = buffer;
    }

    /// Returns the current position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets the current position.
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Recieves crossterm `KeyEvent`s and updates the buffer and caret position.
    ///
    /// It handles:
    /// * Backspace. Deletes one character back from the current pos, and steps the pos back.
    /// * Chars. Inserts the character at the current pos, and steps the pos forward.
    /// * Left. Steps the pos back if possible.
    /// * Right. Steps the pos forward if possible.
    pub fn on_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => {
                if self.pos > 0 {
                    self.buffer.remove(self.pos - 1);
                    self.pos -= 1;
                }
            }
            KeyCode::Char(c) => {
                self.buffer.insert(self.pos, c);
                self.pos += 1;
            }
            KeyCode::Left => {
                if self.pos > 0 {
                    self.pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.pos < self.buffer.len() {
                    self.pos += 1;
                }
            }
            _ => (),
        }
    }
}
