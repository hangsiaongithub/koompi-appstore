use crate::structures::store::Store;
use colored::Colorize;
impl Store {
    pub fn update(&mut self, local_db: &mut Store) {
        println!("{}", "Checking for application updates...".yellow());
        let mut installed_apps: Vec<String> = Vec::new();
        let mut update_target: Vec<&str> = Vec::new();

        for app in self.apps.iter_mut() {
            installed_apps.push(app.name.clone());
        }

        for app in installed_apps.iter() {
            let local_app = &mut self.search_rapp(&app).unwrap().build_date;
            let server_app = &mut local_db.search_rapp(&app).unwrap().build_date;
            if local_app < server_app {
                update_target.push(app);
            }
        }
        if !update_target.is_empty() {
            println!(
                "{} {}",
                format!("{}", update_target.len()).yellow(),
                "Updates found:".yellow()
            );
            println!("Downloading updates...");
            self.install(&*local_db, update_target);
            println!("{}", "Update completed successfully".green());
        } else {
            println!(
                "{}",
                "No updates availble.\nYou are using the latest software available.".green()
            );
        }
    }
}
