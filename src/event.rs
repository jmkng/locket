/// Event representing a terminal resize (x, y).
/// Boxed as a message so it can be sent to the application.
pub struct ResizeEvent(pub u16, pub u16);
