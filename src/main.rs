use std::{thread, time};

fn main() {
    loop {
        // ANSI escape code to clear the screen
        print!("\x1B[2J\x1B[H");

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

