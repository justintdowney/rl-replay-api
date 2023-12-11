use crate::model::stats::Stat;
use crate::stat_collector::PickupHandler;
use crate::{constants::CAR_HEIGHT_ON_GROUND, stat_collector::PositionHandler};
use boxcars::RigidBody;
use serde::{Deserialize, Serialize};
use subtr_actor::{PlayerId, ReplayProcessor};
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
}

#[typetag::serde]
impl Stat for Movement {
    fn update(
        &mut self,
        processor: &ReplayProcessor,
        _pickup_map: &mut PickupHandler,
        _position_handler: &PositionHandler,
        player_id: &PlayerId,
    ) {
        if let Ok((player_rb, idx)) = processor.get_player_rigid_body_and_updated(player_id) {
            if player_rb.sleeping {
                return;
            }

            let boost_active = processor.get_boost_active(player_id).unwrap_or(0) % 2 == 1;
            let lin_vel = player_rb.linear_velocity.unwrap();
            let speed =
                f32::sqrt(lin_vel.x * lin_vel.x + lin_vel.y * lin_vel.y + lin_vel.z * lin_vel.z);

            // Player speed stats
            if speed >= 2200.0 {
                self.frames_super_sonic += 1;

                if boost_active == true {
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

            // Player height stats
            if player_rb.location.z <= CAR_HEIGHT_ON_GROUND {
                self.frames_on_ground += 1;
            } else if player_rb.location.z <= 840.0 {
                self.frames_low_air += 1;
            } else if player_rb.location.z > 840.0 {
                self.frames_high_air += 1;
            }

            // Player distance stats
            if self.previous_rb.is_some() {
                let previous_rb = self.previous_rb.unwrap();
                self.distance_traveled += f32::abs(player_rb.location.x - previous_rb.location.x)
                    + f32::abs(player_rb.location.y - previous_rb.location.y)
                    + f32::abs(player_rb.location.z - previous_rb.location.z);
            }

            self.previous_rb = Some(player_rb.clone());
        }
    }
}