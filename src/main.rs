use logger::init_log;

mod cmd;
mod commons;
mod configure;
mod errors;
mod httpserver;
mod interact;
mod logger;
mod resources;
// mod request;

fn main() {
    init_log();
    cmd::run_app();
}
