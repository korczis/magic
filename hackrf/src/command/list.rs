use super::super::*;

pub fn main() {
    let _hrf = match HackRF::init() {
        Ok(hrf) => hrf,
        Err(err) => {
            println!("Unable to init HackRF, reason: {:?}", err);
            return;
        }
    };

    let devices = HackRF::device_list();

    let serialized = serde_json::to_string_pretty(&devices).unwrap();

    println!("{}", serialized);
}
