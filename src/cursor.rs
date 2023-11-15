/// Execute a block expression and immediately flush stdout.
#[macro_export]
macro_rules! flush {
    ($block:expr) => {{
        let result = $block;

        if let Err(error) = std::io::stdout().flush() {
            Err(error)
        } else {
            Ok(result)
        }
    }};
    () => {
        if let Err(error) = std::io::stdout().flush() {
            Err(error)
        } else {
            Ok(())
        }
    };
}

/// Move the cursor.
///
/// You may prefer one of the "move_*" macros instead.
///
/// Directions are represented by "A", "B", "C", or "D" as described
/// by ANSI/VT100 Terminal Control Escape Sequences:
///
/// https://espterm.github.io/docs/VT100%20escape%20codes.html
#[macro_export]
macro_rules! move_cursor {
    ($direction:literal) => {
        print!("\x1B[{}", $direction);
    };
    ($direction:literal, $num:expr) => {
        print!("\x1B[{}{}", $num, $direction);
    };
}

/// Move the cursor up.
///
/// If the number of steps is not provided, the default of 1 is used.
#[macro_export]
macro_rules! move_up {
    () => {
        locket::move_cursor!("A");
    };
    ($num:expr) => {
        locket::move_cursor!("A", $num);
    };
}

/// Move the cursor down.
///
/// If the number of steps is not provided, the default of 1 is used.
#[macro_export]
macro_rules! move_down {
    () => {
        locket::move_cursor!("B");
    };
    ($num:expr) => {
        locket::move_cursor!("B", $num);
    };
}

/// Move the cursor right.
///
/// If the number of steps is not provided, the default of 1 is used.
#[macro_export]
macro_rules! move_right {
    () => {
        locket::move_cursor!("C");
    };
    ($num:expr) => {
        locket::move_cursor!("C", $num);
    };
}

/// Move the cursor left.
///
/// If the number of steps is not provided, the default of 1 is used.
#[macro_export]
macro_rules! move_left {
    () => {
        locket::move_cursor!("D");
    };
    ($num:expr) => {
        locket::move_cursor!("D", $num);
    };
}
