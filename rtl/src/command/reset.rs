use clap::ArgMatches;

use super::super::*;

pub fn main(args: &ArgMatches) {
    let _hrf = match HackRF::init() {
        Ok(hrf) => hrf,
        Err(err) => {
            println!("Unable to init HackRF, reason: {:?}", err);
            return;
        }
    };

    let tmp = args.value_of("device").unwrap();
    let index: usize = match tmp.to_string().parse::<usize>() {
        Ok(val) => val,
        _ => {
            println!("Invalid device specified - {:?}", tmp);
            return;
        }
    };

    let devices = HackRF::device_list();

    if index > devices.len() {
        println!("Invalid device specified - {:?}", index);
        return;
    }

    let device = &devices[index];
    match device.reset() {
        Ok(_) => println!("Device {:?} was successfully reset", index),
        Err(err) => println!("Unable to reset device, reason: {:?}", err)
    }
}
