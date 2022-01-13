use clap::App;

pub fn new_stop_cmd() -> App<'static> {
    clap::App::new("stop").about("stop")
}
