/// Flush `stdout`.
///
/// You may optionally pass in an expression that will be evaluated
/// before flushing.
#[macro_export]
macro_rules! flush {
    ($($block:expr);*) => {{
        use std::io::Write;

        $(
            let result = $block;
        )*

        std::io::stdout().flush()
    }};
}

/// Clear the terminal.
#[macro_export]
macro_rules! clear {
    () => {
        print!("\x1B[2J")
    };
}
