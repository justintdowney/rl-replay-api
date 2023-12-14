use std::collections::HashMap;

use crate::constants::CAR_HEIGHT_ON_GROUND;
use crate::model::stats::Stat;
use crate::payload::Payload;
use crate::stat_collector::PickupHandler;
use boxcars::RigidBody;
use serde::{Deserialize, Serialize};
use subtr_actor::PlayerId;
use typetag::serde;

/// `Movement` models & encapsulates all movement related data for each player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movement {
    average_speed: f32,
    distance_traveled: f32,
    frames_super_sonic: u32,
    frames_boost_speed: u32,
    frames_slow_speed: u32,
    frames_on_ground: u32,
    frames_low_air: u32,
    frames_high_air: u32,
    frames_boost_while_super_sonic: u32,
    #[serde(skip)]
    frames_count: u32,
    #[serde(skip)]
    speed_total: f32,
    #[serde(skip)]
    previous_rb: Option<RigidBody>,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            average_speed: 0.0,
            distance_traveled: 0.0,
            frames_super_sonic: 0,
            frames_boost_speed: 0,
            frames_slow_speed: 0,
            frames_on_ground: 0,
            frames_low_air: 0,
            frames_high_air: 0,
            frames_boost_while_super_sonic: 0,
            speed_total: 0.0,
            frames_count: 0,
            previous_rb: None,
        }
    }

    fn calculate_speed(&self, lin_vel: boxcars::Vector3f) -> f32 {
        f32::sqrt(lin_vel.x * lin_vel.x + lin_vel.y * lin_vel.y + lin_vel.z * lin_vel.z)
    }

    fn update_speed_stats(&mut self, speed: f32, boost_active: bool) {
        if speed >= 2200.0 {
            self.frames_super_sonic += 1;

            if boost_active {
                self.frames_boost_while_super_sonic += 1;
            }
        } else if speed >= 1400.0 {
            self.frames_boost_speed += 1;
        } else if speed < 1400.0 {
            self.frames_slow_speed += 1;
        }

        self.speed_total += speed;
        self.frames_count += 1;
        self.average_speed = self.speed_total / self.frames_count as f32;
    }

    fn update_height_stats(&mut self, location_z: f32) {
        if location_z <= CAR_HEIGHT_ON_GROUND {
            self.frames_on_ground += 1;
        } else if location_z <= 840.0 {
            self.frames_low_air += 1;
        } else if location_z > 840.0 {
            self.frames_high_air += 1;
        }
    }

    fn update_distance_stats(&mut self, player_rb: &RigidBody) {
        if let Some(previous_rb) = &self.previous_rb {
            self.distance_traveled += f32::abs(player_rb.location.x - previous_rb.location.x)
                + f32::abs(player_rb.location.y - previous_rb.location.y)
                + f32::abs(player_rb.location.z - previous_rb.location.z);
        }

        self.previous_rb = Some(player_rb.clone());
    }
}

#[typetag::serde]
impl Stat for Movement {
    fn update(&mut self, payload: &mut Payload, player_id: &PlayerId) {
        /*if let Some(player_frame) = player_payload.get(player_id) {
            if let Some(player_rb) = player_frame.rigid_body {
                if let Some(lin_vel) = player_rb.linear_velocity {
                    let speed = self.calculate_speed(lin_vel);

                    self.update_speed_stats(speed, player_frame.boost_active);
                }
                self.update_distance_stats(&player_rb);
                self.update_height_stats(player_rb.location.z);
            }
        }*/
    }
}
