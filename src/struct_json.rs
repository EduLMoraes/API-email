use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub address: String,
    pub valid: bool,
}