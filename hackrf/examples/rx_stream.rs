extern crate hackrf;
extern crate env_logger;

use hackrf::*;

fn main() {
    env_logger::init().unwrap();

    let _hrf = match HackRF::init() {
        Ok(hrf) => hrf,
        _ => {
            return;
        }
    };

    let mut devices = HackRF::device_list();

    let device = &mut devices[0];

    device.set_sample_rate(2_000_000 as f64);
    device.set_frequency(150_000_000);
}
