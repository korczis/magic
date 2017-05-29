#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hackrf_sys;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde::ser::{Serialize, Serializer, SerializeMap};

use std::ffi::{CStr};
use std::os::raw::{c_char};
use std::ptr;

pub mod command;
pub mod wrapper;

pub struct HackRF {}

use self::wrapper::*;

impl HackRF {
    pub fn init() -> Result<HackRF, hackrf_sys::Error> {
        unsafe {
            match Wrapper::hackrf_init() {
                hackrf_sys::Error::Success => {
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

#[derive(Serialize, Deserialize)]
pub struct ApiVersion {
    pub major: u8,
    pub minor: u8
}

pub struct Device {
    index: i32,
    model: hackrf_sys::UsbBoardId,
    serial_string: String,
    device: *mut hackrf_sys::Device
}

impl Device {
    pub unsafe fn open(list: *mut hackrf_sys::DeviceList, index: i32) -> Device {
        let mut device: *mut hackrf_sys::Device = ptr::null_mut();
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

    pub unsafe fn firmware(&self) -> String {
        let mut buffer: [c_char; 256] = [0; 256];
        Wrapper::hackrf_version_string_read(self.device, buffer.as_mut_ptr(), 255);
        CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()
    }

    pub unsafe fn serial(&self) -> hackrf_sys::ReadPartIdSerialno {
        let mut res = hackrf_sys::ReadPartIdSerialno {
            part_id: [0; 2],
            serial_no: [0; 4]
        };
        Wrapper::hackrf_board_partid_serialno_read(self.device, &mut res);
        res
    }

    pub fn version(&self) -> ApiVersion {
        unsafe {
            let mut version: u16 = 0;
            Wrapper::hackrf_usb_api_version_read(self.device, &mut version);
            let version_parts: *const u8 = &version as *const _ as *const u8;
            ApiVersion {
                major: *version_parts.offset(1),
                minor: *version_parts.offset(0)
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
            map.serialize_entry("version", &self.version()).unwrap();
            map.serialize_entry("firmware", &self.firmware()).unwrap();
            map.serialize_entry("serial", &self.serial()).unwrap();
            map.serialize_entry("serial_string", &self.serial_string).unwrap();

            map.end()
        }
    }
}

pub struct DeviceList {
    list: *mut hackrf_sys::DeviceList
}

impl DeviceList {
    pub fn from(list: *mut hackrf_sys::DeviceList) -> DeviceList {
      DeviceList {
          list: list
      }
    }

    pub unsafe fn as_raw(&mut self) -> *mut hackrf_sys::DeviceList {
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
            Wrapper::hackrf_device_list_free(self.list);
        }
    }
}
