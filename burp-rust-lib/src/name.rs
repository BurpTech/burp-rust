use std::str::from_utf8;

use const_hex::encode_to_slice_upper;
use log::info;

const NAME_PREFIX: &'static str = "burp-";
const NAME_PREFIX_LEN: usize = NAME_PREFIX.as_bytes().len();
const NAME_SUFFIX_LENGTH: usize = 12;
const NAME_LENGTH: usize = NAME_PREFIX_LEN + NAME_SUFFIX_LENGTH;
static mut NAME: [u8; NAME_LENGTH] = [0_u8; NAME_LENGTH];

pub fn init_name(base_mac_address: &[u8; 6]) {
    unsafe {
        NAME[..NAME_PREFIX_LEN].copy_from_slice(NAME_PREFIX.as_bytes());
        encode_to_slice_upper(base_mac_address, &mut NAME[NAME_PREFIX_LEN..]).unwrap();
        info!("name is: {}", from_utf8(&NAME).unwrap());
    }
}

pub fn get_name() -> &'static str {
    unsafe { from_utf8(&NAME).unwrap() }
}
