#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hackrf_sys;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::ffi::{CStr};
use std::os::raw::{c_char};
use std::ptr;

pub mod command;
pub mod wrapper;

pub struct HackRF {}

use self::wrapper::*;

impl HackRF {
    // Real interface
    pub fn init() -> Result<HackRF, hackrf_sys::Error> {
        match Wrapper::hackrf_init() {
            hackrf_sys::Error::Success => {
                Result::Ok(HackRF {})
            }
            err => {
                Result::Err(err)
            }
        }
    }

    pub fn device_list() -> Vec<Device> {
        let raw_devices = Wrapper::hackrf_device_list();

        let mut res: Vec<Device> = Vec::new();

        unsafe {
            // println!("{:?}", (*raw_devices).devicecount);
            for i in 0 .. (*raw_devices).devicecount {
                let raw_sn = *(*raw_devices).serial_numbers.offset(i as isize) as *const c_char;
                let sn = CStr::from_ptr(raw_sn).to_string_lossy().into_owned();

                let mut device: *mut hackrf_sys::Device = ptr::null_mut();

                // TODO: Check return code
                Wrapper::hackrf_device_list_open(raw_devices, i, &mut device);

                // TODO: Check return code
                let mut version: u16 = 0;
                Wrapper::hackrf_usb_api_version_read(device, &mut version);
                let version_parts: *const u8 = &version as *const _ as *const u8;

                // TODO: Check return code
                let mut buffer: [c_char; 256] = [0; 256];
                Wrapper::hackrf_version_string_read(device, buffer.as_mut_ptr(), 255);
                let firmware = CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned();

                let mut read_part = hackrf_sys::ReadPartIdSerialno {
                    part_id: [0; 2],
                    serial_no: [0; 4]
                };
                Wrapper::hackrf_board_partid_serialno_read(device, &mut read_part);

                println!("{:?}", read_part);

                res.push(Device {
                    index: *(*raw_devices).usb_device_index.offset(i as isize) as i32,
                    model: *(*raw_devices).usb_board_ids.offset(i as isize),
                    api: ApiVersion {
                        major: *version_parts.offset(1),
                        minor: *version_parts.offset(0)
                    },
                    firmware: firmware,
                    serial: read_part,
                    serial_string: sn,
                });

                Wrapper::hackrf_close(device);
            }
        }

        Wrapper::hackrf_device_list_free(raw_devices);

        res
    }
}

impl Drop for HackRF {
    fn drop(&mut self) {
        let _ = Wrapper::hackrf_exit();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiVersion {
    pub major: u8,
    pub minor: u8
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub index: i32,
    pub model: hackrf_sys::UsbBoardId,
    pub api: ApiVersion,
    pub firmware: String,
    pub serial: hackrf_sys::ReadPartIdSerialno,
    pub serial_string: String
}
