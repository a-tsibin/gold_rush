use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct License {
    pub id: i32,
    #[serde(rename(serialize = "digAllowed", deserialize = "digAllowed"))]
    pub dig_allowed: u32,
    #[serde(rename(serialize = "digUsed", deserialize = "digUsed"))]
    pub dig_used: u32,
}

impl License {
    pub fn decrease_dig_allowed(&self) -> Option<Self> {
        if self.dig_allowed > 1 {
            Some(Self {
                id: self.id,
                dig_allowed: self.dig_allowed - 1,
                dig_used: self.dig_used + 1,
            })
        } else {
            None
        }
    }
}
