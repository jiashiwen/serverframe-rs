mod embed_resources;
mod init_resources;
mod tikv;

pub use embed_resources::get_rbac_model;
pub use embed_resources::get_rbac_policy;
pub use init_resources::get_tikv_handler;
pub use init_resources::init_resources;
pub use init_resources::set_tikv;
pub use tikv::TiKVHandler;
