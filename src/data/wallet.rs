use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub wallet: Vec<u32>,
}
