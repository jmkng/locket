pub use self::message::{batch, quit, BatchMessage, Message, QuitMessage};

mod message;

/// Boxed function or closure used to perform an action,
/// and optionally carry a message.
///
/// Returned by implementors of `Model` to communicate with other Locket components.
pub type Command = Box<dyn FnOnce() -> Option<Message> + Send + 'static>;
