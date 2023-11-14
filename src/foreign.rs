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
