use std::{
    io::{self, Write},
    thread, time,
};
use termion::terminal_size;
use ctrlc;

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

struct Fish {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Fish {
    fn new(x: isize, y: isize, dx: isize, dy: isize) -> Self {
        Self { x, y, dx, dy }
    }

    fn render(&mut self, terminal_width: isize, terminal_height: isize) {
        self.x = self.x.clamp(0, terminal_width - 5);
        self.y = self.y.clamp(0, terminal_height - 2);

        let fish = vec![
            "  _",
            "><_>",
        ];

        for (i, line) in fish.iter().enumerate() {
            let fish_y = self.y + i as isize;
            if fish_y >= 0 && fish_y < terminal_height {
                print!("\x1B[{};{}H{}", fish_y + 1, self.x + 1, line);
            }
        }

        let next_y = self.y + self.dy;
        let next_x = self.x + self.dx;

        if next_y <= 0 || next_y >= terminal_height - 2 {
            self.dy = -self.dy;
        } else {
            self.y = next_y;
        }

        if next_x <= 0 || next_x >= terminal_width - 5 {
            self.dx = -self.dx;
        } else {
            self.x = next_x;
        }
    }
}

struct CleanupCursor;

impl Drop for CleanupCursor {
    fn drop(&mut self) {
        print!("\x1B[?25h");
        io::stdout().flush().unwrap();
    }
}

