mod init_resources;
mod tikv;

pub use init_resources::get_tikv_handler;
// pub use init_resources::init_tikv;
pub use init_resources::set_tikv;
pub use tikv::TiKVHandler;
