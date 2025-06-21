use std::{f64::consts::PI, time::Duration};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

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

/// approximate Pi using a Monte Carlo Method. DONT USE THIS.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Amount of threads
    #[arg(short, long, default_value_t = 16, env = "THREADS")]
    threads: usize,

    /// Iteration per thread
    #[arg(short, long, default_value_t = 1000000, env = "ITERATIONS")]
    iterations: usize,
}

fn main() {
    let args = Args::parse();
    println!("🤞 Iterations:{} Threads:{}", args.iterations, args.threads);
    let pb = progress_bar();
    pb.set_message("Crunching the numbers");
    let pi: f64 = (0..args.threads)
        .into_par_iter()
        .map(|_| monte_carlo_run(args.iterations))
        .sum::<f64>()
        / args.threads as f64;
    pb.finish_with_message("Done");
    println!("🥧 Real:     {PI:.8}");
    println!("🎰 Found:    {pi:.8}");
    println!("🤓 Accuracy: {:.4}%", (1. - (pi - PI).abs() / PI) * 100.0);
}

fn monte_carlo_run(iterations: usize) -> f64 {
    let mut rng = rand::rng();
    let circle_hits: usize = (0..iterations)
        .map(|_| {
            let x: f64 = rng.random();
            let y: f64 = rng.random();
            if (x * x + y * y).sqrt() < 1. {
                return 1;
            }
            0
        })
        .sum();
    circle_hits as f64 / (iterations as f64 / 4.)
}
