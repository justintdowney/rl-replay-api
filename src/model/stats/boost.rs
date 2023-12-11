use crate::constants::{BOOST_PER_FRAME, LARGE_BOOST_PICKUP_AMOUNT, SMALL_BOOST_PICKUP_AMOUNT};
use crate::model::stats::Stat;
use crate::stat_collector::{PickupHandler, PositionHandler};
use crate::util::BoostPadSize;
use serde::{Deserialize, Serialize};
use subtr_actor::{PlayerId, ReplayProcessor};

/// `Boost` models the boost stats that are generated during processing
#[derive(Serialize, Deserialize)]
pub struct Boost {
    total_usage: f32,
    frames_active: u32,
    frames_0_25_boost: u32,
    frames_25_50_boost: u32,
    frames_50_75_boost: u32,
    frames_75_100_boost: u32,
    frames_0_boost: u32,
    frames_100_boost: u32,
    small_boost_pickups: u32,
    large_boost_pickups: u32,
    overfill_total: f32, // remainder of pickup when pickup exceeds max boost amount
    overfill_stolen: f32,
}

impl Boost {
    pub fn new() -> Self {
        Boost {
            total_usage: 0.0,
            frames_active: 0,
            frames_0_25_boost: 0,
            frames_25_50_boost: 0,
            frames_50_75_boost: 0,
            frames_75_100_boost: 0,
            frames_0_boost: 0,
            frames_100_boost: 0,
            small_boost_pickups: 0,
            large_boost_pickups: 0,
            overfill_total: 0.0,
            overfill_stolen: 0.0,
        }
    }
}

#[typetag::serde]
impl Stat for Boost {
    fn update(
        &mut self,
        processor: &ReplayProcessor,
        pickup_handler: &mut PickupHandler,
        position_handler: &PositionHandler,
        player_id: &PlayerId,
    ) {
        let boost_active = processor.get_boost_active(player_id).unwrap_or(0) % 2 == 1;
        if boost_active {
            self.total_usage += BOOST_PER_FRAME;
            self.frames_active += 1;
        }

        if let Ok(current_boost) = processor.get_player_boost_level(player_id) {
            if current_boost >= 0.0 && current_boost <= 63.75 {
                self.frames_0_25_boost += 1;
            } else if current_boost > 63.75 && current_boost <= 127.5 {
                self.frames_25_50_boost += 1;
            } else if current_boost > 127.5 && current_boost <= 191.25 {
                self.frames_50_75_boost += 1;
            } else if current_boost > 191.25 && current_boost <= 255.0 {
                self.frames_75_100_boost += 1;
            }

            if current_boost == 0.0 {
                self.frames_0_boost += 1;
            } else if current_boost == 255.0 {
                self.frames_100_boost += 1;
            }

            if let Ok(player_rb) = processor.get_player_rigid_body(player_id) {
                if let Some(pad_size) = pickup_handler.try_pickup(player_rb) {
                    match pad_size {
                        BoostPadSize::Small => {
                            self.small_boost_pickups += 1;
                            if current_boost + SMALL_BOOST_PICKUP_AMOUNT > 255.0 {
                                self.overfill_total +=
                                    (current_boost + SMALL_BOOST_PICKUP_AMOUNT) - 255.0;
                            }
                        }
                        BoostPadSize::Large => {
                            self.large_boost_pickups += 1;
                            if current_boost > 0.0 {
                                self.overfill_total += current_boost;
                            }
                        }
                    }
                }
            }
        }
    }
}
