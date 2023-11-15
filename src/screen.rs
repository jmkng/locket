/// Clear the terminal.
#[macro_export]
macro_rules! clear {
    () => {
        print!("\x1B[2J")
    };
}
