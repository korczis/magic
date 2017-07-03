#![feature(test)]
extern crate test;

#[macro_use]
extern crate log;

extern crate clap;
extern crate num;
extern crate ocl;

pub mod core;
pub mod cpu;
pub mod generic;
pub mod opencl;
pub mod options;

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//    }
//}
