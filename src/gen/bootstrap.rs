// DO NOT CHANGE THIS FILE. IT IS AUTOMATICALLY GENERATED BY build.rs.
use crate::decap;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DATA: HashMap<String, HashMap<String, String>> = {
        let data = include_bytes!("bootstrap.bin");
        decap(data)
    };
    pub static ref FILL: &'static HashMap<String, String> = DATA.get("fill").expect("fill");
    pub static ref NORMAL: &'static HashMap<String, String> = DATA.get("normal").expect("normal");
    pub static ref OUTLINE: &'static HashMap<String, String> =
        DATA.get("outline").expect("outline");
    pub static ref SHARP: &'static HashMap<String, String> = DATA.get("sharp").expect("sharp");
}
