#![allow(dead_code)]

use num::*;

use super::generic::Mathematics;

pub fn vec_add<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] + b[i]);
    }

    res
}

pub fn vec_div<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] / b[i]);
    }

    res
}

pub fn vec_mul<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] * b[i]);
    }

    res
}

pub fn vec_sub<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] - b[i]);
    }

    res
}

pub struct Device {

}

impl Device {
    pub fn new() -> Device {
        Device {
        }
    }
}

impl Mathematics for Device {
    fn vec_add(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        vec_add(a, b)
    }

    fn vec_div(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        vec_div(a, b)
    }

    fn vec_mul(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        vec_mul(a, b)
    }

    fn vec_sub(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        vec_sub(a, b)
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
    fn device_vec_add() {
        let device = super::Device::new();

        let _ = device.vec_add(&A, &B);
    }

    #[test]
    fn device_vec_div() {
        let device = super::Device::new();

        let _ = device.vec_div(&A, &B);
    }

    #[test]
    fn device_vec_mul() {
        let device = super::Device::new();

        let _ = device.vec_mul(&A, &B);
    }

    #[test]
    fn device_vec_sub() {
        let device = super::Device::new();

        let _ = device.vec_sub(&A, &B);
    }

    #[bench]
    fn bench_device_vec_add(bencher: &mut Bencher) {
        let device = super::Device::new();

        bencher.iter(|| {
            let _c = device.vec_add(&A, &B);
        });
    }

    #[bench]
    fn bench_device_vec_div(bencher: &mut Bencher) {
        let device = super::Device::new();

        bencher.iter(|| {
            let _c = device.vec_div(&A, &B);
        });
    }

    #[bench]
    fn bench_device_vec_mul(bencher: &mut Bencher) {
        let device = super::Device::new();

        bencher.iter(|| {
            let _c = device.vec_mul(&A, &B);
        });
    }

    #[bench]
    fn bench_device_vec_sub(bencher: &mut Bencher) {
        let device = super::Device::new();

        bencher.iter(|| {
            let _c = device.vec_sub(&A, &B);
        });
    }
}