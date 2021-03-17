use crate::client::Client;
use crate::data::dig::Dig;
use crate::data::dig_errors::DigError;
use crate::data::dig_point::DigPoint;
use crate::data::license::License;
use crossbeam_deque::{Injector, Steal};
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::RateLimiter;
use log::{info, warn};
use std::sync::Arc;
use std::thread;

pub struct Digger {
    client: Client,
    dig_queue: Arc<Injector<DigPoint>>,
    treasure_queue: Arc<Injector<String>>,
    licenses: Arc<Injector<License>>,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl Digger {
    pub fn new(
        client: Client,
        dig_queue: Arc<Injector<DigPoint>>,
        treasure_queue: Arc<Injector<String>>,
        licenses: Arc<Injector<License>>,
        limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    ) -> Digger {
        Digger {
            client,
            dig_queue,
            treasure_queue,
            licenses,
            limiter,
        }
    }

    pub async fn run(self) {
        info!("Start digging in thread {:?}", thread::current().id());
        loop {
            self.limiter.until_ready().await;
            if let Steal::Success(license) = self.licenses.steal() {
                if let Steal::Success(point) = self.dig_queue.steal() {
                    let dig_p = Dig::from_dig_point(point, &license.id);
                    let dig_result = self.client.dig(dig_p).await;
                    let license_after_dig = match dig_result {
                        Ok(treasure) => {
                            info!("Dig successful. Treasure found {:?}!", treasure);
                            for t in treasure {
                                self.treasure_queue.push(t)
                            }
                            license.decrease_dig_allowed()
                        }
                        Err(DigError::TreasureNotFound) => {
                            info!("Dig successful. Nothing found");
                            self.dig_queue.push(point.increase_depth());
                            license.decrease_dig_allowed()
                        }
                        Err(DigError::InvalidLicense) => {
                            info!("Dig failed due to invalid license {:?}", license.id);
                            None
                        }
                        Err(DigError::Unknown(e)) => {
                            info!("Dig failed due to {:?}", e);
                            Some(license)
                        }
                    };
                    license_after_dig.map(|l| self.licenses.push(l));
                    warn!("Remaining dig queue {:?}", self.dig_queue.len());
                } else {
                    self.licenses.push(license);
                }
            }
        }
    }
}
