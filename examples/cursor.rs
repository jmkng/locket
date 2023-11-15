use std::{io::Write, thread::sleep, time::Duration};

use locket::{flush, move_down, move_right, move_up};

/// Use Locket macros to manipulate the cursor and fix a typo.
fn main() {
    println!("Line 1");
    println!("Line 3");
    println!("Line 3");

    sleep(Duration::from_secs(1));

    for _ in 0..2 {
        locket::flush!({
            move_up!();
        })
        .unwrap();

        sleep(Duration::from_millis(500));
    }
    flush!({
        move_right!(5);
        print!("2")
    })
    .unwrap();

    sleep(Duration::from_secs(2));

    flush!({
        move_down!();
        print!("\n");
    })
    .unwrap()
}
