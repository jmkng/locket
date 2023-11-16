/// `Black` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const BLACK: u8 = 0;

/// `Maroon` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const MAROON: u8 = 1;

/// `Green` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const GREEN: u8 = 2;

/// `Olive` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const OLIVE: u8 = 3;

/// `Navy` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const NAVY: u8 = 4;

/// `Purple` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const PURPLE: u8 = 5;

/// `Teal` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const TEAL: u8 = 6;

/// `Silver` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const SILVER: u8 = 7;

/// `Gray` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const GRAY: u8 = 8;

/// `Red` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const RED: u8 = 9;

/// `Lime` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const LIME: u8 = 10;

/// `Yellow` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const YELLOW: u8 = 11;

/// `Blue` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const BLUE: u8 = 12;

/// `Fuchsia` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const FUCHSIA: u8 = 13;

/// `Aqua` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const AQUA: u8 = 14;

/// `White` XTERM 256.
///
/// More colors are available, but not included as a const in Locket.
///
/// https://en.wikipedia.org/wiki/Xterm
pub const WHITE: u8 = 15;

/// Annotate a string with styles.
///
/// # Examples
///
/// Create a bold, underlined maroon greeting:
///
/// ```
/// locket::font::FontBuilder::new("Hello, World!")
///     .bold()
///     .underline()
///     .fill(locket::font::MAROON)
///     .to_string();
/// ```
pub struct FontBuilder {
    buffer: String,
}

impl FontBuilder {
    /// Return a new instance of FontBuilder.
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            buffer: format!("{}{}", text.as_ref(), "\x1B[0m"),
        }
    }

    /// Apply a bold sequence.
    pub fn bold(self) -> Self {
        Self {
            buffer: format!("\x1B[1m{}", self.buffer),
        }
    }

    /// Apply a faint sequence.
    pub fn faint(self) -> Self {
        Self {
            buffer: format!("\x1B[2m{}", self.buffer),
        }
    }

    /// Apply a italic sequence.
    pub fn italic(self) -> Self {
        Self {
            buffer: format!("\x1B[3m{}", self.buffer),
        }
    }

    /// Apply an underline sequence.
    pub fn underline(self) -> Self {
        Self {
            buffer: format!("\x1B[4m{}", self.buffer),
        }
    }

    /// Apply an inverse sequence.
    pub fn inverse(self) -> Self {
        Self {
            buffer: format!("\x1B[7m{}", self.buffer),
        }
    }

    /// Apply a strikethrough sequence.
    pub fn strikethrough(self) -> Self {
        Self {
            buffer: format!("\x1B[9m{}", self.buffer),
        }
    }

    /// Apply a fill color.
    pub fn fill(self, color: u8) -> Self {
        Self {
            buffer: format!("\x1B[38;5;{}m{}", color, self.buffer),
        }
    }

    /// Apply a background color.
    pub fn background(self, color: u8) -> Self {
        Self {
            buffer: format!("\x1B[48;5;{}m{}", color, self.buffer),
        }
    }

    /// Return the styled string.
    ///
    /// This action will consume the builder.
    pub fn to_string(self) -> String {
        self.buffer
    }
}

/// Return text with a fill color.
pub fn fill<T>(text: T, color: u8) -> String
where
    T: AsRef<str>,
{
    format!("\x1B[38;5;{}m{}\x1B[0m", color, text.as_ref())
}

/// Return text with a background color.
pub fn background<T>(text: T, color: u8) -> String
where
    T: AsRef<str>,
{
    format!("\x1B[48;5;{}m{}\x1B[0m", color, text.as_ref())
}

/// Return text with fill and background colors.
pub fn fill_background<T>(text: T, fill_color: u8, background_color: u8) -> String
where
    T: AsRef<str>,
{
    format!(
        "\x1B[38;5;{}m\x1B[48;5;{}m{}\x1B[0m",
        fill_color,
        background_color,
        text.as_ref()
    )
}

/// Return text with a bold sequence.
#[macro_export]
macro_rules! bold {
    ($text:expr) => {
        format!("\x1B[1m{}\x1B[0m", $text)
    };
}

/// Return text with a faint sequence.
#[macro_export]
macro_rules! faint {
    ($text:expr) => {
        format!("\x1B[2m{}\x1B[0m", $text)
    };
}

/// Return text with a italic sequence.
#[macro_export]
macro_rules! italic {
    ($text:expr) => {
        format!("\x1B[3m{}\x1B[0m", $text)
    };
}

/// Return text with an underline sequence.
#[macro_export]
macro_rules! underline {
    ($text:expr) => {
        format!("\x1B[4m{}\x1B[0m", $text)
    };
}

/// Return text with an inverse sequence.
#[macro_export]
macro_rules! inverse {
    ($text:expr) => {
        format!("\x1B[7m{}\x1B[0m", $text)
    };
}

/// Return text with a strikethrough sequence.
#[macro_export]
macro_rules! strikethrough {
    ($text:expr) => {
        format!("\x1B[9m{}\x1B[0m", $text)
    };
}
