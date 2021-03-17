use crate::client::Client;
use crate::data::license::License;
use crossbeam_deque::Injector;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::RateLimiter;
use log::{error, info};
use std::sync::Arc;
use std::thread;

pub struct Licensor {
    client: Client,
    licenses: Arc<Injector<License>>,
    coins: Arc<Injector<u32>>,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl Licensor {
    pub fn new(
        client: Client,
        licenses: Arc<Injector<License>>,
        coins: Arc<Injector<u32>>,
        limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    ) -> Licensor {
        Licensor {
            client,
            licenses,
            coins,
            limiter,
        }
    }

    pub async fn run(self) {
        info!("Start licensor in thread {:?}", thread::current().id());
        loop {
            self.limiter.until_ready().await;
            if self.licenses.len() < 5 {
                info!("Licenses count: {}", self.licenses.len());

                let coin = self
                    .coins
                    .steal()
                    .success()
                    .map_or_else(|| vec![], |c| vec![c]);
                let license_result = self.client.request_license(coin).await;
                match license_result {
                    Ok(l) => {
                        self.licenses.push(l);
                    }
                    Err(e) => {
                        error!("Error while requesting license: {:?}", e);
                    }
                }
            }
        }
    }
}
