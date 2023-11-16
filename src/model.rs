pub use self::command::{batch, exit, BatchMessage, Command, ExitMessage, Message};

mod command;

/// Defines an executable Locket model.
pub trait Model {
    /// Called a single time when the model is first executed.
    fn init(&self) -> Option<Command> {
        None
    }

    /// Called every time the application receives a `Message`.
    fn update(&mut self, message: &Message) -> Option<Command>;

    /// Returns the interfaceCalled after `update`
    fn view(&self) -> String;
}
