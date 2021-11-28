use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::util::mine_block;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);

        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }
}
