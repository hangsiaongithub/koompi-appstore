use clap::{App, Arg};

// This function does:
// Declare argument list and specs
// Return a matched arguement

pub fn command_line_interface<'a, 'b>() -> App<'a, 'b> {
    App::new("pi")
        .version("0.0.1 alpha")
        .arg(
            Arg::with_name("install")
                .help("install applications")
                .short("i")
                .long("install")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("update")
                .help("update applications")
                .short("u")
                .long("update"),
        )
        .arg(
            Arg::with_name("remove")
                .help("remove applications")
                .short("r")
                .long("remove")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("search")
                .help("search applications")
                .short("s")
                .long("search")
                .takes_value(true)
                .multiple(true),
        )
}
