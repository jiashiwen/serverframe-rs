use chrono::prelude::*;
use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn token_generator(username: String, password: String) -> String {
    let dt = Local::now();
    let seed = format!("{}{}{}", username, password, dt.timestamp_millis());
    let mut hasher = Sha256::new();
    hasher.input_str(seed.as_str());
    hasher.result_str()
}
