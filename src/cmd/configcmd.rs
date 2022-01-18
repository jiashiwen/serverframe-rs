use clap::{App, Arg};

pub fn new_config_cmd() -> App<'static> {
    clap::App::new("config")
        .about("config")
        .subcommand(config_show_cmd())
        .subcommand(config_generate_default())
}

fn config_show_cmd() -> App<'static> {
    clap::App::new("show")
        .about("show some info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_generate_default() -> App<'static> {
    clap::App::new("gendefault")
        .about("generate default config to file")
        .args(&[Arg::new("filepath").value_name("filepath").index(1)])
}

fn config_show_info_cmd() -> App<'static> {
    clap::App::new("info").about("show info")
}

fn config_show_all_cmd() -> App<'static> {
    clap::App::new("all").about("show all ")
}
