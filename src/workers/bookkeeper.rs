use crate::client::Client;
use crossbeam_deque::{Injector, Steal};
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::RateLimiter;
use log::{error, info, warn};
use std::sync::Arc;
use std::thread;

pub struct Bookkeeper {
    client: Client,
    treasure_queue: Arc<Injector<String>>,
    coins: Arc<Injector<u32>>,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl Bookkeeper {
    pub fn new(
        client: Client,
        treasure_queue: Arc<Injector<String>>,
        coins: Arc<Injector<u32>>,
        limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    ) -> Bookkeeper {
        Bookkeeper {
            client,
            treasure_queue,
            coins,
            limiter,
        }
    }

    pub async fn run(self) {
        info!("Start bookkeeper in thread {:?}", thread::current().id());
        loop {
            self.limiter.until_ready().await;
            if let Steal::Success(treasure) = self.treasure_queue.steal() {
                info!("Cashing treasure {}", treasure);
                let _ = self.client.cash(&treasure).await.map(|c| c.iter().for_each(|coin| self.coins.push(*coin))).map_err(|err| {
                    error!("Error while cashing treasure {:?}", err);
                });
                warn!("Remaining treasure count {:?}", self.treasure_queue.len());
            }
        }
    }
}
