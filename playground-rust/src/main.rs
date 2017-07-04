extern crate rayon;

use rayon::prelude::*;

const ITERATIONS: usize = 10_000_000_000;

fn pi() -> f64 {
    let mut sum = 0f64;
    for i in 1..ITERATIONS {
        let a = (-1f64).powf((i as f64) + 1f64);
        let b = (2f64 * i as f64) - 1f64;
        sum += a / b;
    }

    sum * 4f64
}

fn par_pi() -> f64 {
    let res: f64 = (1..ITERATIONS).into_par_iter()
        .map(|i| {
            let a = (-1f64).powf((i as f64) + 1f64);
            let b = (2f64 * i as f64) - 1f64;
            a / b
        })
        .sum();

    res * 4f64
}

fn main() {
    println!("pi = {:?}", pi());
}
