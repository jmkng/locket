/// Execute a block expression and immediately flush stdout.
#[macro_export]
macro_rules! flush {
    ($($block:expr);*) => {{
        use std::io::Write;

        $(
            let result = $block;
        )*

        if let Err(error) = std::io::stdout().flush() {
            Err(error)
        } else {
            Ok(())
        }
    }};
}

/// Clear the terminal.
#[macro_export]
macro_rules! clear {
    () => {
        print!("\x1B[2J")
    };
}
