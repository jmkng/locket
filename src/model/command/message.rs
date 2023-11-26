use super::Command;

/// A command that will exit the Locket application.
pub struct ExitMessage;

/// Exit the application.
pub fn exit() -> Option<Message> {
    Some(Box::new(ExitMessage))
}

/// Any boxed type that may or may not contain data.
///
/// You may use `downcast_ref` to determine the type of the message,
/// and extract any required information.
///
/// # Examples
///
/// // TODO: Remove/rewrite
/// ```no_run
/// struct HttpResponse(String);
///
/// let http_response_message = Box::new(HttpResponse("Hello World".to_string()));
///
/// if let Some(res) = http_response_message.downcast_ref::<HttpResponse>() {
///     model.response = Some(res);
/// }
/// ```
pub type Message = Box<dyn std::any::Any + Send>;

/// A wrapper for `Vec<Command>`, representing a series of commands.
pub struct BatchMessage(pub Vec<Command>);

/// Combine multiple commands.
pub fn batch(cmds: Vec<Command>) -> Command {
    Box::new(|| Some(Box::new(BatchMessage(cmds))))
}
