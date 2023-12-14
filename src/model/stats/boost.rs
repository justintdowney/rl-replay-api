use crate::constants::{BOOST_PER_FRAME, SMALL_BOOST_PICKUP_AMOUNT, STARTING_BOOST_VALUE};
use crate::model::stats::Stat;
use crate::payload::{Payload, PayloadDataType, PlayerFrame};
use crate::stat_collector::PickupHandler;
use crate::util::BoostPickupEvent;
use serde::{Deserialize, Serialize};
use subtr_actor::PlayerId;

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
    #[serde(skip)]
    most_recent_boost_value: f32,
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
            most_recent_boost_value: STARTING_BOOST_VALUE,
        }
    }

    fn update_boost_amount_stats(
        &mut self,
        boost_amount: f32,
        player_frame: &PlayerFrame,
        pickup_handler: &mut PickupHandler,
    ) {
        if player_frame.boost_active {
            self.total_usage += BOOST_PER_FRAME;
            self.frames_active += 1;
        }

        match boost_amount {
            amt if amt > 0.0 && amt <= 63.75 => self.frames_0_25_boost += 1,
            amt if amt > 63.76 && amt <= 127.5 => self.frames_25_50_boost += 1,
            amt if amt > 127.51 && amt <= 191.25 => self.frames_50_75_boost += 1,
            amt if amt > 191.26 && amt <= 255.0 => self.frames_75_100_boost += 1,
            _ => {}
        }

        match boost_amount {
            amt if amt == 0.0 => self.frames_0_boost += 1,
            amt if amt == 255.0 => self.frames_100_boost += 1,
            _ => {}
        }

        if let Some(player_rb) = player_frame.rigid_body {
            if self.most_recent_boost_value >= boost_amount {
                self.most_recent_boost_value = boost_amount;
                return;
            }

            let pickup_result = pickup_handler.try_pickup(&player_rb);
            match pickup_result {
                BoostPickupEvent::Small => {
                    self.small_boost_pickups += 1;
                    if boost_amount + SMALL_BOOST_PICKUP_AMOUNT > 255.0 {
                        self.overfill_total += (boost_amount + SMALL_BOOST_PICKUP_AMOUNT) - 255.0;
                    }
                }
                BoostPickupEvent::Large => {
                    self.large_boost_pickups += 1;
                    if boost_amount > 0.0 {
                        self.overfill_total += boost_amount;
                    }
                }
                BoostPickupEvent::None => return,
            }
        }
    }
}

#[typetag::serde]
impl Stat for Boost {
    fn update(&mut self, payload: &mut Payload, player_id: &PlayerId) {
        let mut pickup_handler: Option<&mut PickupHandler> = None;

        for payload_data in payload.data.iter() {
            if let PayloadDataType::Pickup(pickup_data) = payload_data {
                pickup_handler = Some(&mut pickup_data);
            }
            match payload_data {
                PayloadDataType::Pickup(pickup_data) => pickup_handler = Some(&mut pickup_data),
                _ => None,
            }
        }

        for payload_data in payload.data.iter() {
            if let PayloadDataType::Player(all_player_frames) = payload_data {
                let player_frames = all_player_frames.frames().get(player_id).unwrap();
                let latest_frame = player_frames.iter().nth_back(1).unwrap();

                if let Some(boost_amount) = latest_frame.boost_amount {
                    //self.update_boost_amount_stats(boost_amount, latest_frame)
                }
            }
        }
    }
}
