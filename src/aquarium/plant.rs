pub struct Plant {
    x: isize,
    y: isize,
    frame: usize,      // Tracks the current animation frame
    direction: isize,  // Tracks the direction of the animation (1 for forward, -1 for backward)
}

impl Plant {
    // Constructor for the Plant struct
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y, frame: 0, direction: 1 }
    }

    // Update the y-coordinate of the plant
    pub fn update_y(&mut self, y: isize) {
        self.y = y;
    }

    // Render the plant with a swaying animation
    pub fn render(&mut self) {
        // Define multiple plant states for the swaying animation
        let plant_states = [
            [
                "(",
                " )",
                "( ",
                "_)",
            ],
            [
                " )",
                "(",
                " )",
                "(_",
            ],
            [
                "  )",
                " (",
                " )",
                "(_",
            ],
            [
                "   )",
                "  (",
                " )",
                "(_",
            ],
        ];

        let color_code = "\x1B[32m"; // ANSI code for green text
        let reset_code = "\x1B[0m"; // ANSI code to reset text formatting

        // Select the current plant state based on the frame
        let plant = &plant_states[self.frame];

        // Render the selected plant state with color
        for (i, line) in plant.iter().enumerate() {
            let plant_y = self.y + i as isize;
            print!(
                "\x1B[{};{}H{}{}{}",
                plant_y + 1,
                self.x + 1,
                color_code, // Apply the color
                line,
                reset_code  // Reset the color after the line
            );
        }

        // Update the frame for the next render call
        self.frame = (self.frame as isize + self.direction) as usize;

        // Reverse direction if the animation reaches the start or end
        if self.frame == 0 || self.frame == plant_states.len() - 1 {
            self.direction *= -1;
        }
    }
}

