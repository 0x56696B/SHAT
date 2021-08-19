use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

pub fn hash_pass(raw_pass: String) -> String {
    todo!("Implement hashing of passwords")
}
