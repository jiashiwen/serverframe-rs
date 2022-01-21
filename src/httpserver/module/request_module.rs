use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub(crate) username: String,
}
