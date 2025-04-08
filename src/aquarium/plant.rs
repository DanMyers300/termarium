pub struct Plant {
  x: isize,
  y: isize,
}

impl Plant {
  pub fn new(x: isize, y: isize) -> Self {
    Self {
      x,
      y
    }
  }

  pub fn render(&mut self) {
    let plant = [
      " )",
      "(",
      " )",
      "(_"
    ];

    for (i, line) in plant.iter().enumerate() {
      let plant_y = self.y + i as isize;
      print!(
        "\x1B[{};{}H{}",
        plant_y + 1,
        self.x + 1,
        line
      );
    }
  }
}
