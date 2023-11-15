pub use self::command::{batch, quit, BatchMessage, Command, Message, QuitMessage};

mod command;

/// Defines an executable Locket model.
pub trait Model {
    /// Called a single time when the model is first executed.
    fn init(&self) -> Option<Command> {
        None
    }

    /// Called every time the application receives a `Message`.
    fn update(&mut self, msg: Message) -> Option<Command>;

    /// Returns the interfaceCalled after `update`
    fn view(&self) -> String;
}
