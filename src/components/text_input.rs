use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{quit, Command, Message, Model};

/// Input component.
///
/// Displays a single line text field.
pub struct TextInput {
    buffer: String,
    pos: usize,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Model for TextInput {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(quit)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Enter => {
                    self.clear();
                    return None;
                }
                _ => self.on_key_event(*key_event),
            }
        };

        None
    }

    fn view(&self) -> String {
        let input_buffer = self.buffer();
        let cursor_position = self.position();

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

        // TODO: Splicing the buffer with a vertical bar to display current position is ugly,
        // should just highlight background of character instead.

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

impl TextInput {
    /// Return a new instance of `Input`.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            pos: 0,
        }
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
    }

    /// Return the buffer.
    ///
    /// This action does not clear the buffer, to do that you should also call `.clear`.
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
    /// The state of the component is updated according to these rules:
    ///
    /// `KeyEvent::Backspace`
    ///
    ///     Delete one character behind the current position, and move the cursor back.
    ///
    /// `KeyEvent::Char`
    ///
    ///     Insert the character the current position, and move the cursor forward.
    ///
    /// `KeyEvent::Left`
    ///
    ///     Move the cursor to the left, if possible.
    ///
    /// `KeyEvent::Right`
    ///
    ///     Move the cursor to the right, if possible.
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
