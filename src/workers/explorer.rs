use crate::client::Client;
use crate::data::area::Area;
use crate::data::dig_point::DigPoint;
use crossbeam_deque::Injector;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::RateLimiter;
use log::info;
use std::collections::BinaryHeap;
use std::sync::Arc;
use std::thread;
use tokio::sync::Mutex;

pub struct Explorer {
    client: Client,
    dig_queue: Arc<Injector<DigPoint>>,
    queue: Arc<Mutex<BinaryHeap<Area>>>,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl Explorer {
    pub fn new(
        client: Client,
        dig_queue: Arc<Injector<DigPoint>>,
        queue: Arc<Mutex<BinaryHeap<Area>>>,
        limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    ) -> Explorer {
        Explorer {
            client,
            dig_queue,
            queue,
            limiter,
        }
    }

    pub async fn explore(self) {
        info!("Start exploring in thread {:?}", thread::current().id());
        loop {
            self.limiter.until_ready().await;
            let mut guard = self.queue.lock().await;
            let area = guard.pop();
            drop(guard);
            if let Some(t) = area {
                if let Ok(e) = self.client.explore(&t).await {
                    if e.amount == 0 {
                        continue;
                    }
                    match (&t.size_x, &t.size_y) {
                        (1, 1) => {
                            self.dig_queue
                                .push(DigPoint::new(t.pos_x, t.pos_y, e.amount));
                            info!("Dig queue size: {}", self.dig_queue.len());
                        }
                        (1, ys) => {
                            let next_area_l = Area {
                                pos_x: t.pos_x,
                                pos_y: t.pos_y,
                                size_x: 1,
                                size_y: ys / 2,
                            };
                            let next_area_r = Area {
                                pos_x: t.pos_x,
                                pos_y: t.pos_y + ys / 2,
                                size_x: 1,
                                size_y: ys / 2,
                            };
                            let mut guard = self.queue.lock().await;
                            guard.push(next_area_l);
                            guard.push(next_area_r);
                            drop(guard);
                        }
                        (xs, ys) => {
                            let next_area_l = Area {
                                pos_x: t.pos_x,
                                pos_y: t.pos_y,
                                size_x: xs / 2,
                                size_y: *ys,
                            };
                            let next_area_r = Area {
                                pos_x: t.pos_x + xs / 2,
                                pos_y: t.pos_y,
                                size_x: xs / 2,
                                size_y: *ys,
                            };
                            let mut guard = self.queue.lock().await;
                            guard.push(next_area_l);
                            guard.push(next_area_r);
                            drop(guard);
                        }
                    }
                } else {
                    let mut guard = self.queue.lock().await;
                    guard.push(t);
                    drop(guard);
                }
            }
        }
    }
}
