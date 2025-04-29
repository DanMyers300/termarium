pub struct Fish {
  x: isize,
  y: isize,
  dx: isize,
  dy: isize,
}

impl Fish {
  pub fn new(
    x: isize,
    y: isize,
    dx: isize,
    dy: isize
  ) -> Self {
    Self { x, y, dx, dy }
  }

  pub fn render(
    &mut self,
    term_width: isize,
    term_height: isize
  ) {
    self.x = self.x.clamp(0, term_width - 5);
    self.y = self.y.clamp(0, term_height - 2);

    let fish = [
      "  _",
      "><_>",
    ];

    let color_code = "\x1B[34m";
    let reset_code = "\x1B[0m";

    for (i, line) in fish.iter().enumerate() {
      let fish_y = self.y + i as isize;
      if fish_y >= 0 && fish_y < term_height {
        print!(
          "\x1B[{};{}H{}{}{}",
          fish_y + 1,
          self.x + 1,
          color_code,
          line,
          reset_code
        );
      }
    }

    let next_y = self.y + self.dy;
    let next_x = self.x + self.dx;

    if next_y <= 0 || next_y >= term_height - 2 {
      self.dy = -self.dy;
    } else {
      self.y = next_y;
    }

    if next_x <= 0 || next_x >= term_width - 5 {
      self.dx = -self.dx;
    } else {
      self.x = next_x;
    }
  }
}
