#[macro_use]
extern crate log;

extern crate sdr_hackrf;
extern crate sdr_hackrf_sys;
extern crate env_logger;

use std::os::raw::{c_int};
use std::ptr;
use std::slice;
use std::thread;

use sdr_hackrf::*;

fn main() {
    env_logger::init().unwrap();

    let _hrf = match HackRF::init() {
        Ok(hrf) => hrf,
        Err(err) => {
            println!("Unable to init HackRF, reason: {:?}", err);
            return;
        }
    };

    let handle = thread::spawn(move || {
        let mut devices = Box::new(HackRF::device_list());

        if devices.len() < 1 {
            println!("No compatible devices found. Exiting.");
            return;
        }

        let device = &mut devices[0];

        if let Err(err) = device.set_sample_rate(2_000_000 as f64) {
            println!("Unable to set_sample_rate, reason: {:?}", err);
            return;
        }

        if let Err(err) = device.set_frequency(100_000_000) {
            println!("Unable to set_frequency(), reason: {:?}", err);
            return;
        }

        unsafe extern "C" fn rx_callback(transfer: *mut sdr_hackrf_sys::Transfer) -> c_int {
            debug!("{:?}", *transfer);

            let slice = slice::from_raw_parts((*transfer).buffer, (*transfer).valid_length as usize);
            debug!("{:?}", &slice);
            0
        }

        if let Err(err) = device.start_rx(rx_callback, ptr::null_mut()) {
            println!("Unable to set_frequency(), reason: {:?}", err);
            return;
        }

        loop {}
    });

    let _ = handle.join();
}
