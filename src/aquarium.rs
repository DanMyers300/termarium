use rand::Rng;

mod fish;
use fish::Fish;

mod bubble;
use bubble::Bubble;

mod plant;
use plant::Plant;

pub struct Aquarium {
  fish1: Fish,
  fish2: Fish,
  bubbles: Vec<Bubble>,
  plants: Vec<Plant>
}

impl Aquarium {
  pub fn new() -> Self {
    Self {
      fish1: Fish::new(10, 5, 1, 1),
      fish2: Fish::new(20, 10, -1, 1),
      bubbles: Vec::new(),
      plants: vec![
        Plant::new(20, 0),
        Plant::new(50, 0),
        Plant::new(90, 0),
        Plant::new(110, 0),
        Plant::new(150, 0),
        Plant::new(190, 0),
      ]
    }
  }

  pub fn render(&mut self, term_width: isize, term_height: isize) {

    for plant in &mut self.plants {
      plant.update_y(term_height - 4);
    };

    self.fish1.render(term_width, term_height);
    self.fish2.render(term_width, term_height);

    for bubble in &mut self.bubbles {
      bubble.render(term_height);
    }

    self.bubbles.retain(|bubble| bubble.is_active);
    self.generate_bubbles(term_width, term_height);
    
    for plant in &mut self.plants {
      plant.render();
    }
  }

  fn generate_bubbles(&mut self, term_width: isize, term_height: isize) {
    let mut rng = rand::rng();

    if rng.random_range(0..100) < 15 {
      let x = rng.random_range(0..term_width as i32) as isize;
      let y = term_height - 1;
      if rng.random_range(0..15) < 4 {
        self.bubbles.push(Bubble::new(x, y, -1, false));
      } else {
        self.bubbles.push(Bubble::new(x, y, -1, true));
      }
    }
  }
}

