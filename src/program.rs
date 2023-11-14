use std::io::Result;

use crossterm::execute;

/// Enables mouse capture events on your application.
///
/// This is optional as it can cause a spam of your update method.
pub fn enable_mouse_capture() -> Result<()> {
    execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
}

// use std::io::{Error, Read, StdinLock, StdoutLock, Write};

// use crossbeam::{channel::Sender, select};

// use crate::{
//     model::{Command, Info},
//     Message, Model,
// };

// /// A Locket program.
// ///
// /// The generic parameter is the message type that you intend to
// /// pass around.
// pub struct Program<'a, T> {
//     /// The initial model to be rendered.
//     initial: Box<dyn Model<T>>,

//     /// The last rendered view.
//     last: String,

//     /// Used to read/write to the terminal.
//     rw: ReadWriter<StdinLock<'a>, StdoutLock<'a>>,
// }

// impl<'a, T> Program<'a, T>
// where
//     T: Send + Sync + Info + 'static,
// {
//     /// Create a new instance of Program with an initial model.
//     pub fn new(initial: Box<dyn Model<T>>) -> Self {
//         Self {
//             initial,
//             last: String::new(),
//             rw: ReadWriter::new(),
//         }
//     }

//     /// Run the Locket program.
//     ///
//     /// Displays the initial model and allows the user to take over control.
//     pub fn run(&mut self) {
//         let (msg_tx, msg_rx) = crossbeam::channel::unbounded::<Message<T>>();
//         let (mut cmd_tx, cmd_rx) = crossbeam::channel::unbounded::<Command<T>>();
//         let (mut set_tx, set_rx) = crossbeam::channel::unbounded::<()>(); // Done<-
//         let (err_tx, err_rx) = crossbeam::channel::unbounded::<Error>();

//         let set_rx_1 = set_rx.clone();

//         // Input loop reads user input and sends it to the Locket program as
//         // a `Message` instance.
//         std::thread::spawn(|| async move {
//             loop {
//                 select! {
//                     recv(set_rx_1) -> _ => break,
//                     default => {
//                         // msg, err := terminput.Read(p.rw)
//                         // if err != nil {
//                         //     errs <- err
//                         //     return
//                         // }
//                         // msgs <- msg
//                     }
//                 }
//             }
//         });

//         let set_rx_2 = set_rx.clone();

//         // Command loop will process received `Command` instances.
//         std::thread::spawn(|| async move {
//             loop {
//                 select! {
//                     recv(set_rx_2) -> _ => break,
//                     recv(cmd_rx) -> rx_result => {
//                         match rx_result {
//                             Ok(command) =>  msg_tx.send(command()).expect("unable to send to msg_tx"),
//                             Err(error) => panic!("{:?}", error)
//                         }
//                     }
//                 }
//             }
//         });

//         let update = self.initial.init();
//         if let Some(command) = update.command {
//             cmd_tx.send(command).expect("unable to send to cmd_tx");
//         }

//         if let Some(view) = update.model {
//             let data = view.view();
//             self.rw
//                 .write(data.as_bytes())
//                 .expect("unable to write to program `rw` :(");
//         }

//         // Main loop handles drawing the models, processing messages, etc.
//         loop {
//             select! {
//                 recv(err_rx) -> _ => {
//                     set_tx.send(()).expect("failed to transfer receive error: unable to send done signal on set_tx");
//                     break;
//                 },
//                 recv(msg_rx) -> result => {
//                     if let Ok(message) = result {
//                         self.handle_message(message, &mut set_tx, &mut cmd_tx);
//                     } else {
//                         set_tx.send(()).expect("failed to receive message: unable to send done signal on set_tx");
//                         break;
//                     }
//                 }
//             }
//         }

//         println!("uhhh");
//     }

//     fn handle_message(
//         &mut self,
//         message: T,
//         set_tx: &mut Sender<()>,
//         cmd_tx: &mut Sender<fn() -> T>,
//     ) {
//         if message.exit() {
//             set_tx
//                 .send(())
//                 .expect("unable to send done signal on set_tx");
//             return;
//         } else if message.error() {
//             return; // TODO: Handle error
//         }

//         let update = self.initial.update(message);
//         if let Some(command) = update.command {
//             cmd_tx.send(command).expect("unable to send to cmd_tx");
//         }

//         if let Some(view) = update.model {
//             let data = normalize(&view.view());
//             self.rw
//                 .write(data.as_bytes())
//                 .expect("unable to write to program `rw` :(");
//             self.last = data;
//         }
//     }
// }

// struct ReadWriter<R, W>
// where
//     R: Read,
//     W: Write,
// {
//     reader: R,
//     writer: W,
// }

// impl<R, W> Read for ReadWriter<R, W>
// where
//     R: Read,
//     W: Write,
// {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.reader.read(buf)
//     }
// }

// impl<R, W> Write for ReadWriter<R, W>
// where
//     R: Read,
//     W: Write,
// {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.writer.write(buf)
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         self.writer.flush()
//     }
// }

// impl ReadWriter<StdinLock<'_>, StdoutLock<'_>> {
//     pub fn new() -> Self {
//         ReadWriter {
//             reader: std::io::stdin().lock(),
//             writer: std::io::stdout().lock(),
//         }
//     }
// }

// fn normalize(s: &str) -> String {
//     s.replace("\n", "\r\n")
// }

// fn clear_lines(n: usize) {
//     for _ in 0..n {
//         move_up(1);
//         clear_line();
//     }
// }

// fn clear_line() {
//     print!("\x1B[2K");
//     std::io::stdout().flush().unwrap();
// }

// fn move_up(n: usize) {
//     print!("\x1B[{}F", n);
//     std::io::stdout().flush().unwrap();
// }
