use std::os::raw::{c_char};

pub enum rtlsdr_dev_t {}

#[link(name = "rtlsdr")]
extern "C" {
    pub fn rtlsdr_get_device_count() -> u32;
    pub fn rtlsdr_get_device_name(index: u32) -> *const c_char;
    pub fn rtlsdr_get_device_usb_strings(index: u32, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32;
    pub fn rtlsdr_get_index_by_serial(serial: *const c_char) -> i32;

    pub fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: u32) -> i32;
    pub fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> i32;
}
