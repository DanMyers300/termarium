use std::{thread, time};

fn main() {
    let mut x = 0; // Horizontal position
    let mut y = 0; // Vertical position
    let mut dx = 1; // Horizontal direction (1 = right, -1 = left)
    let mut dy = 1; // Vertical direction (1 = down, -1 = up)

    let terminal_width = 60;
    let terminal_height = 30;

    loop {
        // ANSI escape code to clear the screen
        print!("\x1B[2J");

        // ANSI escape code to move the cursor to (y, x)
        print!("\x1B[{};{}H", y + 1, x + 1);

        // Define the fish
        let fish = r#"
              _
            ><_>
        "#;

        // Print the fish
        println!("{}", fish);

        // Update position
        x += dx;
        y += dy;

        // Check for horizontal boundaries and reverse direction if needed
        if x <= 0 || x >= terminal_width - 6 { // Adjust for fish width
            dx = -dx; // Reverse horizontal direction
        }

        // Check for vertical boundaries and reverse direction if needed
        if y <= 0 || y >= terminal_height - 3 { // Adjust for fish height
            dy = -dy; // Reverse vertical direction
        }

        // Delay for smooth animation
        let delay = time::Duration::from_millis(300);
        thread::sleep(delay);
    }
}

