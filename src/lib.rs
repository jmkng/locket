pub use crossterm;

pub use model::{batch, exit, Command, Message, Model};
pub mod components;
pub mod event;
pub mod font;

mod cursor;
mod error;
mod foreign;
mod model;
mod screen;

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

    let (msg_tx, msg_rx) = std::sync::mpsc::channel::<model::Message>();
    let msg_tx2 = msg_tx.clone();

    let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<model::Command>();
    let cmd_tx2 = cmd_tx.clone();

    std::thread::spawn(move || loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => msg_tx.send(Box::new(event)).unwrap(),
            crossterm::event::Event::Mouse(event) => msg_tx.send(Box::new(event)).unwrap(),
            crossterm:: event::Event::Resize(x, y) => {
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
        if msg.is::<model::ExitMessage>() {
            break;
        } else if msg.is::<model::BatchMessage>() {
            let batch = msg.downcast::<model::BatchMessage>().unwrap();
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
    cmd_tx: std::sync::mpsc::Sender<model::Command>,
) -> std::io::Result<()> {
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

fn deinitialize(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    crossterm::execute!(stdout, crossterm::cursor::Show)?;
    crossterm::execute!(stdout, crossterm::event::DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()
}
