use crate::client::Client;
use crate::config::AppConfig;
use crate::data::area::Area;
use crate::workers::bookkeeper::Bookkeeper;
use crate::workers::digger::Digger;
use crate::workers::explorer::Explorer;
use crate::workers::licensor::Licensor;
use crossbeam_deque::Injector;
use governor::Quota;
use governor::RateLimiter;
use std::collections::BinaryHeap;
use std::convert::TryFrom;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct GameLogic {
    config: AppConfig,
}

impl GameLogic {
    pub fn new(config: AppConfig) -> GameLogic {
        GameLogic { config }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let licenses = Arc::new(Injector::new());
        let dig_queue = Arc::new(Injector::new());
        let treasure_queue = Arc::new(Injector::new());
        let unexplored_areas = self.premade_areas();
        let explore_queue = Arc::new(Mutex::new(unexplored_areas));
        let coins = Arc::new(Injector::new());

        let license_limiter = Arc::new(RateLimiter::direct(Quota::per_second(
            NonZeroU32::try_from(self.config.licensor_rps_limit).unwrap(),
        )));
        let explorer_limiter = Arc::new(RateLimiter::direct(Quota::per_second(
            NonZeroU32::try_from(self.config.explorer_rps_limit).unwrap(),
        )));
        let digger_limiter = Arc::new(RateLimiter::direct(Quota::per_second(
            NonZeroU32::try_from(self.config.digger_rps_limit).unwrap(),
        )));
        let bookkeeper_limiter = Arc::new(RateLimiter::direct(Quota::per_second(
            NonZeroU32::try_from(self.config.bookkeeper_rps_limit).unwrap(),
        )));

        let mut handles = (0..self.config.explorers_counter)
            .map(|_| {
                let expl = Explorer::new(
                    Client::new(self.config.url.clone(), Duration::from_millis(100)),
                    Arc::clone(&dig_queue),
                    Arc::clone(&explore_queue),
                    Arc::clone(&explorer_limiter),
                );
                tokio::spawn(expl.explore())
            })
            .collect::<Vec<_>>();

        let mut diggers_handles = (0..self.config.diggers_count)
            .map(|_| {
                let digger = Digger::new(
                    Client::new(self.config.url.clone(), Duration::from_millis(100)),
                    Arc::clone(&dig_queue),
                    Arc::clone(&treasure_queue),
                    Arc::clone(&licenses),
                    Arc::clone(&digger_limiter),
                );
                tokio::spawn(digger.run())
            })
            .collect::<Vec<_>>();

        let mut licensor_handles = (0..self.config.licensors_count)
            .map(|_| {
                let licensor = Licensor::new(
                    Client::new(self.config.url.clone(), Duration::from_millis(5000)),
                    Arc::clone(&licenses),
                    Arc::clone(&coins),
                    Arc::clone(&license_limiter),
                );
                tokio::spawn(licensor.run())
            })
            .collect::<Vec<_>>();

        let mut bookkeeper_handle = (0..self.config.bookkeeper_count)
            .map(|_| {
                let bookkeeper = Bookkeeper::new(
                    Client::new(self.config.url.clone(), Duration::from_millis(5000)),
                    Arc::clone(&treasure_queue),
                    Arc::clone(&coins),
                    Arc::clone(&bookkeeper_limiter),
                );
                tokio::spawn(bookkeeper.run())
            })
            .collect::<Vec<_>>();

        handles.append(&mut bookkeeper_handle);
        handles.append(&mut diggers_handles);
        handles.append(&mut licensor_handles);

        for h in handles {
            h.await?;
        }
        Ok(())
    }

    fn premade_areas(&self) -> BinaryHeap<Area> {
        let (x_step, y_step) = (32, 32);
        let areas = (0..3500 - x_step)
            .step_by(x_step)
            .flat_map(|x| {
                (0..3500 - y_step)
                    .step_by(y_step)
                    .map(move |y| Area::new(x as u32, y as u32, x_step as u32, y_step as u32))
            })
            .collect::<Vec<Area>>();
        BinaryHeap::from(areas)
    }
}
