pub struct Bubble {
  x: isize,
  y: isize,
  dy: isize,
  is_active: bool,
  large_bubble: bool,
}

impl Bubble {
  pub fn new(x: isize, y: isize, dy: isize, large_bubble: bool) -> Self {
    Self {
      x,
      y,
      dy,
      is_active: true,
      large_bubble,
    }
  }

  pub fn render(&mut self, term_height: isize) {
    if !self.is_active {
      return;
    }

    // Clamp the bubble's vertical position to stay within bounds
    self.y = self.y.clamp(0, term_height - 2);

    let bubble = if self.large_bubble {
        ["∘˙○˚.•"]
    } else {
        ["｡˚○"]
    };

    // Render the bubble at its current position
    for (i, line) in bubble.iter().enumerate() {
      let bubble_y = self.y + i as isize;
      if bubble_y >= 0 && bubble_y < term_height {
        // Properly position the cursor using ANSI escape codes
        print!(
          "\x1B[{};{}H{}", // Print the bubble
          bubble_y + 1,  // Vertical position (1-based index for terminal)
          self.x + 1,    // Horizontal position (1-based index for terminal)
          line       // Bubble content
        );
      }
    }

    // Update the bubble's vertical position
    let next_y = self.y + self.dy;

    // Check if the bubble hits the top boundary
    if next_y <= 0 {
      self.is_active = false;
    } else if next_y >= term_height - 2 {
      self.dy = -self.dy; // Reverse direction if it hits the bottom boundary
    } else {
      self.y = next_y; // Update position if within bounds
    }
  }
}

