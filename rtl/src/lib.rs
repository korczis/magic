#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate sdr_rtl_sys;
extern crate serde;
extern crate serde_json;

// use serde::ser::{Serialize, Serializer, SerializeMap};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use std::ptr;

pub mod command;
pub mod wrapper;

#[derive(Debug)]
pub struct Device {
    handle: *mut sdr_rtl_sys::rtlsdr_dev_t
}


impl Drop for Device {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                Wrapper::rtlsdr_close(self.handle);
            }
        }
    }
}

impl Device {
    //! Set crystal oscillator frequencies used for the RTL2832 and the tuner IC.
    //!
    //! Usually both ICs use the same clock. Changing the clock may make sense if
    //! you are applying an external clock to the tuner or to compensate the
    //! frequency (and sample rate) error caused by the original (cheap) crystal.
    //!
    //! NOTE: Call this function only if you fully understand the implications.
    pub fn set_xtal_freq(&mut self, rtl_freq: u32, tuner_freq: u32) ->bool {
        unsafe {
            Wrapper::rtlsdr_set_xtal_freq(self.handle, rtl_freq, tuner_freq) == 0
        }
    }

    pub fn get_xtal_freq(&mut self) -> Option<(u32, u32)> {
        unsafe {
            let mut rtl_freq = 0;
            let mut tuner_freq = 0;
            if Wrapper::rtlsdr_get_xtal_freq(self.handle, &mut rtl_freq as *mut u32, &mut tuner_freq as *mut u32) == 0 {
                Some((rtl_freq, tuner_freq))
            } else {
                None
            }
        }
    }

    pub fn get_usb_strings(&mut self) -> Option<(String, String, String)> {
        unsafe {
            let mut manufact: [c_char; 256] = [0; 256];
            let mut product: [c_char; 256] = [0; 256];
            let mut serial: [c_char; 256] = [0; 256];

            if Wrapper::rtlsdr_get_usb_strings(self.handle, manufact.as_mut_ptr(), product.as_mut_ptr(), serial.as_mut_ptr()) == 0 {
                Some((
                    CStr::from_ptr(manufact.as_ptr()).to_string_lossy().into_owned(),
                    CStr::from_ptr(product.as_ptr()).to_string_lossy().into_owned(),
                    CStr::from_ptr(serial.as_ptr()).to_string_lossy().into_owned()
                ))
            } else {
                None
            }
        }
    }

    pub fn set_center_freq(dev: *mut sdr_rtl_sys::rtlsdr_dev_t, freq: u32) -> bool {
        unsafe {
            sdr_rtl_sys::rtlsdr_set_center_freq(dev, freq) == 0
        }
    }

    pub fn get_center_freq(&mut self) -> Option<u32> {
        unsafe {
            match Wrapper::rtlsdr_get_center_freq(self.handle) {
                0 => None,
                freq => Some(freq)
            }
        }
    }

    pub fn set_freq_correction(&mut self, ppm: u32) -> bool {
        unsafe {
            Wrapper::rtlsdr_set_center_freq(self.handle, ppm) == 0
        }
    }

    pub fn get_freq_correction(&mut self) -> i32 {
        unsafe {
            Wrapper::rtlsdr_get_freq_correction(self.handle)
        }
    }

    pub fn get_tuner_type(&mut self) -> sdr_rtl_sys::TunerType {
        unsafe {
            Wrapper::rtlsdr_get_tuner_type(self.handle)
        }
    }

    pub fn get_tuner_gains(&mut self) -> Vec<i32> {
        unsafe {
            let mut gains: [i32; 256] = [0; 256];
            let mut res = Vec::new();
            for i in 0..Wrapper::rtlsdr_get_tuner_gains(self.handle, gains.as_mut_ptr()) {
                res.push(gains[i as usize])
            }

            res
        }
    }

    pub fn set_tuner_gain(&mut self, gain: i32) -> bool {
        unsafe {
            Wrapper::rtlsdr_set_tuner_gain(self.handle, gain) == 0
        }
    }

    pub fn get_tuner_gain(&mut self) -> i32 {
        unsafe {
            Wrapper::rtlsdr_get_tuner_gain(self.handle)
        }
    }

    pub fn set_tuner_if_gain(&mut self, stage: i32, gain: i32) -> bool {
        unsafe {
            Wrapper::rtlsdr_set_tuner_if_gain(self.handle, stage, gain) == 0
        }
    }

    pub fn set_tuner_gain_mode(&mut self, manual: i32) -> bool {
        unsafe {
            Wrapper::rtlsdr_set_tuner_gain_mode(self.handle, manual) == 0
        }
    }

    pub fn set_sample_rate(&mut self, rate: u32) -> bool {
        unsafe {
            Wrapper::rtlsdr_set_sample_rate(self.handle, rate) == 0
        }
    }

    pub fn get_sample_rate(&mut self) -> Option<u32> {
        unsafe {
            match Wrapper::rtlsdr_get_sample_rate(self.handle) {
                0 => None,
                rate => Some(rate)
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

    pub fn get_device_name(index: u32) -> Option<String> {
        unsafe {
            let raw_name = Wrapper::rtlsdr_get_device_name(index);
            let res = CStr::from_ptr(raw_name).to_string_lossy().into_owned();
            if res == "" {
                None
            } else {
                Some(res)
            }
        }
    }

    pub fn get_device_usb_strings(index: u32) -> Option<(String, String, String)> {
        unsafe {
            let mut manufact: [c_char; 256] = [0; 256];
            let mut product: [c_char; 256] = [0; 256];
            let mut serial: [c_char; 256] = [0; 256];

            if Wrapper::rtlsdr_get_device_usb_strings(index, manufact.as_mut_ptr(), product.as_mut_ptr(), serial.as_mut_ptr()) == 0 {
                Some((
                    CStr::from_ptr(manufact.as_ptr()).to_string_lossy().into_owned(),
                    CStr::from_ptr(product.as_ptr()).to_string_lossy().into_owned(),
                    CStr::from_ptr(serial.as_ptr()).to_string_lossy().into_owned()
                ))
            } else {
                None
            }
        }
    }

    pub fn get_index_by_serial(serial: String) -> Result<u32, Error> {
        unsafe {
            let serial = CString::new(serial).unwrap().as_ptr();
            match Wrapper::rtlsdr_get_index_by_serial(serial as *const i8) {
                -1 => Err(Error::EmptyName),
                -2 => Err(Error::NoDevices),
                -3 => Err(Error::NoSuchDevice),
                index => Ok(index as u32)
            }
        }
    }

    pub fn open(index: u32) -> Option<Device> {
        let mut dev = Device {
            handle: ptr::null_mut()
        };

        unsafe {
            if Wrapper::rtlsdr_open(&mut dev.handle as *mut *mut sdr_rtl_sys::rtlsdr_dev_t, index) == 0 {
                Some(dev)
            } else {
                None
            }
        }
    }
}
