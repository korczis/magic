extern crate hackrf_sys;

use std::os::raw::{c_char, c_int};

pub struct Wrapper {}

impl Wrapper {
    pub fn hackrf_init() -> hackrf_sys::Error {
        debug!("hackrf_init()");
        unsafe  {
            hackrf_sys::hackrf_init()
        }
    }

    pub fn hackrf_board_partid_serialno_read(device: *const hackrf_sys::Device, serial: *mut hackrf_sys::ReadPartIdSerialno) -> hackrf_sys::Error {
        debug!("hackrf_board_partid_serialno_read({:?}, {:?})", device, serial);
        unsafe  {
            hackrf_sys::hackrf_board_partid_serialno_read(device, serial)
        }
    }

    pub fn hackrf_close(device: *mut hackrf_sys::Device) -> hackrf_sys::Error {
        debug!("hackrf_close({:?})", device);
        unsafe  {
            hackrf_sys::hackrf_close(device)
        }
    }

    pub fn hackrf_exit() -> hackrf_sys::Error {
        debug!("hackrf_exit()");
        unsafe  {
            hackrf_sys::hackrf_exit()
        }
    }

    pub fn hackrf_device_list() -> *mut hackrf_sys::DeviceList {
        debug!("hackrf_device_list()");
        unsafe  {
            hackrf_sys::hackrf_device_list()
        }
    }

    pub fn hackrf_device_list_free(list: *mut hackrf_sys::DeviceList) {
        debug!("hackrf_device_list_free({:?})", list);
        unsafe  {
            hackrf_sys::hackrf_device_list_free(list)
        }
    }

    pub fn hackrf_device_list_open(list: *mut hackrf_sys::DeviceList, idx: c_int, device: *mut *mut hackrf_sys::Device) -> hackrf_sys::Error {
        debug!("hackrf_device_list_open({:?}, {:?}, {:?})", list, idx, device);
        unsafe  {
            hackrf_sys::hackrf_device_list_open(list, idx, device)
        }
    }

    pub fn hackrf_usb_api_version_read(device: *const hackrf_sys::Device, version: *mut u16) -> hackrf_sys::Error {
        debug!("hackrf_usb_api_version_read({:?}, {:?})", device, version);
        unsafe  {
            hackrf_sys::hackrf_usb_api_version_read(device, version)
        }
    }

    pub fn hackrf_version_string_read(device: *const hackrf_sys::Device, version: *mut c_char, length: u8) -> hackrf_sys::Error {
        debug!("hackrf_version_string_read({:?}, {:?}, {:?})", device, version, length);
        unsafe  {
            hackrf_sys::hackrf_version_string_read(device, version, length)
        }
    }
}
