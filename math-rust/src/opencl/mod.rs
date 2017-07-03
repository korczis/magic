extern crate glob;

use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;

use super::generic::Mathematics;

#[derive(Debug)]
pub struct Manager {
    kernels: HashMap<String, String>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            kernels: HashMap::new()
        }
    }

    pub fn load_kernels<S: Into<String> + fmt::Display>(&mut self, path: S) -> &mut Manager {
        let path = path.into();
        // TODO: Make sure there are no std::path::MAIN_SEPARATORs
        let pattern = format!("{}/**/*.cl", path);
        for entry in glob::glob(&pattern[..]).expect("Failed to read glob pattern") {
            match entry {
                Ok(kernel_path) => {
                    match File::open(&kernel_path) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            match f.read_to_string(&mut buffer) {
                                Ok(_) => {

                                    let kernel_path = kernel_path
                                        .into_os_string()
                                        .into_string()
                                        .unwrap()
                                        .replace(&path[..], "")
                                        .replace(".cl", "");

                                    info!("Loading kernel {:?}", &kernel_path);
                                    self.kernels.insert(kernel_path, buffer);
                                }
                                Err(e) => {
                                    println!("Unable to read file {:?}, reason: {:?}", &kernel_path, e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Unable to read file {:?}, reason: {:?}", &kernel_path, e);
                        }
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }

        self
    }

    pub fn get_kernel<S: Into<String>>(&self, name: S) -> Option<&String> {
        let name = name.into();
        self.kernels.get(&name)
    }

    pub fn device<'a>(&'a self) -> Device {
        let platform = ::ocl::Platform::default();
        let device = ::ocl::Device::first(&platform);
        // let device = ::ocl::Device::by_idx_wrap(platform, 1);

        Device {
            manager: self,
            device: device

        }
    }
}

pub struct Device<'a> {
    manager: &'a Manager,
    device: ::ocl::Device
}

impl<'a> Device<'a> {
    pub fn vec_op(&self, op: &str, a: &[f32], b: &[f32]) -> Vec<f32> {
        assert!(a.len() == b.len());

        let dims = a.len();

        let src: &String = self.manager.get_kernel("vector").unwrap();

        let pro_que = ::ocl::ProQue::builder()
            .device(self.device)
            .src(&src[..])
            .dims(dims)
            .build()
            .unwrap();

        let buffer_a = ::ocl::Buffer::builder()
            .queue(pro_que.queue().clone())
            .flags(::ocl::MemFlags::new().read_only().use_host_ptr())
            .dims(dims)
            .host_data(&a)
            .build().unwrap();

        let buffer_b = ::ocl::Buffer::builder()
            .queue(pro_que.queue().clone())
            .flags(::ocl::MemFlags::new().read_only().use_host_ptr())
            .dims(dims)
            .host_data(&b)
            .build().unwrap();

        let mut vec_result = vec![0.0f32; dims];
        let buffer_res: ::ocl::Buffer<f32> = pro_que.create_buffer().unwrap();

        let kernel = pro_que
            .create_kernel(op).unwrap()
            .arg_buf(&buffer_a)
            .arg_buf(&buffer_b)
            .arg_buf(&buffer_res);

        kernel.enq().unwrap();
        buffer_res.read(&mut vec_result).enq().unwrap();
        debug!("Kernel '{:?}' result {:?}", op, &buffer_res);

        vec_result
    }
}

impl<'a> Mathematics for Device<'a> {
    fn vec_add(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        self.vec_op("add", a, b)
    }

    fn vec_div(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        self.vec_op("div", a, b)
    }

    fn vec_mul(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        self.vec_op("mul", a, b)
    }

    fn vec_sub(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        self.vec_op("sub", a, b)
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use generic::Mathematics;

    const DIM: usize = 1 << 16;
    const A: [f32; DIM] = [1f32; DIM];
    const B: [f32; DIM] = [2f32; DIM];

    #[test]
    fn manager_loads_kernels() {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
    }

    #[test]
    fn device_vec_add() {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        let _ = device.vec_add(&A, &B);
    }

    #[test]
    fn device_vec_div() {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        let _ = device.vec_div(&A, &B);
    }

    #[test]
    fn device_vec_mul() {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        let _ = device.vec_mul(&A, &B);
    }

    #[test]
    fn device_vec_sub() {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        let _ = device.vec_sub(&A, &B);
    }

    #[bench]
    fn bench_device_vec_add(bencher: &mut Bencher) {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        bencher.iter(|| {
            let _c = device.vec_add(&A, &B);
        });
    }

    #[bench]
    fn bench_device_vec_div(bencher: &mut Bencher) {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        bencher.iter(|| {
            let _c = device.vec_div(&A, &B);
        });
    }

    #[bench]
    fn bench_device_vec_mul(bencher: &mut Bencher) {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        bencher.iter(|| {
            let _c = device.vec_mul(&A, &B);
        });
    }

    #[bench]
    fn bench_device_vec_sub(bencher: &mut Bencher) {
        let mut mgr = super::Manager::new();
        mgr.load_kernels("../data/kernels/");
        let device = mgr.device();

        bencher.iter(|| {
            let _c = device.vec_sub(&A, &B);
        });
    }
}
