use crate::data::area::Area;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Explore {
    pub area: Area,
    pub amount: u32,
}
