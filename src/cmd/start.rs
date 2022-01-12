use clap::App;

pub fn new_start_cmd() -> App<'static> {
    clap::App::new("start").about("start")
}
