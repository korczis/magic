#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate sdr_hackrf_sys;
extern crate serde;
extern crate serde_json;

use serde::ser::{Serialize, Serializer, SerializeMap};

use std::ffi::{CStr};
use std::os::raw::{c_char, c_void};
use std::ptr;

pub mod command;
pub mod shared;
pub mod wrapper;

pub struct HackRF {}

use self::wrapper::*;

impl HackRF {
    pub fn init() -> Result<HackRF, sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_init() {
                sdr_hackrf_sys::Error::Success => {
                    Result::Ok(HackRF {})
                }
                err => {
                    Result::Err(err)
                }
            }
        }
    }

    pub fn device_list() -> Vec<Device> {
        unsafe {
            let mut list = DeviceList::default();
            let raw_devices = list.as_raw();

            let mut res: Vec<Device> = Vec::new();

            for i in 0 .. (*raw_devices).devicecount {
                res.push(Device::open(raw_devices, i));
            }

            res
        }
    }
}

impl Drop for HackRF {
    fn drop(&mut self) {
        unsafe {
            let _ = Wrapper::hackrf_exit();
        }
    }
}

//#[derive(Serialize, Deserialize)]
//pub struct ApiVersion {
//    pub major: u8,
//    pub minor: u8
//}

pub struct Device {
    index: i32,
    model: sdr_hackrf_sys::UsbBoardId,
    serial_string: String,
    device: *mut sdr_hackrf_sys::Device
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    pub unsafe fn open(list: *mut sdr_hackrf_sys::DeviceList, index: i32) -> Device {
        let mut device: *mut sdr_hackrf_sys::Device = ptr::null_mut();
        Wrapper::hackrf_device_list_open(list, index, &mut device);

        let raw_sn = *(*list).serial_numbers.offset(index as isize) as *const c_char;
        let sn = CStr::from_ptr(raw_sn).to_string_lossy().into_owned();

        Device {
            index: *(*list).usb_device_index.offset(index as isize) as i32,
            model: *(*list).usb_board_ids.offset(index as isize),
            serial_string: sn,
            device: device
        }
    }

    pub unsafe fn firmware(&self) -> Result<String, sdr_hackrf_sys::Error> {
        let mut buffer: [c_char; 256] = [0; 256];
        match Wrapper::hackrf_version_string_read(self.device, buffer.as_mut_ptr(), 255) {
            sdr_hackrf_sys::Error::Success => Ok(CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()),
            err => Result::Err(err)
        }
    }

    pub unsafe fn serial(&self) -> Result<sdr_hackrf_sys::ReadPartIdSerialno, sdr_hackrf_sys::Error> {
        let mut res = sdr_hackrf_sys::ReadPartIdSerialno {
            part_id: [0; 2],
            serial_no: [0; 4]
        };
        match Wrapper::hackrf_board_partid_serialno_read(self.device, &mut res) {
            sdr_hackrf_sys::Error::Success => Ok(res),
            err => Result::Err(err)
        }
    }

    pub fn reset(&self) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_reset(self.device) {
                sdr_hackrf_sys::Error::Success => Ok(()),
                err => Result::Err(err)
            }
        }
    }

    pub fn set_frequency(&self, freq_hz: u64) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_set_freq(self.device, freq_hz) {
                sdr_hackrf_sys::Error::Success => Ok(()),
                err => Result::Err(err)
            }
        }
    }

    pub fn set_sample_rate(&self, freq_hz: f64) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_set_sample_rate(self.device, freq_hz) {
                sdr_hackrf_sys::Error::Success => Ok(()),
                err => Result::Err(err)
            }
        }
    }

    pub fn start_rx(&self, callback: sdr_hackrf_sys::SampleBlockCallback, rx_ctx: *mut c_void) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_start_rx(self.device, callback, rx_ctx) {
                sdr_hackrf_sys::Error::Success => {
                    Ok(())
                },
                err => Result::Err(err)
            }
        }
    }
    pub fn start_tx(&self, callback: sdr_hackrf_sys::SampleBlockCallback, tx_ctx: *mut c_void) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_start_tx(self.device, callback, tx_ctx) {
                sdr_hackrf_sys::Error::Success => {
                    Ok(())
                },
                err => Result::Err(err)
            }
        }
    }

    pub fn stop_rx(&self) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_stop_rx(self.device) {
                sdr_hackrf_sys::Error::Success => {
                    Ok(())
                },
                err => Result::Err(err)
            }
        }
    }

    pub fn stop_tx(&self) -> Result<(), sdr_hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_stop_tx(self.device) {
                sdr_hackrf_sys::Error::Success => {
                    Ok(())
                },
                err => Result::Err(err)
            }
        }
    }

    pub fn version(&self) -> Result<(u8, u8), sdr_hackrf_sys::Error> {
        unsafe {
            let mut version: u16 = 0;
            match Wrapper::hackrf_usb_api_version_read(self.device, &mut version) {
                sdr_hackrf_sys::Error::Success => {
                    let version_parts: *const u8 = &version as *const _ as *const u8;
                    Ok((*version_parts.offset(1), *version_parts.offset(0)))
                },
                err => Result::Err(err)
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            Wrapper::hackrf_close(self.device);
        }
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        unsafe {
            let mut map = serializer.serialize_map(Some(6)).unwrap();

            map.serialize_entry("index", &self.index).unwrap();
            map.serialize_entry("model", &self.model).unwrap();
            map.serialize_entry("version", &self.version().unwrap()).unwrap();
            map.serialize_entry("firmware", &self.firmware().unwrap()).unwrap();
            map.serialize_entry("serial", &self.serial().unwrap()).unwrap();
            map.serialize_entry("serial_string", &self.serial_string).unwrap();

            map.end()
        }
    }
}

struct DeviceList {
    list: *mut sdr_hackrf_sys::DeviceList
}

impl DeviceList {
    pub unsafe fn as_raw(&mut self) -> *mut sdr_hackrf_sys::DeviceList {
        self.list
    }
}

impl Default for DeviceList {
    fn default() -> DeviceList {
        unsafe {
            DeviceList {
                list: Wrapper::hackrf_device_list()
            }
        }
    }
}

impl Drop for DeviceList {
    fn drop(&mut self) {
        unsafe {
            println!("Test");
            Wrapper::hackrf_device_list_free(self.list);
        }
    }
}
