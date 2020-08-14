use crate::schemas::{app::App, store::Store};
use pi_library::utils::download::download;

use rayon::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
impl Store {
    pub fn new(apps: Vec<App>) -> Store {
        Store { apps }
    }
    pub fn export(&self, target_path: &str) -> Result<()> {
        let output_file = &format!("{}/{}.json", target_path, "store");
        let mut f = File::create(output_file)?;
        f.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;

        f.sync_all()?;
        Ok(())
    }
    pub fn pull(&self, target_path: &str) {
        self.apps.iter().for_each(|app| {
            let address = &format!(
                "http://mirror.xtom.com.hk/archlinux/{}/os/{}/{}",
                &app.SUBREPO, &app.ARCH, &app.FILENAME
            );

            match std::fs::create_dir_all(&format!("{}/{}", target_path, &app.SUBREPO)) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }

            let file_path = &format!("{}/{}/{}", target_path, &app.SUBREPO, &app.FILENAME);
            match download(file_path, &app.NAME, address) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        })
    }
}
