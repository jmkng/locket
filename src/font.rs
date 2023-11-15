/// Print text with a bold sequence.
#[macro_export]
macro_rules! bold {
    ($text:expr) => {
        format!("\x1B[1m{}\x1B[0m", $text)
    };
}

/// Print text with a faint sequence.
#[macro_export]
macro_rules! faint {
    ($text:expr) => {
        format!("\x1B[2m{}\x1B[0m", $text)
    };
}

/// Print text with a italic sequence.
#[macro_export]
macro_rules! italic {
    ($text:expr) => {
        format!("\x1B[3m{}\x1B[0m", $text)
    };
}

/// Print text with a underline sequence.
#[macro_export]
macro_rules! underline {
    ($text:expr) => {
        format!("\x1B[4m{}\x1B[0m", $text)
    };
}

/// Print text with a inverse sequence.
#[macro_export]
macro_rules! inverse {
    ($text:expr) => {
        format!("\x1B[7m{}\x1B[0m", $text)
    };
}

/// Print text with a strikethrough sequence.
#[macro_export]
macro_rules! strikethrough {
    ($text:expr) => {
        format!("\x1B[9m{}\x1B[0m", $text)
    };
}
