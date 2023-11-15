use crate::error::Error;

/// Return the (row, column) size of the terminal.
pub unsafe fn get_ui_size() -> Result<(u16, u16), Error> {
    // <sys/ioctl.h>
    let mut size: libc::winsize = std::mem::zeroed();

    if libc::ioctl(
        libc::STDOUT_FILENO,
        libc::TIOCGWINSZ.into(),
        &mut size as *mut _,
    ) != 0
    {
        return Err(Error::FFI("unable to determine terminal size".into()));
    }

    Ok((size.ws_row as u16, size.ws_col as u16))
}

/// Convert a C return code to a result.
fn c_err(value: libc::c_int, reason: &'static str) -> Result<(), Error> {
    if value != 0 {
        return Err(Error::FFI(reason));
    }

    Ok(())
}

unsafe extern "C" fn handle_signal(
    signum: i32,
    _info: *mut libc::siginfo_t,
    _context: *mut libc::c_void,
) {
    match signum {
        libc::SIGWINCH => {
            println!("Received SIGWINCH - Window Size Change");
        }
        _ => {}
    }
}
