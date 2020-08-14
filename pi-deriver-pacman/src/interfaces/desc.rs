use crate::schemas::app::App;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn import(&mut self, src_file: String) -> Self {
        let nsrc = src_file.clone();
        let repo_name: Vec<&str> = nsrc.split('/').collect();
        // println!("{:?}", repo_name);
        let file = File::open(src_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let mut lines: Vec<&str> = contents.split("\n\n").collect();

        for line in lines.iter_mut() {
            let splitted: Vec<&str> = line.split('%').collect();
            for (i, c) in splitted.iter().enumerate() {
                let info: Vec<&str> = c.split('\n').collect();
                if i > 0 && info.len() == 1 {
                    match splitted[i] {
                        "FILENAME" => {
                            let FILENAME = splitted[i + 1].to_string();
                            let data: Vec<&str> = FILENAME.split('\n').collect();
                            self.FILENAME = data[1].to_string();
                        }
                        "NAME" => {
                            let NAME = splitted[i + 1].to_string();
                            let data: Vec<&str> = NAME.split('\n').collect();
                            self.NAME = data[1].to_string();
                        }
                        "BASE" => {
                            let BASE = splitted[i + 1].to_string();
                            let data: Vec<&str> = BASE.split('\n').collect();
                            self.BASE = data[1].to_string();
                        }
                        "VERSION" => {
                            let VERSION = splitted[i + 1].to_string();
                            let data: Vec<&str> = VERSION.split('\n').collect();
                            self.VERSION = data[1].to_string();
                        }
                        "DESC" => {
                            let DESC = splitted[i + 1].to_string();
                            let data: Vec<&str> = DESC.split('\n').collect();
                            self.DESC = data[1].to_string();
                        }
                        "CSIZE" => {
                            let CSIZE = splitted[i + 1].to_string();
                            let data: Vec<&str> = CSIZE.split('\n').collect();
                            self.CSIZE = data[1].to_string();
                        }
                        "ISIZE" => {
                            let ISIZE = splitted[i + 1].to_string();
                            let data: Vec<&str> = ISIZE.split('\n').collect();
                            self.ISIZE = data[1].to_string();
                        }
                        "MD5SUM" => {
                            let MD5SUM = splitted[i + 1].to_string();
                            let data: Vec<&str> = MD5SUM.split('\n').collect();
                            self.MD5SUM = data[1].to_string();
                        }
                        "SHA256SUM" => {
                            let SHA256SUM = splitted[i + 1].to_string();
                            let data: Vec<&str> = SHA256SUM.split('\n').collect();
                            self.SHA256SUM = data[1].to_string();
                        }
                        "PGPSIG" => {
                            let PGPSIG = splitted[i + 1].to_string();
                            let data: Vec<&str> = PGPSIG.split('\n').collect();
                            self.PGPSIG = data[1].to_string();
                        }
                        "URL" => {
                            let URL = splitted[i + 1].to_string();
                            let data: Vec<&str> = URL.split('\n').collect();
                            self.URL = data[1].to_string();
                        }
                        "LICENSE" => {
                            let LICENSE = splitted[i + 1].to_string();
                            let data: Vec<&str> = LICENSE.split('\n').collect();
                            self.LICENSE = data[1].to_string();
                        }
                        "ARCH" => {
                            let ARCH = splitted[i + 1].to_string();
                            let data: Vec<&str> = ARCH.split('\n').collect();
                            self.ARCH = data[1].to_string();
                        }
                        "BUILDDATE" => {
                            let BUILDDATE = splitted[i + 1].to_string();
                            let data: Vec<&str> = BUILDDATE.split('\n').collect();
                            self.BUILDDATE = data[1].to_string();
                        }
                        "PACKAGER" => {
                            let PACKAGER = splitted[i + 1].to_string();
                            let data: Vec<&str> = PACKAGER.split('\n').collect();
                            self.PACKAGER = data[1].to_string();
                        }
                        "PROVIDES" => {
                            let PROVIDES = vec![splitted[i + 1].to_string()];
                            let data: Vec<&str> = PROVIDES[0].split('\n').collect();
                            let mut new_data: Vec<String> = Vec::new();
                            for i in data.iter() {
                                if *i != "" {
                                    new_data.push(i.to_string())
                                }
                            }
                            self.PROVIDES = new_data;
                        }
                        "DEPENDS" => {
                            let DEPENDS = vec![splitted[i + 1].to_string()];
                            let data: Vec<&str> = DEPENDS[0].split('\n').collect();
                            let mut new_data: Vec<String> = Vec::new();
                            for i in data.iter() {
                                if *i != "" {
                                    new_data.push(i.to_string())
                                }
                            }
                            self.DEPENDS = new_data;
                        }
                        &_ => continue,
                    }
                    let repo_number: usize = repo_name.len() - 3;
                    self.SUBREPO = repo_name[repo_number].to_string();
                }
            }
        }
        self.clone()
    }

    pub fn export(&self) -> Result<()> {
        let output_file = &format!("./dist/{}.json", &self.NAME);
        let mut f = File::create(output_file)?;
        f.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;

        f.sync_all()?;
        Ok(())
    }
}
