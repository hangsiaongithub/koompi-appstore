use crate::structures::{store, version, xml};
use std::{
    fs::File,
    io::{prelude::*, BufReader, Result},
    str,
};

pub fn file_reader(_path: &str) -> store::Store {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let store: store::Store = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    store
}

pub fn file_writer(data_src: store::Store, _path: &str) -> Result<()> {
    let mut f = File::create(_path)?;
    f.write_all(serde_json::to_string_pretty(&data_src)?.as_bytes())?;

    f.sync_all()?;
    Ok(())
}

pub fn version_reader(_path: &str) -> version::Version {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let v: version::Version = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    v
}

pub fn xml_file_reader(_path: &str) -> String {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}

pub fn xml_file_writer(data_src: xml::Store, _path: &str, pretty: bool) -> Result<()> {
    let mut f = File::create(_path)?;
    match pretty {
        true => f.write_all(serde_json::to_string_pretty(&data_src)?.as_bytes())?,
        false => f.write_all(serde_json::to_string(&data_src)?.as_bytes())?,
    }

    f.sync_all()?;
    Ok(())
}
