use rand::Rng;

fn main() {
    println!(" 🎰 🥧 {}", monte_carlo_run(100000000));
}

fn monte_carlo_run(iterations: usize) -> f64 {
    let mut circle_counter = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let x: f64 = rng.gen_range(0.0..2.0) - 1.;
        let y: f64 = rng.gen_range(0.0..2.0) - 1.;
        if (x * x + y * y).sqrt() < 1. {
            circle_counter += 1;
        }
    }
    circle_counter as f64 / (iterations as f64 / 4.)
}
