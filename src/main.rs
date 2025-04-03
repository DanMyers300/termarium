use std::{thread, time};

fn main() {
    // ANSI escape code to clear the screen and reset the cursor position
    loop {
        // Clear the screen and reset the cursor position
        print!("\x1B[2J\x1B[H");

        // Draw the fish
        let fish = format!(
          "
              _
            ><_>
          "
        );

        println!("{}", fish);

        // Add a delay to make the animation smooth
        let delay = time::Duration::from_millis(500); // 500 milliseconds
        thread::sleep(delay);
    }
}

