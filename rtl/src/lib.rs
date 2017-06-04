#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate sdr_rtl_sys;
extern crate serde;
extern crate serde_json;

use serde::ser::{Serialize, Serializer, SerializeMap};

use std::ffi::{CStr};
use std::os::raw::{c_char, c_void};
use std::ptr;

pub mod command;
pub mod wrapper;

pub struct RTL {}

use self::wrapper::*;

impl RTL {

}

impl Drop for RTL {
    fn drop(&mut self) {

    }
}
