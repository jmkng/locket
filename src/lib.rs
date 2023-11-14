use std::io::Result;

pub use crossterm;
pub use message::Message;
pub use model::Model;
pub use program::enable_mouse_capture;

use command::Command;

pub mod command;
pub mod components;
pub mod event;

mod foreign;
mod message;
mod model;
mod program;

/// Execute a model.
pub fn execute(model: impl Model) -> Result<()> {
    let mut model = model;
    let mut stdout = std::io::stdout();

    let (msg_tx, msg_rx) = std::sync::mpsc::channel::<Message>();
    let msg_tx2 = msg_tx.clone();

    let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<Command>();
    let cmd_tx2 = cmd_tx.clone();

    std::thread::spawn(move || loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => msg_tx.send(Box::new(event)).unwrap(),
            crossterm::event::Event::Mouse(event) => msg_tx.send(Box::new(event)).unwrap(),
            crossterm::event::Event::Resize(x, y) => {
                msg_tx.send(Box::new(event::ResizeEvent(x, y))).unwrap()
            }
            _ => {}
            // crossterm::event::Event::FocusGained => todo!(),
            // crossterm::event::Event::FocusLost => todo!(),
            // crossterm::event::Event::Paste(_) => todo!(),
        }
    });

    std::thread::spawn(move || loop {
        let cmd = match cmd_rx.recv() {
            Ok(cmd) => cmd,
            Err(_) => return,
        };

        let msg_tx2 = msg_tx2.clone();
        std::thread::spawn(move || {
            if let Some(msg) = cmd() {
                msg_tx2.send(msg).unwrap();
            }
        });
    });

    initialize(&mut stdout, &model, cmd_tx2)?;
    let mut prev = normalized_view(&model);
    crossterm::execute!(stdout, crossterm::style::Print(&prev))?;

    loop {
        let msg = msg_rx.recv().unwrap();
        if msg.is::<command::QuitMessage>() {
            break;
        } else if msg.is::<command::BatchMessage>() {
            let batch = msg.downcast::<command::BatchMessage>().unwrap();
            for cmd in batch.0 {
                cmd_tx.send(cmd).unwrap();
            }
        } else if let Some(cmd) = model.update(msg) {
            cmd_tx.send(cmd).unwrap();
        }

        let curr = normalized_view(&model);
        clear_lines(&mut stdout, prev.matches("\r\n").count())?;
        crossterm::execute!(stdout, crossterm::style::Print(&curr))?;
        prev = curr;
    }

    deinitialize(&mut stdout)
}

fn initialize(
    stdout: &mut std::io::Stdout,
    model: &impl Model,
    cmd_tx: std::sync::mpsc::Sender<Command>,
) -> Result<()> {
    if let Some(cmd) = model.init() {
        cmd_tx.send(cmd).unwrap();
    }

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, crossterm::cursor::Hide)
}

fn normalized_view(model: &impl Model) -> String {
    let view = model.view();
    let view = if !view.ends_with('\n') {
        view + "\n"
    } else {
        view
    };
    view.replace('\n', "\r\n")
}

fn clear_lines(stdout: &mut std::io::Stdout, count: usize) -> Result<()> {
    for _ in 0..count {
        crossterm::execute!(
            stdout,
            crossterm::cursor::MoveToPreviousLine(1),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
        )?;
    }

    Ok(())
}

fn deinitialize(stdout: &mut std::io::Stdout) -> Result<()> {
    crossterm::execute!(stdout, crossterm::cursor::Show)?;
    crossterm::execute!(stdout, crossterm::event::DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()
}

#[cfg(test)]
mod tests {
    // use cate::{
    //     model::{Info, Update},
    //     Model, Program,
    // };

    // #[test]
    // pub fn test_main() {
    //     let mut p = Program::new(Box::new(Bonk(10)));
    //     p.run();
    // }

    // struct Bonk(i32);

    // enum MessageKind {
    //     Exit,
    //     Test,
    // }

    // impl Info for MessageKind {
    //     fn exit(&self) -> bool {
    //         match self {
    //             MessageKind::Exit => true,
    //             MessageKind::Test => false,
    //         }
    //     }

    //     fn error(&self) -> bool {
    //         match self {
    //             MessageKind::Exit => false,
    //             MessageKind::Test => false,
    //         }
    //     }
    // }

    // impl Model<MessageKind> for Bonk {
    //     fn init(&self) -> Update<MessageKind> {
    //         Update {
    //             model: Some(Box::new(Bonk(50))),
    //             command: Some(|| MessageKind::Test),
    //         }
    //     }

    //     fn update(&self, message: MessageKind) -> Update<MessageKind> {
    //         match message {
    //             MessageKind::Exit => println!("received exit message (omg)"),
    //             MessageKind::Test => println!("received test message!"),
    //         }

    //         Update {
    //             model: None,
    //             command: None,
    //         }
    //     }

    //     fn view(&self) -> String {
    //         format!("my number is: {}\n", self.0)
    //     }
    // }
}
