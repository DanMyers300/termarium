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

        let delay = time::Duration::from_millis(500);
        thread::sleep(delay);
    }
}

