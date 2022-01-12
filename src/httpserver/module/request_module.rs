use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub(crate) username: String,
}
