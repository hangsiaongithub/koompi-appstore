extern crate pi_library;

// #![allow(unused_imports, unused_assignments, unused_variables, unused_mut)]
mod cli_args;
mod operations;
// pub mod config;
pub mod database;
// pub mod helpers;
// pub mod interfaces;

// pub mod schemas;

use cli_args::command_line_interface;
use database::{db, sync};
use operations::Operation;
use pi_library::{structures::configuration::Config, utils::config::get};
// use schemas::config::Config;

fn main() {
    let configuration: Config = get();
    sync::sync();
    let mut local_db = db::init(&configuration.local_db);
    let mut registry = db::init(&configuration.registry);

    // Empty vectore for storing application list from user input
    let mut apps: Vec<&str> = Vec::new();
    // All arguement interfaces declared using CLAP
    let pi = command_line_interface();
    let matches = pi.clone().get_matches();

    // Check for given operation variants from user input
    let op = if matches.is_present("install") {
        Operation::Install
    } else if matches.is_present("update") {
        Operation::Update
    } else if matches.is_present("remove") {
        Operation::Remove
    } else if matches.is_present("search") {
        Operation::Search
    } else {
        Operation::Help
    };

    // Bind each operation variants to operation functions
    match op {
        Operation::Install => {
            let args_list = matches.values_of("install").unwrap().collect::<Vec<_>>();
            for arg in args_list.iter() {
                apps.push(*arg);
            }
            registry.install(&local_db, apps);
        }
        Operation::Update => {
            registry.update(&mut local_db);
        }
        Operation::Remove => {
            let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
            for arg in args_list.iter() {
                apps.push(*arg);
            }
            registry.remove(apps);
        }
        Operation::Search => {
            let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
            for arg in args_list.iter() {
                apps.push(*arg);
            }
            registry.search_papps(apps);
        }
        _ => {
            let helper = pi.clone().print_help();
            helper.unwrap();
            println!();
        }
    }
}
