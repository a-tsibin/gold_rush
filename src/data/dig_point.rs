use crate::data::explore::Explore;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DigPoint {
    pub x: u32,
    pub y: u32,
    pub depth: u32,
    pub amount: u32,
}

impl DigPoint {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, amount: u32) -> DigPoint {
        DigPoint {
            x,
            y,
            depth: 1,
            amount,
        }
    }

    pub fn increase_depth(self) -> DigPoint {
        DigPoint {
            x: self.x,
            y: self.y,
            depth: self.depth + 1,
            amount: self.amount,
        }
    }
}

impl From<Explore> for DigPoint {
    fn from(e: Explore) -> Self {
        DigPoint {
            x: e.area.pos_x,
            y: e.area.pos_y,
            depth: 1,
            amount: e.amount,
        }
    }
}

impl Ord for DigPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl PartialOrd for DigPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
