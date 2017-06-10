use super::super::*;

pub fn main() {
//    println!("{}", RTL::get_device_count());
//    println!("{:?}", RTL::get_device_name(0));
//    println!("{:?}", RTL::get_device_usb_strings(0));
//    println!("{:?}", RTL::get_index_by_serial(String::from("00000001")));

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
        },
        None => {
            println!("Unable to open device");
        }
    }
}
