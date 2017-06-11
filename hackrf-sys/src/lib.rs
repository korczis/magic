extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::os::raw::{c_void, c_int, c_uint, c_char, c_uchar, c_double};

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Error {
    Success = 0,
    True = 1,
    InvalidParam = -2,
    NotFound = -5,
    Busy = -6,
    NoMem = -11,
    Libusb = -1000,
    Thread = -1001,
    StreamingThreadErr = -1002,
    StreamingStopped = -1003,
    StreamingExitCalled = -1004,
    UsbApiVersion = -1005,
    Other = -9999,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum BoardId {
    Jellybean = 0,
    Jawbreaker = 1,
    One = 2,
    Rad10 = 3,
    Invalid = 255,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum UsbBoardId {
    Jawbreaker = 24651,
    One = 24713,
    Rad10 = 52245,
    Invalid = 65535,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum RfPathFilter {
    Bypass = 0,
    Low = 1,
    High = 2,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum TransceiverMode {
    Off = 0,
    Rx = 1,
    Tx = 2,
    Sf = 3,
    Cpld = 4,
}

pub enum Device { }

#[derive(Debug)]
#[repr(C)]
pub struct Transfer {
    pub device: *mut Device,
    pub buffer: *mut u8,
    pub buffer_length: c_int,
    pub valid_length: c_int,
    pub rx_ctx: *mut c_void,
    pub tx_ctx: *mut c_void,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct ReadPartIdSerialno {
    pub part_id: [u32; 2],
    pub serial_no: [u32; 4],
}

#[derive(Debug)]
#[repr(C)]
pub struct DeviceList {
    pub serial_numbers: *mut *mut c_char,
    pub usb_board_ids: *mut UsbBoardId,
    pub usb_device_index: *mut c_int,
    pub devicecount: c_int,
    pub usb_devices: *mut *mut c_void,
    pub usb_devicecount: c_int,
}

pub type SampleBlockCallback = unsafe extern "C" fn(transfer: *mut Transfer) -> c_int;

#[link(name = "hackrf")]
extern "C" {
    pub fn hackrf_init() -> Error;
    pub fn hackrf_exit() -> Error;

    pub fn hackrf_board_partid_serialno_read(device: *const Device, serial: *mut ReadPartIdSerialno) -> Error;

    pub fn hackrf_close(device: *mut Device) -> Error;

    pub fn hackrf_device_list() -> *mut DeviceList;
    pub fn hackrf_device_list_free(list: *mut DeviceList);
    pub fn hackrf_device_list_open(list: *mut DeviceList, idx: c_int, device: *mut *mut Device) -> Error;

    pub fn hackrf_reset(device: *const Device) -> Error;

    pub fn hackrf_set_freq(device: *const Device, freq_hz: u64) -> Error;
    pub fn hackrf_set_sample_rate(device: *const Device, freq_hz: f64) -> Error;

    pub fn hackrf_start_rx(device: *mut Device, callback: SampleBlockCallback, rx_ctx: *mut c_void) -> Error;
    pub fn hackrf_stop_rx(device: *mut Device) -> Error;

    pub fn hackrf_start_tx(device: *mut Device, callback: SampleBlockCallback, tx_ctx: *mut c_void) -> Error;
    pub fn hackrf_stop_tx(device: *mut Device) -> Error;

    pub fn hackrf_usb_api_version_read(device: *const Device, version: *mut u16) -> Error;
    pub fn hackrf_version_string_read(device: *const Device, version: *mut c_char, length: u8) -> Error;
}
