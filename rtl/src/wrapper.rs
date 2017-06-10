extern crate sdr_rtl_sys;

use std::os::raw::{c_char};

pub struct Wrapper {}

impl Wrapper {
    pub unsafe fn rtlsdr_get_device_count() -> u32 {
        debug!("rtlsdr_get_device_count()");
        sdr_rtl_sys::rtlsdr_get_device_count()
    }

    pub unsafe fn rtlsdr_get_device_name(index: u32) ->  *const c_char {
        debug!("rtlsdr_get_device_name({})", index);
        sdr_rtl_sys::rtlsdr_get_device_name(index)
    }

    pub unsafe fn rtlsdr_get_device_usb_strings(index: u32, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32 {
        debug!("rtlsdr_get_device_usb_strings({}, {:?}, {:?}, {:?})", index, manufact, product, serial);
        sdr_rtl_sys::rtlsdr_get_device_usb_strings(index, manufact, product, serial)
    }

    pub unsafe fn rtlsdr_get_index_by_serial(serial: *const c_char) -> i32 {
        debug!("rtlsdr_get_index_by_serial({:?})", serial);
        sdr_rtl_sys::rtlsdr_get_index_by_serial(serial)
    }

    pub unsafe fn rtlsdr_open(dev: *mut *mut sdr_rtl_sys::rtlsdr_dev_t, index: u32) -> i32 {
        debug!("rtlsdr_open({:?}, {:?})", dev, index);
        sdr_rtl_sys::rtlsdr_open(dev, index)
    }

    pub unsafe fn rtlsdr_close(dev: *mut sdr_rtl_sys::rtlsdr_dev_t) -> i32 {
        debug!("rtlsdr_close({:?})", dev);
        sdr_rtl_sys::rtlsdr_close(dev)
    }
}
