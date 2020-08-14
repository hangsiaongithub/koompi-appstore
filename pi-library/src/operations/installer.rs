// use crate::{
//     config::config::get,
//     helpers::{download::download, extract::extract, file::file_writer},
//     schemas::store::Store,
// };
use crate::{
    structures::store::Store,
    utils::{config::get, download::download, extract::extract, file::file_writer},
};

use colored::Colorize;
use itertools::Itertools;

fn _flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

impl Store {
    pub fn install(&mut self, db: &Store, mut apps: Vec<&str>) {
        apps.sort();
        let mut apps_not_found: Vec<&str> = vec![];
        let mut apps_location: Vec<u32> = vec![];
        for app in apps.iter() {
            let res = db.search_rindex(app);
            if res == u32::max_value() {
                apps_not_found.push(app);
            } else {
                apps_location.push(res);
            }
        }
        #[allow(clippy::print_literal)]
        if !apps_not_found.is_empty() {
            println!(
                "{}\n- {}\n\n{}\n{}",
                "These apps are not found:".red().bold(),
                apps_not_found.join(" \n- ").bold().blink(),
                "The problem can either be:\n- misspelling or\n- those apps are not available in our palform for now.\n\nFeel free to reach out for help at:",
                "https://t.me/koompi".blue()
            );
        } else {
            let mut dep_list: Vec<String> = vec![];
            for app in apps_location.iter() {
                let app_deps = &db.apps[*app as usize].runtime_deps;
                if !app_deps.is_empty() && app_deps[0] != "none" {
                    for dep in app_deps.iter() {
                        dep_list.push(dep.clone());
                    }
                };
            }
            dep_list.sort();

            let clean_dep_list: Vec<_> = dep_list.into_iter().unique().collect();

            for dep in clean_dep_list.iter() {
                let res = db.search_rindex(dep);
                if res == u32::max_value() {
                    apps_not_found.push(dep);
                } else {
                    apps_location.push(res);
                }
            }

            let clean_app_list: Vec<_> = apps_location.into_iter().unique().collect();

            for app in clean_app_list.iter() {
                let src = &db.apps[*app as usize].tarball_src;
                let name = format!("{}{}", &db.apps[*app as usize].name, get().executable_ext);
                let dest = get().cache_directory;
                let file_path = format!("{}/{}", &dest, &name);
                match download(&file_path, &name, src) {
                    Ok(()) => (),
                    Err(e) => println!("{}", e),
                }
            }

            for app in clean_app_list.iter() {
                let path = get().cache_directory;
                let name = &format!("{}{}", &db.apps[*app as usize].name, get().executable_ext);
                let dest = get().installation_target;
                match extract(&path, name, &dest) {
                    Ok(()) => (),
                    Err(e) => println!("{}", e),
                }
            }

            for app in clean_app_list.iter() {
                if self.search_rbool(&db.apps[*app as usize].name) {
                    let target = self.search_rindex(&db.apps[*app as usize].name);
                    self.apps[target as usize] = db.apps[*app as usize].clone();
                } else {
                    self.apps.push(db.apps[*app as usize].clone());
                }
            }

            match file_writer(self.clone(), &get().registry) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
    }
}
