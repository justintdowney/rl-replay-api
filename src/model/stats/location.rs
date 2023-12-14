use crate::constants::{BACK_WALL, MAP_SIXTH, MAP_THIRD, MAP_Y};
use crate::model::player::Team;
use crate::model::stats::Stat;
use crate::payload::{Payload, PlayerFrame};
use crate::stat_collector::PickupHandler;
use boxcars::RigidBody;
use serde::{Deserialize, Serialize};
use subtr_actor::PlayerId;

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

    fn update_team_stats(&mut self, player_rb: &RigidBody) {
        if player_rb.location.y > 0.0 {
            self.frames_defensive_half += 1;
        } else {
            self.frames_offensive_half += 1;
        }

        if player_rb.location.y > BACK_WALL - MAP_SIXTH {
            self.frames_offensive_third += 1;
        } else if player_rb.location.y < -BACK_WALL + MAP_SIXTH {
            self.frames_defensive_third += 1;
        } else if player_rb.location.y > -(MAP_THIRD / 2.0)
            && player_rb.location.y < (MAP_THIRD / 2.0)
        {
            self.frames_neutral_third += 1;
        }
    }

    fn update_location_stats(&mut self, player_frame: &PlayerFrame) {
        /*if let Some(player_rb) = player_frame.rigid_body {
            if player_frame.team == Team::Zero {
                self.update_team_stats(&player_rb);
            } else if player_frame.team == Team::One {
                self.update_team_stats(&player_rb);
            }
        }*/
    }
}

#[typetag::serde]
impl Stat for Location {
    fn update(&mut self, player_payload: &mut Payload, player_id: &PlayerId) {
        /*if let Some(player_frame) = player_payload.get(player_id) {
            self.update_location_stats(player_frame);

            for (candidate_player_id, candidate_frame) in player_payload.frames.iter() {
                if let Some(candidate_player_rb) = candidate_frame.rigid_body {}
            }
        }*/
    }
}
