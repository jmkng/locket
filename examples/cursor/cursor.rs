use locket::{flush, move_down, move_right, move_up};

/// Use Locket macros to manipulate the cursor and fix a typo.
fn main() {
    println!("Line 1");
    println!("Line 3"); // <- Typo here!
    println!("Line 3");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Move the cursor around to fix it.
    for _ in 0..2 {
        locket::flush!({
            move_up!();
        })
        .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    flush!({
        move_right!(5);
        print!("2")
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    flush!({
        move_down!();
        print!("\n");
    })
    .unwrap()
}
