use crate::constants::{MAP_THIRD, MAP_Y};
use crate::model::stats::Stat;
use crate::stat_collector::{PickupHandler, PositionHandler};
use serde::{Deserialize, Serialize};
use subtr_actor::{PlayerId, ReplayProcessor};

/// `Location` models & encapsulates location related statistics for each player
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    frames_defensive_half: u32,
    frames_offensive_half: u32,
    frames_defensive_third: u32,
    frames_neutral_third: u32,
    frames_offensive_third: u32,
    frames_closest_to_ball: u32,
    frames_farthest_from_ball: u32,
    frames_farthest_forward: u32,
    frames_farthest_back: u32,
}

impl Location {
    pub fn new() -> Self {
        Self {
            frames_defensive_half: 0,
            frames_offensive_half: 0,
            frames_defensive_third: 0,
            frames_neutral_third: 0,
            frames_offensive_third: 0,
            frames_closest_to_ball: 0,
            frames_farthest_from_ball: 0,
            frames_farthest_forward: 0,
            frames_farthest_back: 0,
        }
    }
}

#[typetag::serde]
impl Stat for Location {
    fn update(
        &mut self,
        processor: &ReplayProcessor,
        pickup_handler: &mut PickupHandler,
        position_handler: &PositionHandler,
        player_id: &PlayerId,
    ) {
        if let Ok(rb) = processor.get_player_rigid_body(player_id) {
            
        }
    }
}
