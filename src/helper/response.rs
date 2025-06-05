use serde::{Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ResponseJSON {
    pub message: String,
    pub values: Value,
}
