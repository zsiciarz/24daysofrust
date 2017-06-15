#![feature(test)]

extern crate test;

extern crate num_cpus;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;

fn monte_carlo_pi(points: usize) -> f64 {
    let within_circle = (0..points)
        .filter_map(|_| {
            let x = rand::random::<f64>() * 2f64 - 1f64;
            let y = rand::random::<f64>() * 2f64 - 1f64;
            if x * x + y * y <= 1f64 { Some(1) } else { None }
        })
        .count();
    4f64 * within_circle as f64 / points as f64
}

fn parallel_monte_carlo_pi(points: usize) -> f64 {
    let within_circle = (0..points)
        .into_par_iter()
        .filter_map(|_| {
            let x = rand::random::<f64>() * 2f64 - 1f64;
            let y = rand::random::<f64>() * 2f64 - 1f64;
            if x * x + y * y <= 1f64 { Some(1) } else { None }
        })
        .count();
    4f64 * within_circle as f64 / points as f64
}

fn semicircle(x: f64) -> f64 {
    (1f64 - x * x).sqrt()
}

fn integrate<F>(f: F, points: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let delta = 1f64 / points as f64;
    (0..points)
        .map(|i| {
            let a = i as f64 * delta;
            delta * (f(a) + f(a + delta)) / 2f64
        })
        .sum()
}

fn parallel_integrate<F>(f: F, points: usize) -> f64
where
    F: Fn(f64) -> f64 + Sync,
{
    let delta = 1f64 / points as f64;
    (0..points)
        .into_par_iter()
        .map(|i| {
            let a = i as f64 * delta;
            delta * (f(a) + f(a + delta)) / 2f64
        })
        .sum()
}

fn main() {
    println!("24 Days of Rust vol. 2 - rayon");
    println!("This machine has {} CPUs", num_cpus::get());
    println!("sequential: {}", 4f64 * integrate(semicircle, 10_000_000));
    println!(
        "parallel: {}",
        4f64 * parallel_integrate(semicircle, 10_000_000)
    );
    println!("sequential MC: {}", monte_carlo_pi(1_000_000));
    println!("parallel MC: {}", parallel_monte_carlo_pi(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::{semicircle, integrate, parallel_integrate, monte_carlo_pi, parallel_monte_carlo_pi};
    use test::Bencher;

    #[bench]
    fn bench_sequential_integrate(b: &mut Bencher) {
        b.iter(|| integrate(semicircle, 100_000))
    }

    #[bench]
    fn bench_parallel_integrate(b: &mut Bencher) {
        b.iter(|| parallel_integrate(semicircle, 100_000))
    }

    #[bench]
    fn bench_sequential_monte_carlo(b: &mut Bencher) {
        b.iter(|| monte_carlo_pi(100_000))
    }

    #[bench]
    fn bench_parallel_monte_carlo(b: &mut Bencher) {
        b.iter(|| parallel_monte_carlo_pi(100_000))
    }
}
