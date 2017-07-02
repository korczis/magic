#![allow(dead_code)]

use num::*;

// use super::core::*;

pub fn add<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] + b[i]);
    }

    res
}

pub fn div<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] / b[i]);
    }

    res
}

pub fn mul<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] * b[i]);
    }

    res
}

pub fn sub<T: Float>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(a.len() == b.len());

    let mut res = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        res.push(a[i] - b[i]);
    }

    res
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn bench_add(bencher: &mut Bencher) {
        let a = [0f32; 1024];
        let b = [10f32; 1024];

        bencher.iter(|| {
            let _c = super::add(&a, &b);
        });
    }

    #[bench]
    fn bench_div(bencher: &mut Bencher) {
        let a = [0f32; 1024];
        let b = [10f32; 1024];

        bencher.iter(|| {
            let _c = super::div(&a, &b);
        });
    }

    #[bench]
    fn bench_mul(bencher: &mut Bencher) {
        let a = [0f32; 1024];
        let b = [10f32; 1024];

        bencher.iter(|| {
            let _c = super::mul(&a, &b);
        });
    }

    #[bench]
    fn bench_sub(bencher: &mut Bencher) {
        let a = [0f32; 1024];
        let b = [10f32; 1024];

        bencher.iter(|| {
            let _c = super::sub(&a, &b);
        });
    }
}