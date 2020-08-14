#![allow(
    non_snake_case,
    unused_imports,
    unused_assignments,
    unused_variables,
    dead_code
)]
extern crate pi_library;

pub mod helpers;
pub mod interfaces;
pub mod schemas;

use helpers::{
    // download::download,
    extractor::extractor,
    file::{build_json_files, build_store_file, file_reader},
};
use pi_library::utils::download::download;
use rayon::prelude::*;
use schemas::{app::App, store::Store};
use std::{
    fs,
    io::{stderr, Write},
    process::Command,
    thread,
    time::{Duration, Instant},
};

fn main() {
    let start = Instant::now();
    let mut db_files: Vec<String> = Vec::new();
    let paths = fs::read_dir("/var/lib/pacman/sync/").unwrap();
    let input_dir = "/home/brilliant/Documents/projects/pkg-dist/development/pacman/input";
    let output_dir = "/home/brilliant/Documents/projects/pkg-dist/development/pacman/output";
    for path in paths {
        let file_path: String = format!("{}", path.unwrap().path().display());

        db_files.push(file_path);
    }

    db_files.par_iter().for_each(|db| {
        let repo_ext: Vec<&str> = db.split('/').collect();
        let repo_name: Vec<&str> = repo_ext.last().unwrap().split('.').collect();
        let repo_dir = repo_name.first().unwrap();
        match fs::create_dir_all(&format!("{}/{}", input_dir, repo_dir)) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
        extractor(db, &format!("{}/{}", input_dir, repo_dir));
    });

    // build individual json file
    // build_json_files(true, "work");

    // build store file
    let apps = build_store_file(input_dir);
    let app_store = Store::new(apps);

    // println!("{:#?}", app_store);
    match app_store.export(output_dir) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
    app_store.pull(output_dir);

    // let local_store: Store = file_reader("dist/store.json");
    // local_store.apps.par_iter().for_each(|app| {
    //     if download("", "", &app.NAME, true) {
    //         print!("");
    //     }
    // });
    // println!("{:?}", local_store.apps.len());
    // set stop time
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
