mod fish;
use fish::Fish;

pub struct Aquarium {
    fish1: Fish,
    fish2: Fish,
}

impl Aquarium {
    pub fn new() -> Self {
        Self {
            fish1: Fish::new(10, 5, 1, 1),
            fish2: Fish::new(20, 10, -1, 1),
        }
    }

    pub fn render(&mut self, term_width: isize, term_height: isize) {
        self.fish1.render(term_width, term_height);
        self.fish2.render(term_width, term_height);
    }
}
