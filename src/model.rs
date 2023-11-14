use crate::{command::Command, message::Message};

/// The trait your model must implement in order to be `run`.
///
/// `init` is called once when the model is run for the first time, and optionally returns a `Command`.
/// There is a default implementation of `init` that returns `None`.
///
/// `update` is called every time your application recieves a `Message`.
/// You are allowed to mutate your model's state in this function.
/// It optionally returns a `Command`.
///
/// `view` is called after every `update` and is responsible for rendering the model.
/// It returns a `String` that is printed to the screen.
/// You are _not_ allowed to mutate the state of your application in the view, only render it.
///
/// For examples, check the `examples` directory.
pub trait Model {
    fn init(&self) -> Option<Command> {
        None
    }

    fn update(&mut self, msg: Message) -> Option<Command>;

    fn view(&self) -> String;
}

// /// Implemented by component types that want to be rendered in
// /// a Locket program.
// pub trait Model<T: Info> {
//     /// Invoked one time when the model loads.
//     fn init(&self) -> Update<T>;

//     /// Return the user interface of the model as a string.
//     fn view(&self) -> String;

//     /// Invoked for every message that the model receives.
//     ///
//     /// May return a new, updated model and optional command.
//     fn update(&self, message: T) -> Update<T>;
// }

// /// A response value returned by a Model implementor.
// ///
// /// May contain a new model to display, or a command to dispatch.
// pub struct Update<T> {
//     pub model: Option<Box<dyn Model<T>>>,
//     pub command: Option<Command<T>>,
// }

// /// Provide some facilities for Locket to introspect messages.
// pub trait Info {
//     /// Return true if this message should cause the program to exit.
//     fn exit(&self) -> bool;

//     /// Return true if this message should be treated as an error.
//     fn error(&self) -> bool;
// }

// pub type Message<T> = T;

// /// A function used to perform some action.
// pub type Command<T> = fn() -> Message<T>;
