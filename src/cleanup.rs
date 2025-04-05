use std::io::{self, Write};

pub struct CleanupCursor;

impl Drop for CleanupCursor {
    fn drop(&mut self) {
        print!("\x1B[?25h");
        io::stdout().flush().unwrap();
    }
}

