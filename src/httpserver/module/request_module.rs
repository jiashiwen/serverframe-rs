use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Deserialize)]
pub struct CreateUser {
    pub(crate) username: String,
}

#[derive(EnumString, Display, Debug, PartialEq, Deserialize)]
pub enum Option {
    Put,
    Del,
    Get,
}
