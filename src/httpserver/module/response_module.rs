use crate::httpserver::exception::Response_Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
