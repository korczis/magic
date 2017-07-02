#![feature(test)]
extern crate test;

extern crate clap;
extern crate num;

pub mod core;
pub mod cpu;
pub mod ocl;
pub mod options;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
