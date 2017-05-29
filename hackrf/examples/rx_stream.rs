extern crate hackrf;
extern crate env_logger;

use hackrf::*;

fn main() {
    env_logger::init().unwrap();

    let _hrf = match HackRF::init() {
        Ok(hrf) => hrf,
        Err(err) => {
            println!("Unable to init HackRF, reason: {:?}", err);
            return;
        }
    };

    let mut devices = HackRF::device_list();

    let device = &mut devices[0];

    // device.reset().unwrap();

    if let Err(err) = device.set_sample_rate(2_000_000 as f64) {
        println!("Unable to set_sample_rate, reason: {:?}", err);
        return;
    }

    if let Err(err) = device.set_frequency(150_000_000) {
        println!("Unable to set_frequency(), reason: {:?}", err);
        return;
    }
}
