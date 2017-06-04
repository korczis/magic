extern crate sdr_hackrf_sys;

use std::os::raw::{c_char, c_int, c_void};

pub struct Wrapper {}

impl Wrapper {
    pub unsafe fn hackrf_init() -> sdr_hackrf_sys::Error {
        debug!("hackrf_init()");
        sdr_hackrf_sys::hackrf_init()
    }

    pub unsafe fn hackrf_board_partid_serialno_read(device: *const sdr_hackrf_sys::Device, serial: *mut sdr_hackrf_sys::ReadPartIdSerialno) -> sdr_hackrf_sys::Error {
        debug!("hackrf_board_partid_serialno_read({:?}, {:?})", device, serial);
        sdr_hackrf_sys::hackrf_board_partid_serialno_read(device, serial)
    }

    pub unsafe fn hackrf_close(device: *mut sdr_hackrf_sys::Device) -> sdr_hackrf_sys::Error {
        debug!("hackrf_close({:?})", device);
        sdr_hackrf_sys::hackrf_close(device)
    }

    pub unsafe fn hackrf_exit() -> sdr_hackrf_sys::Error {
        debug!("hackrf_exit()");
        sdr_hackrf_sys::hackrf_exit()
    }

    pub unsafe fn hackrf_device_list() -> *mut sdr_hackrf_sys::DeviceList {
        debug!("hackrf_device_list()");
        sdr_hackrf_sys::hackrf_device_list()
    }

    pub unsafe fn hackrf_device_list_free(list: *mut sdr_hackrf_sys::DeviceList) {
        debug!("hackrf_device_list_free({:?})", list);
        sdr_hackrf_sys::hackrf_device_list_free(list)
    }

    pub unsafe fn hackrf_device_list_open(list: *mut sdr_hackrf_sys::DeviceList, idx: c_int, device: *mut *mut sdr_hackrf_sys::Device) -> sdr_hackrf_sys::Error {
        debug!("hackrf_device_list_open({:?}, {:?}, {:?})", list, idx, device);
        sdr_hackrf_sys::hackrf_device_list_open(list, idx, device)
    }

    pub unsafe fn hackrf_reset(device: *const sdr_hackrf_sys::Device) -> sdr_hackrf_sys::Error {
        debug!("hackrf_reset({:?})", device);
        sdr_hackrf_sys::hackrf_reset(device)
    }

    pub unsafe fn hackrf_set_freq(device: *const sdr_hackrf_sys::Device, freq_hz: u64) -> sdr_hackrf_sys::Error {
        debug!("hackrf_set_freq({:?}, {:?})", device, freq_hz);
        sdr_hackrf_sys::hackrf_set_freq(device, freq_hz)
    }

    pub unsafe fn hackrf_set_sample_rate(device: *const sdr_hackrf_sys::Device, freq_hz: f64) -> sdr_hackrf_sys::Error {
        debug!("hackrf_set_sample_rate({:?}, {:?})", device, freq_hz);
        sdr_hackrf_sys::hackrf_set_sample_rate(device, freq_hz)
    }

    pub unsafe fn hackrf_start_rx(device: *mut sdr_hackrf_sys::Device, callback: sdr_hackrf_sys::SampleBlockCallback, rx_ctx: *mut c_void) -> sdr_hackrf_sys::Error {
        debug!("hackrf_start_tx({:?}, {:?}, {:?})", device, callback, rx_ctx);
        sdr_hackrf_sys::hackrf_start_rx(device, callback, rx_ctx)
    }

    pub unsafe fn hackrf_start_tx(device: *mut sdr_hackrf_sys::Device, callback: sdr_hackrf_sys::SampleBlockCallback, tx_ctx: *mut c_void) -> sdr_hackrf_sys::Error {
        debug!("hackrf_start_tx({:?}, {:?}, {:?})", device, callback, tx_ctx);
        sdr_hackrf_sys::hackrf_start_tx(device, callback, tx_ctx)
    }

    pub unsafe fn hackrf_stop_rx(device: *mut sdr_hackrf_sys::Device) -> sdr_hackrf_sys::Error {
        debug!("hackrf_stop_rx({:?})", device);
        sdr_hackrf_sys::hackrf_stop_rx(device)
    }

    pub unsafe fn hackrf_stop_tx(device: *mut sdr_hackrf_sys::Device) -> sdr_hackrf_sys::Error {
        debug!("hackrf_stop_tx({:?})", device);
        sdr_hackrf_sys::hackrf_stop_tx(device)
    }

    pub unsafe fn hackrf_usb_api_version_read(device: *const sdr_hackrf_sys::Device, version: *mut u16) -> sdr_hackrf_sys::Error {
        debug!("hackrf_usb_api_version_read({:?}, {:?})", device, version);
        sdr_hackrf_sys::hackrf_usb_api_version_read(device, version)
    }

    pub unsafe fn hackrf_version_string_read(device: *const sdr_hackrf_sys::Device, version: *mut c_char, length: u8) -> sdr_hackrf_sys::Error {
        debug!("hackrf_version_string_read({:?}, {:?}, {:?})", device, version, length);
        sdr_hackrf_sys::hackrf_version_string_read(device, version, length)
    }
}
