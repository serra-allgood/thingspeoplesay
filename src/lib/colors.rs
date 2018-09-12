extern crate crypto;

use self::crypto::{digest::Digest, md5::Md5};
use std::str::FromStr;

pub fn text_to_colors(text: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(text);
    let hashed = hasher.result_str();

    String::from_str(&hashed[0..18]).unwrap()
}
