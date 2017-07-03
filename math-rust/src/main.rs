extern crate clap;
extern crate env_logger;
extern crate time;

extern crate math;

extern crate ocl;

use clap::{App, Arg};
use math::*;
use math::generic::Mathematics;
use std::env;
use time::*;

const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author(AUTHOR)
        .about("Math CLI")
        .arg(Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true))
        .arg(Arg::with_name("kernels-path")
            .help("Path to kernels")
            .default_value("data/kernels")
            .required(true)
            .long("kernels-path"))
        .get_matches();

    match matches.occurrences_of("verbose") {
        0 => {}
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        _ => env::set_var("RUST_LOG", "debug"),
    }

    env_logger::init().unwrap();

    let opts = options::Options::from(&matches);

    let mut mgr = math::opencl::Manager::new();
    mgr.load_kernels(opts.kernels_path);

    const DIM: usize = 1 << 24;

    let a = vec![1f32; DIM];
    let b = vec![2f32; DIM];

    let start = PreciseTime::now();
    let res1 = mgr.device().vec_mul(&a, &b);
    let diff = start.to(PreciseTime::now());
    let elapsed_secs = diff.num_nanoseconds().unwrap() as f64 * 1e-9;
    println!(" CL: Vector size: {:?}, elapsed {:?} secs.", &DIM, &elapsed_secs);

    let start = PreciseTime::now();
    let res2 = math::cpu::vec_mul(&a, &b);
    let diff = start.to(PreciseTime::now());
    let elapsed_secs = diff.num_nanoseconds().unwrap() as f64 * 1e-9;
    println!("CPU: Vector size: {:?}, elapsed {:?} secs.", &DIM, &elapsed_secs);

    let mut same = true;
    for i in 0..res1.len() {
        if res1[i] != res2[i] {
            same = false;
            break;
        }
    }
    println!("{:?}", same);

    // println!("{:?}", res1);
    // println!("{:?}", res2);
}

