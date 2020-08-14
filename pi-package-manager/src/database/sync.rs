use colored::Colorize;
use pi_library::{
    structures::{configuration::Config, version::Version},
    utils::{
        compare::compare, config::get, download::download, file::version_reader, rest::http_get,
    },
};

fn cloud_version(url: &str) -> Version {
    let v_string = http_get(url);
    let v: Version = serde_json::from_str(&v_string).expect("Error reading version");

    v
}

fn disk_version(path: &str) -> Version {
    version_reader(path)
}

pub fn sync() {
    let configurations: Config = get();
    if !configurations.production {
        println!(
            "{}\n{}",
            "WARNING:".yellow().bold(),
            "This software is running development mode.".yellow()
        );
    };
    let repo_address: String = configurations.repos[0].address.clone();
    let db_address = format!("{}/db/db.json", &repo_address);
    let version_address = format!("{}/db/version", &repo_address);
    let server_version: Version = cloud_version(&version_address);
    let local_version: Version = disk_version(&get().version_file);
    let cmp_result: i8 = compare(local_version.version, server_version.version);
    println!("{}", "AppStore is checking for database updates...".blue());
    if cmp_result != 0 {
        println!("Downloading database updates...");
        match download("root/store/db", "store.json", &db_address) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("{}", "AppStore database is up to date.".green());
    };
}
