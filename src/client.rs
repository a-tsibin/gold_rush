use crate::data::area::Area;
use crate::data::dig::Dig;
use crate::data::dig_errors::DigError;
use crate::data::explore::Explore;
use crate::data::license::License;
use log::info;
use reqwest::StatusCode;
use tokio::time::Duration;

#[derive(Clone)]
pub struct Client {
    url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String, timeout: Duration) -> Client {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .connect_timeout(timeout)
            .build()
            .unwrap(); //todo
        Client { url, client }
    }

    pub async fn request_license(&self, coins: Vec<u32>) -> reqwest::Result<License> {
        info!("Requesting license");
        let resp: reqwest::Result<License> = self
            .client
            .post(format!("{}/licenses", &self.url))
            .header("Content-Type", "application/json")
            .body(reqwest::Body::from(format!("{:?}", coins)))
            .send()
            .await?
            .json()
            .await;
        info!("Request license response: {:?}", resp);
        resp
    }

    pub async fn dig(&self, action: Dig) -> Result<Vec<String>, DigError> {
        info!("Digging at {:?}", action);
        let resp = self
            .client
            .post(format!("{}/dig", &self.url))
            .json(&action)
            .send()
            .await.map_err(|e| DigError::Unknown(e))?;
        info!("Response from dig {:?}", resp);
        match resp.status() {
            StatusCode::NOT_FOUND => Err(DigError::TreasureNotFound),
            StatusCode::FORBIDDEN => Err(DigError::InvalidLicense),
            _ => resp.json().await.map_err(|e| DigError::Unknown(e)),
        }
    }

    pub async fn cash(&self, treasure: &String) -> reqwest::Result<Vec<u32>> {
        info!("Cashing treasure {:?}", treasure);
        let resp: reqwest::Result<Vec<u32>> = self
            .client
            .post(format!("{}/cash", &self.url))
            .header("Content-Type", "application/json")
            .json(&treasure)
            .send()
            .await?
            .json()
            .await;
        info!("Response from cash {:?}", resp);
        resp
    }

    pub async fn explore(&self, area: &Area) -> reqwest::Result<Explore> {
        info!("Exploring area {:?}", area);
        let resp: reqwest::Result<Explore> = self
            .client
            .post(format!("{}/explore", &self.url))
            .json(area)
            .send()
            .await?
            .json()
            .await;
        info!("Explore result: {:?}", resp);
        resp
    }
}
