use std::{cell::RefCell, rc::Rc, sync::{RwLock, Arc}};

use crate::{replay::TIME_PER_FRAME, pickup::PickupHandler, player::Stat};
use boxcars::RigidBody;
use serde::{Serialize, Deserialize};
use subtr_actor::{PlayerId, ReplayProcessor};

const BOOST_PER_FRAME: f32 = 1.12;

#[derive(Serialize, Deserialize)]
pub struct Boost {
    #[serde(skip)]
    pickup_map: Rc<PickupHandler>,
    total_usage: f32,
    frames_active: u32,
    time_0_25_boost: f32,
    time_25_50_boost: f32,
    time_50_75_boost: f32,
    time_75_100_boost: f32,
    time_0_boost: u32,
    time_100_boost: u32,
    small_boost_pickups: u32,
    large_boost_pickups: u32,
    overfill_total: u32, // remainder of pickup when pickup exceeds max boost amount
    overfill_stolen: u32,
}

impl Boost {
    pub fn new(_pickup_map: Rc<PickupHandler>) -> Self {
        Boost {
            pickup_map: _pickup_map,
            total_usage: 0.0,
            frames_active: 0,
            time_0_25_boost: 0.0,
            time_25_50_boost: 0.0,
            time_50_75_boost: 0.0,
            time_75_100_boost: 0.0,
            time_0_boost: 0,
            time_100_boost: 0,
            small_boost_pickups: 0,
            large_boost_pickups: 0,
            overfill_total: 0,
            overfill_stolen: 0
        }
    }
}

#[typetag::serde]
impl Stat for Boost {
    fn update(&mut self, processor: &ReplayProcessor, player_id: &PlayerId) {
        let boost_active = processor.get_boost_active(player_id).unwrap_or(0) % 2 == 1;
        if boost_active {
            self.total_usage += BOOST_PER_FRAME;
            self.frames_active += 1;
        }

        if let Ok(current_boost) = processor.get_player_boost_level(player_id) {
            if current_boost >= 0.0 && current_boost <= 63.75 {
                self.time_0_25_boost += TIME_PER_FRAME;
            } else if current_boost > 63.75 && current_boost <= 127.5 {
                self.time_25_50_boost += TIME_PER_FRAME;
            } else if current_boost > 127.5 && current_boost <= 191.25 {
                self.time_50_75_boost += TIME_PER_FRAME;
            } else if current_boost > 191.25 && current_boost <= 255.0 {
                self.time_75_100_boost += TIME_PER_FRAME;
            }

            if current_boost == 0.0 {
                self.time_0_boost += 1;
            } else if current_boost == 255.0 {
                self.time_100_boost += 1;
            }
        }

        if let Ok(player_rb) = processor.get_player_rigid_body(player_id) {
            if let Some(pad) = self.pickup_map.check_small_pad_collision(player_rb) { 
                let pickup = self.pickup_map.try_pickup(*pad, processor.get_seconds_remaining().unwrap() as f32);
                if pickup == true {
                    self.small_boost_pickups += 1;
                    
                }
            }
            
            if let Some(pad) =  self.pickup_map.check_large_pad_collision(player_rb) { 
                let pickup = self.pickup_map.try_pickup(*pad, processor.get_seconds_remaining().unwrap() as f32);
                if pickup == true {
                    self.large_boost_pickups += 1;
                }
            }
        }
    }
}

