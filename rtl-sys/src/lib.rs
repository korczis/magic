use std::os::raw::{c_char};

pub enum rtlsdr_dev_t {}

#[derive(Debug)]
#[repr(C)]
pub enum TunerType {
    Unknown = 0,
    E4000,
    FC0012,
    FC0013,
    FC2580,
    R820T,
    R828D
}

#[derive(Debug)]
#[repr(C)]
pub enum DirectSampling {
    Disabled = 0,
    I,
    Q
}

impl DirectSampling {
    pub fn from(val: i32) -> Option<DirectSampling> {
        match val {
            0 => Some(DirectSampling::Disabled),
            1 => Some(DirectSampling::I),
            2 => Some(DirectSampling::Q),
            _ => None
        }
    }
}

#[link(name = "rtlsdr")]
extern "C" {
    pub fn rtlsdr_get_device_count() -> u32;
    pub fn rtlsdr_get_device_name(index: u32) -> *const c_char;
    pub fn rtlsdr_get_device_usb_strings(index: u32, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32;
    pub fn rtlsdr_get_index_by_serial(serial: *const c_char) -> i32;

    pub fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: u32) -> i32;
    pub fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> i32;

    pub fn rtlsdr_set_xtal_freq(dev: *mut rtlsdr_dev_t, rtl_freq: u32, tuner_freq: u32) -> i32;
    pub fn rtlsdr_get_xtal_freq(dev: *mut rtlsdr_dev_t, rtl_freq: *mut u32, tuner_freq: *mut u32) -> i32;

    pub fn rtlsdr_get_usb_strings(dev: *mut rtlsdr_dev_t, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32;

//    pub fn rtlsdr_write_eeprom(dev: *mut rtlsdr_dev_t, data: *mut u8, offset: *mut u8, len: u16) -> i32;
//    pub fn rtlsdr_read_eeprom(dev: *mut rtlsdr_dev_t, data: *mut u8, offset: *mut u8, len: u16) -> i32;

    pub fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq: u32) -> i32;
    pub fn rtlsdr_get_center_freq(dev: *mut rtlsdr_dev_t) -> u32;

    pub fn rtlsdr_set_freq_correction(dev: *mut rtlsdr_dev_t, ppm: i32) -> i32;
    pub fn rtlsdr_get_freq_correction(dev: *mut rtlsdr_dev_t) -> i32;

    pub fn rtlsdr_get_tuner_type(dev: *mut rtlsdr_dev_t) -> TunerType;

    pub fn rtlsdr_get_tuner_gains(dev: *mut rtlsdr_dev_t, gains: *mut i32) -> i32;
    pub fn rtlsdr_set_tuner_gain(dev: *mut rtlsdr_dev_t, gain: i32) -> i32;

    pub fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev_t) -> i32;
    pub fn rtlsdr_set_tuner_if_gain(dev: *mut rtlsdr_dev_t, stage: i32, gain: i32) -> i32;
    pub fn rtlsdr_set_tuner_gain_mode(dev: *mut rtlsdr_dev_t, manual: i32) -> i32;

    pub fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, rate: u32) -> i32;
    pub fn rtlsdr_get_sample_rate(dev: *mut rtlsdr_dev_t) -> u32;

    pub fn rtlsdr_set_testmode(dev: *mut rtlsdr_dev_t, on: i32) -> i32;
    pub fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev_t, on: i32) -> i32;
    pub fn rtlsdr_set_direct_sampling(dev: *mut rtlsdr_dev_t, on: i32) -> i32;
    pub fn rtlsdr_get_direct_sampling(dev: *mut rtlsdr_dev_t) -> i32;
    pub fn rtlsdr_set_offset_tuning(dev: *mut rtlsdr_dev_t, on: i32) -> i32;
    pub fn rtlsdr_get_offset_tuning(dev: *mut rtlsdr_dev_t) -> i32;

    pub fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> i32;
    pub fn rtlsdr_read_sync(dev: *mut rtlsdr_dev_t, buf: *mut c_char, len: i32, n_read: *mut i32) -> i32;

    pub fn rtlsdr_cancel_async(dev: *mut rtlsdr_dev_t) -> i32;
}
