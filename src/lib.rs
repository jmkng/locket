pub use crossterm;
pub use model::{batch, exit};
pub use model::{Command, Message, Model};
pub use utility::Pager;

pub mod components;
pub mod event;
pub mod font;

mod cursor;
mod error;
mod foreign;
mod model;
mod screen;
mod utility;

/// Enable mouse capture events.
///
/// Necessary if your application involves tracking or interacting with the cursor.
#[macro_export]
macro_rules! with_mouse_capture {
    () => {
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
    };
}

/// Exit the application when a message containing a `ctrl-c` key event
/// is received.
#[macro_export]
macro_rules! with_exit {
    ($event:expr) => {
        if let crossterm::event::KeyModifiers::CONTROL = $event.modifiers {
            match $event.code {
                KeyCode::Char('c') => return Some(Box::new(locket::exit)),
                _ => return None,
            }
        }
    };
}

/// Execute a model.
pub fn execute(model: impl Model) -> std::io::Result<()> {
    let mut model = model;
    let mut stdout = std::io::stdout();

    let (message_tx, message_rx) = std::sync::mpsc::channel::<model::Message>();
    let message_tx_2 = message_tx.clone();

    let (command_tx, command_rx) = std::sync::mpsc::channel::<model::Command>();
    let command_tx_2 = command_tx.clone();

    std::thread::spawn(move || loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => message_tx.send(Box::new(event)).unwrap(),
            crossterm::event::Event::Mouse(event) => message_tx.send(Box::new(event)).unwrap(),
            crossterm:: event::Event::Resize(x, y) => {
                message_tx.send(Box::new(event::ResizeEvent(x, y))).unwrap()
            }
            
            _ => {}
            // crossterm::event::Event::FocusGained => todo!(),
            // crossterm::event::Event::FocusLost => todo!(),
            // crossterm::event::Event::Paste(_) => todo!(),
        }
    });

    std::thread::spawn(move || loop {
        let cmd = match command_rx.recv() {
            Ok(cmd) => cmd,
            Err(_) => return,
        };

        let msg_tx2 = message_tx_2.clone();
        std::thread::spawn(move || {
            if let Some(msg) = cmd() {
                msg_tx2.send(msg).unwrap();
            }
        });
    });

    initialize(&mut stdout, &model, command_tx_2)?;
    let mut prev = utility::normalize_endings(model.view());
    crossterm::execute!(stdout, crossterm::style::Print(&prev))?;

    loop {
        let message = message_rx.recv().unwrap();
        if message.is::<model::ExitMessage>() {
            break;
        } else if message.is::<model::BatchMessage>() {
            let batch = message.downcast::<model::BatchMessage>().unwrap();
            for cmd in batch.0 {
                command_tx.send(cmd).unwrap();
            }
        } else if let Some(cmd) = model.update(&message) {
            command_tx.send(cmd).unwrap();
        }

        let curr = utility::normalize_endings(model.view());
        clear_lines(&mut stdout, prev.matches("\r\n").count())?;
        crossterm::execute!(stdout, crossterm::style::Print(&curr))?;
        prev = curr;
    }

    deinitialize(&mut stdout)
}

fn initialize(
    stdout: &mut std::io::Stdout,
    model: &impl Model,
    cmd_tx: std::sync::mpsc::Sender<model::Command>,
) -> std::io::Result<()> {
    if let Some(cmd) = model.init() {
        cmd_tx.send(cmd).unwrap();
    }

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, crossterm::cursor::Hide)
}

fn deinitialize(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    crossterm::execute!(stdout, crossterm::cursor::Show)?;
    crossterm::execute!(stdout, crossterm::event::DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()
}

fn clear_lines(stdout: &mut std::io::Stdout, count: usize) -> std::io::Result<()> {
    for _ in 0..count {
        crossterm::execute!(
            stdout,
            crossterm::cursor::MoveToPreviousLine(1),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
        )?;
    }

    Ok(())
}