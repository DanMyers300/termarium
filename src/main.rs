use std::{thread, time};

fn main() {
    println!("\r\x1b[2J\x1b[?25l");

    loop {
        thread::sleep(time::Duration::from_millis(17));
    }
}
