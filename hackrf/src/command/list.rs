use super::super::*;

pub fn main() {
    let _hrf = match HackRF::init() {
        Ok(hrf) => hrf,
        _ => {
            return;
        }
    };

    let devices = HackRF::device_list();

    let serialized = serde_json::to_string_pretty(&devices).unwrap();

    println!("{}", serialized);
}
