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
        ["∘˙○˚.•",
         " .•"]
    } else {
        ["˚○", "｡"]
    };

    for (i, line) in bubble.iter().enumerate() {
      let bubble_y = self.y + i as isize;
      if bubble_y >= 0 && bubble_y < term_height {
        print!(
          "\x1B[{};{}H{}",
          bubble_y + 1,
          self.x + 1,
          line
        );
      }
    }

    // Update the bubble's vertical position
    let next_y = self.y + self.dy;

    // Check if the bubble hits the top boundary
    if next_y <= 0 {
      self.is_active = false;
    } else {
      self.y = next_y; // Update position if within bounds
    }
  }
}

