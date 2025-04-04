use std::{
    io::{self, Write},
    thread, time,
};
use termion::terminal_size;

fn main() {
    // Hide the cursor
    print!("\x1B[?25l");
    io::stdout().flush().unwrap(); // Flush to ensure the cursor is hidden

    // Ensure the cursor is shown again when the program exits
    let _cleanup = CleanupCursor;

    let mut fish1 = Fish::new(10, 5, 1, 1); // Starting at (10, 5), moving right and down
    let mut fish2 = Fish::new(20, 10, -1, 1); // Starting at (20, 10), moving left and down

    loop {
        // Dynamically get the terminal size
        let (width, height) = terminal_size().unwrap();
        let terminal_width = width as isize;
        let terminal_height = height as isize;

        // Clear the screen and render the aquarium
        print!("\x1B[2J\x1B[H");

        // Render the fish
        fish1.render(terminal_width, terminal_height);
        fish2.render(terminal_width, terminal_height);

        // Flush the output to ensure it renders immediately
        io::stdout().flush().unwrap();

        // Delay for smooth animation
        thread::sleep(time::Duration::from_millis(100));
    }
}

struct Fish {
    x: isize,  // Horizontal position
    y: isize,  // Vertical position
    dx: isize, // Horizontal direction (1 = right, -1 = left)
    dy: isize, // Vertical direction (1 = down, -1 = up)
}

impl Fish {
    fn new(x: isize, y: isize, dx: isize, dy: isize) -> Self {
        Self { x, y, dx, dy }
    }

    fn render(&mut self, terminal_width: isize, terminal_height: isize) {
        self.x = self.x.clamp(0, terminal_width - 5); // Fish width is 5
        self.y = self.y.clamp(0, terminal_height - 2); // Fish height is 2

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

        // Update position
        let next_y = self.y + self.dy;
        let next_x = self.x + self.dx;

        // Check for vertical boundaries and reverse direction if needed
        if next_y <= 0 || next_y >= terminal_height - 2 {
            self.dy = -self.dy; // Reverse vertical direction
        } else {
            self.y = next_y; // Update position only if within bounds
        }

        // Check for horizontal boundaries and reverse direction if needed
        if next_x <= 0 || next_x >= terminal_width - 5 {
            self.dx = -self.dx; // Reverse horizontal direction
        } else {
            self.x = next_x; // Update position only if within bounds
        }
    }
}

// Struct to ensure the cursor is shown again when the program exits
struct CleanupCursor;

impl Drop for CleanupCursor {
    fn drop(&mut self) {
        // Show the cursor
        print!("\x1B[?25h");
        io::stdout().flush().unwrap();
    }
}

