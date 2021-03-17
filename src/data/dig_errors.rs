use reqwest::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DigError {
    #[error("Treasure not found")]
    TreasureNotFound,
    #[error("Invalid license")]
    InvalidLicense,
    #[error("Unknown error")]
    Unknown(#[from] Error)
}