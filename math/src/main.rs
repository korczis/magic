extern crate clap;
extern crate env_logger;
extern crate time;

extern crate math;

extern crate ocl;
use ocl::ProQue;

use clap::{App, Arg};
use math::*;
use std::env;
use time::*;

const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn cpu_vec_mul(a: &[f32], b: &[f32]) -> Vec<f32> {
    math::cpu::add(a, b)
}

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

    let mut mgr = math::ocl::Manager::new();
    mgr.load_kernels(opts.kernels_path);

    fn cl_vec_mul(mgr: &math::ocl::Manager, a: &[f32], b: &[f32]) -> Vec<f32> {
        let dims = a.len();

        let platform = ocl::Platform::default();
        let device = ocl::Device::by_idx_wrap(platform, 1);
        let src: &String = mgr.get_kernel("vector").unwrap();

        // println!("{:?}", ocl::core::get_device_info(&device, ocl::core::DeviceInfo::Type));

        let pro_que = ProQue::builder()
            .device(device)
            .src(&src[..])
            .dims(dims)
            .build()
            .unwrap();

        let buffer_a = ocl::Buffer::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::MemFlags::new().read_write().copy_host_ptr())
            .dims(dims)
            .host_data(&a)
            .build().unwrap();

        let buffer_b = ocl::Buffer::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::MemFlags::new().read_write().copy_host_ptr())
            .dims(dims)
            .host_data(&b)
            .build().unwrap();

        let mut vec_result = vec![0.0f32; dims];
        let buffer_res: ocl::Buffer<f32> = pro_que.create_buffer().unwrap();

        const KERNEL_NAME: &str = "mul";
        let kernel = pro_que
            .create_kernel(KERNEL_NAME).unwrap()
            .arg_buf(&buffer_a)
            .arg_buf(&buffer_b)
            .arg_buf(&buffer_res);

        println!("Running kernel {:?}", KERNEL_NAME);
        kernel.enq().unwrap();


        buffer_res.read(&mut vec_result).enq().unwrap();
        vec_result
    }


    const DIM: usize = 1 << 24;

    let a = vec![1f32; DIM];
    let b = vec![2f32; DIM];

    let start = PreciseTime::now();
    let res1 = cl_vec_mul(&mgr, &a, &b);
    let diff = start.to(PreciseTime::now());
    let elapsed_secs = diff.num_nanoseconds().unwrap() as f64 * 1e-9;
    println!(" CL: Vector size: {:?}, elapsed {:?} secs.", &DIM, &elapsed_secs);

    let start = PreciseTime::now();
    let res2 = cpu_vec_mul(&a, &b);
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

