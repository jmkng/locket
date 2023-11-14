use crate::message::Message;

/// A boxed function or closure that performs computations and optionally dispatches messages.
/// All commands are processed in their own threads, so blocking commands are totally fine.
/// Frequently, data needs to be passed to commands. Since commands take no arguments,
/// a common solution to this is to build constructor functions.
///
/// # Example
///
/// ```
/// // a constructor function
/// fn make_request_command(url: &str) -> Command {
///     // it's okay to block since commands are multi threaded
///     let text_response = reqwest::blocking::get(url).unwrap().text().unwrap();
///     
///     // the command itself
///     Box::new(move || Some(Box::new(HttpResponse(text_response))))
/// }
pub type Command = Box<dyn FnOnce() -> Option<Message> + Send + 'static>;

pub(crate) struct QuitMessage;

/// A built in command that quits the application.
pub fn quit() -> Option<Message> {
    Some(Box::new(QuitMessage))
}

pub(crate) struct BatchMessage(pub Vec<Command>);

/// A built in command that combines multiple commands together.
///
/// These commands are executed in parallel, just like normal.
pub fn batch(cmds: Vec<Command>) -> Command {
    Box::new(|| Some(Box::new(BatchMessage(cmds))))
}
