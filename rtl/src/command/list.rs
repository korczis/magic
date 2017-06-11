use super::super::*;

pub fn main() {
    println!("Count: {}", RTL::get_device_count());
    println!("Device Name: {:?}", RTL::get_device_name(0));
    println!("USB Strings: {:?}", RTL::get_device_usb_strings(0));
}
