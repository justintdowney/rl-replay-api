use std::collections::HashMap;

use crate::constants::{BACK_WALL, MAP_THIRD, MAP_Y};
use crate::model::player::Team;
use crate::model::stats::Stat;
use crate::payload::{Payload, PayloadDataType, PlayerFrame};
use crate::util;
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

    fn update_location_stats(&mut self, team: &Team, player_rb: &RigidBody) {
        self.frames_defensive_half += util::is_defensive_half(team, player_rb) as u32;
        self.frames_offensive_half += util::is_offensive_half(team, player_rb) as u32;
        self.frames_defensive_third += util::is_defensive_third(team, player_rb) as u32;
        self.frames_neutral_third += util::is_neutral_third(team, player_rb) as u32;
        self.frames_offensive_third += util::is_offensive_third(team, player_rb) as u32;
    }

    fn update_relative_location_stats(
        &mut self,
        team: &Team,
        target_player: &PlayerId,
        player_frames: &HashMap<PlayerId, PlayerFrame>,
    ) {
        self.frames_farthest_back +=
            util::is_farthest_back(team, target_player, player_frames) as u32;
        self.frames_farthest_forward +=
            util::is_farthest_forward(team, target_player, player_frames) as u32;
    }
}

#[typetag::serde]
impl Stat for Location {
    fn update(&mut self, payload: &mut Payload, player_id: &PlayerId) {
        for payload_data in payload.data.iter() {
            if let PayloadDataType::Player(player_frames) = payload_data {
                let player_frame = player_frames.frames().get(player_id).unwrap();

                self.update_relative_location_stats(
                    &player_frame.team,
                    player_id,
                    player_frames.frames(),
                );

                if let Some(player_rb) = player_frame.rigid_body {
                    self.update_location_stats(&player_frame.team, &player_rb);
                }
            }
        }
    }
}
