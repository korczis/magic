#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate sdr_rtl_sys;
extern crate serde;
extern crate serde_json;

use serde::ser::{Serialize, Serializer, SerializeMap};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;

pub mod command;
pub mod wrapper;

#[derive(Debug)]
pub struct Device {
    handle: *mut sdr_rtl_sys::rtlsdr_dev_t
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.handle != ptr::null_mut() {
            unsafe {
                Wrapper::rtlsdr_close(self.handle);
            }
        }
    }
}

pub struct RTL {}

use self::wrapper::*;

#[derive(Debug)]
pub enum Error {
    Success = 0,
    EmptyName = -1,
    NoDevices = -2,
    NoSuchDevice = -3,
}

impl RTL {
    pub fn get_device_count() -> u32 {
        unsafe {
            Wrapper::rtlsdr_get_device_count()
        }
    }

    pub fn get_device_name(index: u32) -> Result<String, bool> {
        unsafe {
            let raw_name = Wrapper::rtlsdr_get_device_name(index);
            let res = CStr::from_ptr(raw_name).to_string_lossy().into_owned();
            if res == String::from("") {
                Err(false)
            } else {
                Ok(res)
            }
        }
    }

    pub fn get_device_usb_strings(index: u32) -> Result<(String, String, String), bool> {
        unsafe {
            let mut manufact: [c_char; 256] = [0; 256];
            let mut product: [c_char; 256] = [0; 256];
            let mut serial: [c_char; 256] = [0; 256];

            if Wrapper::rtlsdr_get_device_usb_strings(index, manufact.as_mut_ptr(), product.as_mut_ptr(), serial.as_mut_ptr()) == 0 {
                Ok((
                    CStr::from_ptr(manufact.as_ptr()).to_string_lossy().into_owned(),
                    CStr::from_ptr(product.as_ptr()).to_string_lossy().into_owned(),
                    CStr::from_ptr(serial.as_ptr()).to_string_lossy().into_owned()
                ))
            } else {
                Err(false)
            }
        }
    }

    pub fn get_index_by_serial(serial: String) -> Result<u32, Error> {
        unsafe {
            match Wrapper::rtlsdr_get_index_by_serial(CString::new(serial).unwrap().as_ptr() as *const i8) {
                -1 => Err(Error::EmptyName),
                -2 => Err(Error::NoDevices),
                -3 => Err(Error::NoSuchDevice),
                index => Ok(index as u32)
            }
        }
    }

    pub fn open(index: u32) -> Result<Device, bool> {
        let mut dev = Device {
            handle: ptr::null_mut()
        };

        unsafe {
            if Wrapper::rtlsdr_open(&mut dev.handle as *mut *mut sdr_rtl_sys::rtlsdr_dev_t, index) == 0 {
                Ok(dev)
            } else {
                Err(false)
            }
        }
    }
}

impl Drop for RTL {
    fn drop(&mut self) {

    }
}
