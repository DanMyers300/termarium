mod fish;
use fish::Fish;

mod bubble;
use bubble::Bubble;

pub struct Aquarium {
    fish1: Fish,
    fish2: Fish,
    bubble1: Bubble,
}

impl Aquarium {
    pub fn new() -> Self {
        Self {
            fish1: Fish::new(10, 5, 1, 1),
            fish2: Fish::new(20, 10, -1, 1),
            bubble1: Bubble::new(200, 200, -1),
        }
    }

    pub fn render(&mut self, term_width: isize, term_height: isize) {
        self.fish1.render(term_width, term_height);
        self.fish2.render(term_width, term_height);
        self.bubble1.render(term_height);
    }
}
