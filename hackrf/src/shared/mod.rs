use hackrf_sys;

pub fn get_device() -> Result<hackrf_sys::Device, hackrf_sys::Error> {
    Err(hackrf_sys::Error::Success)
}
