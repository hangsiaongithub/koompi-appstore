use colored::Colorize;
use std::fs::read_dir;
use std::io::prelude::*;
use std::io::stderr;
use std::process::Command;

fn main() {
    let mut files: Vec<String> = Vec::new();
    let paths =
        read_dir("/home/brilliant/Documents/projects/pkg-dist/development/pacman/output/multilib")
            .unwrap();

    for path in paths {
        files.push(path.unwrap().path().display().to_string())
    }

    for file in files.iter() {
        println!("{}", file);
        extractor(
            &file,
            "/home/brilliant/Documents/projects/pkg-dist/development/builder/jail",
        );
        let path_splitted: Vec<&str> = file.split('/').collect();
        let file_name = path_splitted.last().clone().unwrap();
        let name_splitted: Vec<&str> = file_name.split('.').collect();
        let name = name_splitted.first().clone().unwrap();
        let output_file = &format!(
            "/home/brilliant/Documents/projects/pkg-dist/development/builder/output/{}.app",
            name
        );
        compressor(
            output_file,
            "/home/brilliant/Documents/projects/pkg-dist/development/builder/jail",
        );

        std::fs::remove_dir_all(
            "/home/brilliant/Documents/projects/pkg-dist/development/builder/jail",
        )
        .unwrap();
        std::fs::create_dir_all(
            "/home/brilliant/Documents/projects/pkg-dist/development/builder/jail",
        )
        .unwrap();
    }
}

fn compressor(output_file: &str, src_dir: &str) {
    println!("Converting: {}", output_file.yellow());
    let compress = Command::new("tar")
        .arg("-czf")
        .arg(output_file)
        .arg(src_dir)
        .stdout(std::process::Stdio::null())
        .output()
        .expect("failed to execute process");
    match stderr().write_all(&compress.stdout) {
        Ok(()) => println!("{}", "Convertion completed.".green()),
        Err(e) => eprintln!("Error: {}", e.to_string().red()),
    }
}

fn extractor(data_path: &str, output_dir: &str) {
    println!("Extracting: {}", &data_path.yellow());
    let unzip = Command::new("tar")
        .arg("-xf")
        .arg(data_path)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to execute process");
    let unzst = Command::new("tar")
        .arg("-I")
        .arg("zstd")
        .arg("-xf")
        .arg(data_path)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to execute process");
    match stderr().write_all(&unzip.stderr) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }

    let exts: Vec<&str> = data_path.split('.').collect();
    let ext = *exts.last().clone().unwrap();
    match ext {
        "zst" => match stderr().write_all(&unzst.stderr) {
            Ok(()) => (),
            Err(e) => println!("{}", e.to_string().red()),
        },
        "xz" => match stderr().write_all(&unzip.stderr) {
            Ok(()) => (),
            Err(e) => println!("{}", e.to_string().red()),
        },
        _ => {
            eprintln!("{}", "Unsupported file type.".red());
        }
    }
}
