extern crate pi_library;
use crate::pi_library::{
    structures::xml::{Application, Store},
    utils::file::{xml_file_reader, xml_file_writer},
};
use std::fs;
use treexml::Document;

fn main() {
    let mut db_files: Vec<String> = Vec::new();
    let working_path = "/home/brilliant/Documents/projects/pkg-dist/development/discover/input";
    let output_path = "/home/brilliant/Documents/projects/pkg-dist/development/discover/output";
    let paths = fs::read_dir(working_path).unwrap();
    for path in paths {
        let file_path: String = format!("{}", path.unwrap().path().display());

        db_files.push(file_path);
    }

    for file in db_files.iter() {
        println!("Reading: {}", file);
        let mut my_store: Store = Store::default();
        let splitted_path: Vec<&str> = file.split('/').collect();
        let file_name = splitted_path.last().clone().unwrap();
        let name: Vec<&str> = file_name.split('.').collect();
        let output_file_name = &format!("{}/{}.json", output_path, name.first().unwrap());

        let data = xml_file_reader(file);
        let doc = Document::parse(data.as_bytes()).unwrap();
        let root = doc.root.unwrap();

        for app in root.children.iter() {
            let new_app: Application = Application::new(app.clone());
            // &my_store.applications.push(new_app);
            my_store.applications.push(new_app);
        }

        println!("{}", output_file_name);
        xml_file_writer(my_store, output_file_name, true).unwrap();
    }
}
