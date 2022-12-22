use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref L_S_INVALID: Mutex<String> = Mutex::new(String::from("Invalid"));
    pub static ref L_S_FISHY: Mutex<String> = Mutex::new(String::from("Fishy"));
    pub static ref L_S_VALID: Mutex<String> = Mutex::new(String::from("Valid"));
}
