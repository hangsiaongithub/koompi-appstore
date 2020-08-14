use crate::structures::configuration::Config;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn get() -> Config {
    let file = File::open("root/etc/pi-conf.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let configurations: Config =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    configurations
}
