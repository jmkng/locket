pub use viewport::{BoundMap, Viewport};

mod viewport;

/// Normalize "\n" line endings to "\r\n".
///
/// This is normally handled by termios, but since Locket operates in raw mode
/// we must be sure to translate manually.
pub fn normalize_endings(initial: String) -> String {
    let initial = if !initial.ends_with('\n') {
        initial + "\n"
    } else {
        initial
    };

    initial.replace('\n', "\r\n")
}
