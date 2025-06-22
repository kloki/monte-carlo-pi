use std::{f64::consts::PI, time::Duration};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

/// approximate Pi using a Monte Carlo Method. DONT USE THIS.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Amount of threads
    #[arg(short, long, default_value_t = 16, env = "THREADS")]
    threads: usize,

    /// Iteration per thread
    #[arg(short, long, default_value_t = 10000000, env = "ITERATIONS")]
    iterations: usize,
}

fn main() {
    let args = Args::parse();
    println!("🤞 Iterations:{} Threads:{}", args.iterations, args.threads);
    let pb = progress_bar();
    pb.set_message("Crunching the numbers");
    let pi = find_pi(args.iterations, args.threads);
    pb.finish_with_message("Done");
    println!("🥧 Real:     {PI:.8}");
    println!("🎰 Found:    {pi:.8}");
    println!("🤓 Accuracy: {:.4}%", (1. - (pi - PI).abs() / PI) * 100.0);
}

pub fn progress_bar() -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(500));
    bar.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["🙈", "🙉", "🙊", "🙊"]),
    );
    bar
}

fn find_pi(iterations: usize, threads: usize) -> f64 {
    (0..threads)
        .into_par_iter()
        .map(|_| monte_carlo_run(iterations))
        .sum::<f64>()
        / threads as f64
}

fn monte_carlo_run(i: usize) -> f64 {
    let mut rng = rand::rng();
    fn throw(x: f64, y: f64) -> bool {
        (x * x + y * y).sqrt() < 1.
    }
    let circle_hits = (0..i).filter(|_| throw(rng.random(), rng.random())).count();
    circle_hits as f64 / (i as f64 / 4.)
}
