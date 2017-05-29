extern crate hackrf_sys;

use std::os::raw::{c_char, c_int};

pub struct Wrapper {}

impl Wrapper {
    pub unsafe fn hackrf_init() -> hackrf_sys::Error {
        debug!("hackrf_init()");
        hackrf_sys::hackrf_init()
    }

    pub unsafe fn hackrf_board_partid_serialno_read(device: *const hackrf_sys::Device, serial: *mut hackrf_sys::ReadPartIdSerialno) -> hackrf_sys::Error {
        debug!("hackrf_board_partid_serialno_read({:?}, {:?})", device, serial);
        hackrf_sys::hackrf_board_partid_serialno_read(device, serial)
    }

    pub unsafe fn hackrf_close(device: *mut hackrf_sys::Device) -> hackrf_sys::Error {
        debug!("hackrf_close({:?})", device);
        hackrf_sys::hackrf_close(device)
    }

    pub unsafe fn hackrf_exit() -> hackrf_sys::Error {
        debug!("hackrf_exit()");
        hackrf_sys::hackrf_exit()
    }

    pub unsafe fn hackrf_device_list() -> *mut hackrf_sys::DeviceList {
        debug!("hackrf_device_list()");
        hackrf_sys::hackrf_device_list()
    }

    pub unsafe fn hackrf_device_list_free(list: *mut hackrf_sys::DeviceList) {
        debug!("hackrf_device_list_free({:?})", list);
        hackrf_sys::hackrf_device_list_free(list)
    }

    pub unsafe fn hackrf_device_list_open(list: *mut hackrf_sys::DeviceList, idx: c_int, device: *mut *mut hackrf_sys::Device) -> hackrf_sys::Error {
        debug!("hackrf_device_list_open({:?}, {:?}, {:?})", list, idx, device);
        hackrf_sys::hackrf_device_list_open(list, idx, device)
    }

    pub unsafe fn hackrf_reset(device: *const hackrf_sys::Device) -> hackrf_sys::Error {
        debug!("hackrf_reset({:?})", device);
        hackrf_sys::hackrf_reset(device)
    }

    pub unsafe fn hackrf_set_freq(device: *const hackrf_sys::Device, freq_hz: u64) -> hackrf_sys::Error {
        debug!("hackrf_set_freq({:?}, {:?})", device, freq_hz);
        hackrf_sys::hackrf_set_freq(device, freq_hz)
    }

    pub unsafe fn hackrf_set_sample_rate(device: *const hackrf_sys::Device, freq_hz: f64) -> hackrf_sys::Error {
        debug!("hackrf_set_sample_rate({:?}, {:?})", device, freq_hz);
        hackrf_sys::hackrf_set_sample_rate(device, freq_hz)
    }

    pub unsafe fn hackrf_usb_api_version_read(device: *const hackrf_sys::Device, version: *mut u16) -> hackrf_sys::Error {
        debug!("hackrf_usb_api_version_read({:?}, {:?})", device, version);
        hackrf_sys::hackrf_usb_api_version_read(device, version)
    }

    pub unsafe fn hackrf_version_string_read(device: *const hackrf_sys::Device, version: *mut c_char, length: u8) -> hackrf_sys::Error {
        debug!("hackrf_version_string_read({:?}, {:?}, {:?})", device, version, length);
        hackrf_sys::hackrf_version_string_read(device, version, length)
    }
}
