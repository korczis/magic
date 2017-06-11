use clap::ArgMatches;

use super::super::*;

pub fn main(_args: &ArgMatches) {
    match RTL::open(0 as u32) {
        Some(mut dev) => {
            println!("{:?}", dev.get_xtal_freq());
            println!("{:?}", dev.get_usb_strings());
            println!("{:?}", dev.get_center_freq());
            println!("{:?}", dev.get_freq_correction());
            println!("{:?}", dev.get_tuner_type());
            println!("{:?}", dev.get_tuner_gains());
            println!("{:?}", dev.get_tuner_gain());
            println!("{:?}", dev.get_sample_rate());
            println!("{:?}", dev.get_direct_sampling());
            println!("{:?}", dev.get_offset_tuning());
        },
        None => {
            println!("Unable to open device");
        }
    }
}
