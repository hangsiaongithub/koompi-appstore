use std::io::prelude::*;
use std::io::stderr;
use std::process::Command;

pub fn extractor(data_path: &str, output_dir: &str) {
    println!("Extracting: {}", &data_path);
    let unzip = Command::new("tar")
        .arg("-xf")
        .arg(data_path)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to execute process");
    match stderr().write_all(&unzip.stderr) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}
