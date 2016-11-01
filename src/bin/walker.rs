extern crate rand;

use rand::Rng;

struct Walker {
    x: f32,
    y: f32
}

impl Walker {
    fn render(&self) {
        println!("x,y: {},{}", self.x, self.y);
    }

    fn update(&mut self) {
        let mut dx = rand::random::<f32>();
        let mut dy = rand::random::<f32>();

        dx = dx * 10.0 - 5.0;
        dy = dy * 10.0 - 5.0;

        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let width = 800;
    let height = 600;

    let mut walker = Walker {
        x: (width as f32) / 2.0,
        y: (height as f32) / 2.0,
    };

    loop {
        walker.update();
        walker.render();
    }
}
