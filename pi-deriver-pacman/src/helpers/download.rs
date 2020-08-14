use std::fs::{create_dir_all, read_dir};
use std::io::{stderr, Write};
use std::path::Path;
use std::process::Command;

fn download_file(address: &str, file: &str, resume: bool) -> u8 {
    let download_resume = Command::new("curl")
        .arg("-#")
        .arg(address)
        .arg("-o")
        .arg(file)
        .arg("-C")
        .arg("-")
        .env("PATH", "/bin")
        .env("COLUMNS", "80")
        .output()
        .expect("failed to execute process");

    let download_no_resume = Command::new("curl")
        .arg("-#")
        .arg(address)
        .arg("-o")
        .arg(file)
        .env("PATH", "/bin")
        .env("COLUMNS", "80")
        .output()
        .expect("failed to execute process");

    if resume {
        match stderr().write_all(&download_resume.stderr) {
            Ok(()) => 0,
            Err(err) => {
                println!("{:?}", err);
                1
            }
        }
    } else {
        match stderr().write_all(&download_no_resume.stderr) {
            Ok(()) => 0,
            Err(err) => {
                println!("{:?}", err);
                1
            }
        }
    }
}

pub fn download(address: &str, output_dir: &str, file_name: &str, resume: bool) -> bool {
    let dir_exist = Path::new(output_dir).exists();
    if !dir_exist {
        match std::fs::create_dir_all(output_dir) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
    }
    let f_name = &format!("{}/{}", output_dir, file_name);
    let exit_code = download_file(address, f_name, resume);

    match exit_code {
        0 => true,
        _ => false,
    }
}

pub fn exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}
