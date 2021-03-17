use crate::data::dig_point::DigPoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dig {
    pub depth: u32,
    #[serde(rename(serialize = "licenseID"))]
    pub license_id: i32,
    #[serde(rename(serialize = "posX"))]
    pub pos_x: u32,
    #[serde(rename(serialize = "posY"))]
    pub pos_y: u32,
}

impl Dig {
    pub fn from_dig_point(point: DigPoint, license: &i32) -> Dig {
        Dig {
            depth: point.depth,
            license_id: *license,
            pos_x: point.x,
            pos_y: point.y,
        }
    }
}
