use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: i32,
    pub msg: String,
}
