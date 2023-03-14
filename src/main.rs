use rand::Rng;

fn main() {
    let height = 2.;
    let width = 3.;

    let mut square_counter = 0;
    let mut circle_counter = 0;

    let mut rng = rand::thread_rng();

    for _ in 0..100000000 {
        let point = Point::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
        if point.hit_square() {
            square_counter += 1;
        } else if point.hit_circle() {
            circle_counter += 1;
        }
    }

    println!("🥧 {}", circle_counter as f64 / square_counter as f64);
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    pub fn hit_square(&self) -> bool {
        self.x < 1.0 && self.y < 1.0
    }
    pub fn hit_circle(&self) -> bool {
        let w = (2. - self.x).abs();
        let h = (1. - self.y).abs();
        (w * w + h * h).sqrt() < 1.0
    }
}
