use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response_Error {
    error: Error_msg,
}

impl Response_Error {
    pub fn default() -> Self {
        Self {
            error: Error_msg::new(),
        }
    }
    pub fn new(err: Error_msg) -> Self {
        Self { error: err }
    }
    pub fn set_error(&mut self, error: Error_msg) {
        self.error = error;
    }
}

#[derive(Serialize)]
pub struct Error_msg {
    error_code: u64,
    msg: String,
}

impl Error_msg {
    pub fn new() -> Self {
        Self {
            error_code: 0,
            msg: "".to_string(),
        }
    }
    pub fn set_error_code(&mut self, error_code: u64) {
        self.error_code = error_code;
    }
    pub fn set_msg(&mut self, msg: String) {
        self.msg = msg;
    }
}
