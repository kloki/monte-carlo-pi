use clap::Parser;
use rand::Rng;
use rayon::prelude::*;

/// approximate Pi using a Monte Carlo Method. DONT USE THIS.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Amount of threads
    #[arg(short, long, default_value_t = 16)]
    threads: usize,

    /// Iteration per thread
    #[arg(short, long, default_value_t = 100000)]
    iterations: usize,
}

fn main() {
    let args = Args::parse();

    let pi: f64 = (0..args.threads)
        .into_par_iter()
        .map(|_| monte_carlo_run(args.iterations))
        .sum::<f64>()
        / args.threads as f64;
    println!(" 🎰 🥧 {}", pi);
}

fn monte_carlo_run(iterations: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let circle_hits: usize = (0..iterations)
        .map(|_| {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            if (x * x + y * y).sqrt() < 1. {
                return 1;
            }
            0
        })
        .sum();
    circle_hits as f64 / (iterations as f64 / 4.)
}
