use super::super::*;

pub fn main() {
    println!("{}", RTL::get_device_count());
    println!("{:?}", RTL::get_device_name(0));
    println!("{:?}", RTL::get_device_usb_strings(0));
    println!("{:?}", RTL::get_index_by_serial(String::from("00000001")));

    let dev = RTL::open(0 as u32);
}
