use crate::structures::{application::App, store::Store};

impl Store {
    // r... means return
    pub fn search_rbool(&self, name: &str) -> bool {
        let mut result: bool = false;
        for app in self.apps.iter() {
            if name == app.name {
                result = true;
            };
        }

        result
    }
    pub fn search_rindex(&self, name: &str) -> u32 {
        let mut location: u32 = u32::max_value();
        for (i, app) in self.apps.iter().enumerate() {
            if app.name == name {
                location = i as u32;
            }
        }
        location
    }

    pub fn search_rapp(&self, name: &str) -> Option<App> {
        let index = self.search_rindex(name);
        let mut app = None;
        if index < u32::max_value() {
            app = Some(self.apps[index as usize].clone());
        }
        app
    }
    // p... mean print
    pub fn search_papp(&self, name: &str) {
        let result = self.search_rindex(name);
        if result == u32::max_value() {
            println!("Not found.")
        } else {
            println!("{:#?}", self.apps[result as usize]);
        }
    }

    pub fn search_papps(&self, apps: Vec<&str>) {
        for app in apps.iter() {
            self.search_papp(app);
        }
    }
}
