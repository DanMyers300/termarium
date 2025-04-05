use std::{
    io::{self, Write},
    thread, time,
};
use termion::terminal_size;

mod fish;
use fish::Fish;

fn main() {
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();

    let _cleanup = CleanupCursor;

    ctrlc::set_handler(move || {
        print!("\x1B[?25h");
        io::stdout().flush().unwrap();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    let mut fish1 = Fish::new(10, 5, 1, 1);
    let mut fish2 = Fish::new(20, 10, -1, 1);

    loop {
        let (width, height) = terminal_size().unwrap();
        let terminal_width = width as isize;
        let terminal_height = height as isize;

        print!("\x1B[2J\x1B[H");

        fish1.render(terminal_width, terminal_height);
        fish2.render(terminal_width, terminal_height);

        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}

struct CleanupCursor;

impl Drop for CleanupCursor {
    fn drop(&mut self) {
        print!("\x1B[?25h");
        io::stdout().flush().unwrap();
    }
}

