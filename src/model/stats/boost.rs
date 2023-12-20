use crate::constants::{BOOST_PER_FRAME, LARGE_BOOST_PICKUP_AMOUNT, SMALL_BOOST_PICKUP_AMOUNT};
use crate::model::stats::Stat;
use crate::payload::{Payload, PayloadDataType, PlayerFrame};
use crate::util;
use serde::{Deserialize, Serialize};
use subtr_actor::{BoostPickup, PlayerId};

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
    overfill_updates: u32,
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
            overfill_updates: 0,
        }
    }

    fn update_boost_amount_stats(&mut self, current_boost_amount: f32, player_frame: &PlayerFrame) {
        if player_frame.boost_active {
            self.total_usage += BOOST_PER_FRAME;
            self.frames_active += 1;
        }

        match current_boost_amount {
            amt if amt > 0.0 && amt <= 63.75 => self.frames_0_25_boost += 1,
            amt if amt > 63.76 && amt <= 127.5 => self.frames_25_50_boost += 1,
            amt if amt > 127.51 && amt <= 191.25 => self.frames_50_75_boost += 1,
            amt if amt > 191.26 && amt <= 255.0 => self.frames_75_100_boost += 1,
            _ => {}
        }

        match current_boost_amount {
            amt if amt == 0.0 => self.frames_0_boost += 1,
            amt if amt == 255.0 => self.frames_100_boost += 1,
            _ => {}
        }

        match player_frame.boost_pickup {
            BoostPickup::Small => self.small_boost_pickups += 1,
            BoostPickup::Large => self.large_boost_pickups += 1,
            _ => {}
        };

        let boost_pickup_amount = match player_frame.boost_pickup {
            BoostPickup::Small => SMALL_BOOST_PICKUP_AMOUNT,
            BoostPickup::Large => LARGE_BOOST_PICKUP_AMOUNT,
            _ => 0.0,
        };

        if player_frame.boost_pickup == BoostPickup::None {
            return;
        }

        let overfill = (boost_pickup_amount + current_boost_amount) - 255.0;
        if overfill > 0.0 {
            self.overfill_updates += 1;
            self.overfill_total += overfill;
            if let Some(player_rb) = player_frame.rigid_body {
                if util::is_offensive_third(&player_frame.team, &player_rb) {
                    self.overfill_stolen += overfill;
                }
            }
        }
    }
}

#[typetag::serde]
impl Stat for Boost {
    fn update(&mut self, payload: &mut Payload, player_id: &PlayerId) {
        for payload_data in payload.data.iter() {
            if let PayloadDataType::Player(player_frames) = payload_data {
                let player_frame = player_frames.frames().get(player_id).unwrap();
                if let Some(current_boost_amount) = player_frame.current_boost_amount {
                    self.update_boost_amount_stats(current_boost_amount, player_frame);
                }
            }
        }
    }
}
