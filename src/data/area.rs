use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Area {
    #[serde(rename(serialize = "posX", deserialize = "posX"))]
    pub pos_x: u32,
    #[serde(rename(serialize = "posY", deserialize = "posY"))]
    pub pos_y: u32,
    #[serde(rename(serialize = "sizeX", deserialize = "sizeX"))]
    pub size_x: u32,
    #[serde(rename(serialize = "sizeY", deserialize = "sizeY"))]
    pub size_y: u32,
}

impl Area {
    pub fn new(pos_x: u32, pos_y: u32, size_x: u32, size_y: u32) -> Area {
        Area {
            pos_x,
            pos_y,
            size_x,
            size_y,
        }
    }
}

impl Ord for Area {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .size_x
            .cmp(&self.size_x)
            .then_with(|| other.size_y.cmp(&self.size_y))
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(u32, u32)> for Area {
    fn from(p: (u32, u32)) -> Self {
        Area {
            pos_x: p.0,
            pos_y: p.1,
            size_x: 1,
            size_y: 1,
        }
    }
}
