use crate::schemas::{app::App, store::Store};
use rayon::prelude::*;
use std::{
    fs::{read_dir, File},
    io::{prelude::*, BufReader, Result},
    str,
};

pub fn file_reader(_path: &str) -> Store {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let store: Store = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    store
}

pub fn build_store_file(data_path: &str) -> Vec<App> {
    /*
     * Generate json description file
     */
    let paths = read_dir(data_path).unwrap();
    let mut all_apps: Vec<App> = Vec::new();

    for path in paths {
        let sub_path = &format!("{}", path.unwrap().path().display());
        let app_paths = read_dir(sub_path).unwrap();
        for app in app_paths {
            let file_path = format!("{}/desc", app.unwrap().path().display());
            let mut package_info: App = App::new();
            let app = package_info.import(file_path);
            all_apps.push(app);
        }
        // let file_path = &format!("{}/desc", path.unwrap().path().display());
        // let mut package_info: App = App::new();
        // let app = package_info.import(file_path);
        // all_apps.push(app);
    }

    all_apps
}

pub fn build_json_files(threaded: bool, data_path: &str) {
    /*
     * Generate json description file
     */
    let paths = read_dir(data_path).unwrap();

    match threaded {
        true => {
            /*
             * Concurrent
             */
            let mut all_file: Vec<String> = Vec::new();
            for path in paths {
                let file_path: String = format!("{}/desc", path.unwrap().path().display());
                all_file.push(file_path.clone());
            }

            all_file.par_iter().for_each(|n| {
                let mut package_info: App = App::new();
                match package_info.import(n.to_string()).export() {
                    Ok(()) => (),
                    Err(e) => println!("{}", e),
                }
            });
        }
        false => {
            /*
             * Non-concurrent
             */
            for path in paths {
                let file_path = format!("{}/desc", path.unwrap().path().display());
                let mut package_info: App = App::new();
                match package_info.import(file_path).export() {
                    Ok(()) => (),
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
}
