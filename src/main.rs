use std::{
    io::{self, Write},
    thread, time,
};
use termion::terminal_size;

mod aquarium;
use aquarium::Aquarium;

mod cleanup;
use cleanup::CleanupCursor;

fn main() {
    // Remove cursor from screen
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();

    // Show cursor on end
    let _cleanup = CleanupCursor;

    // Handle ctrl+c
    ctrlc::set_handler(move || {
        print!("\x1B[?25h");
        io::stdout().flush().unwrap();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    // Create the aquarium
    let mut aquarium = Aquarium::new();

    loop {
        let (width, height) = terminal_size().unwrap();
        let term_width = width as isize;
        let term_height = height as isize;

        // Clear screen & move cursor
        print!("\x1B[2J\x1B[H");

        // Render the aquarium
        aquarium.render(term_width, term_height);

        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}
