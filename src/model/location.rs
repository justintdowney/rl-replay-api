use crate::model::replay::TIME_PER_FRAME;
use serde::Serialize;
use subtr_actor::{PlayerId, ReplayProcessor};

const STANDARD_FIELD_LENGTH: u32 = 5120;
const STANDARD_FIELD_WIDTH: u32 = 4096;
const STANDARD_GOAL_WIDTH_HALF: u32 = 893;
const BALL_SIZE: f32 = 92.75;
const BALL_GROUND_HEIGHT: u32 = 95;
const CAR_GROUND: u32 = 20;
const GOAL_HEIGHT: u32 = 840;
const MAP_Y: u32 = 10280;
const MAP_X: u32 = 8200;
const MAP_Z: u32 = 2050;
const GOAL_X: u32 = 1792;
const GOAL_Y: u32 = 900;
const GOAL_Z: u32 = 640;
const MAP_THIRD: f32 = MAP_Y as f32 / 6.0;
const NEUTRAL_ZONE: f32 = MAP_Y as f32 / 20.0;
const BIG_BOOST_RADIUS: u32 = 208;
const SMALL_BOOST_RADIUS: u32 = 149;
const SMALL_BOOST_HEIGHT: u32 = 165;
const BIG_BOOST_HEIGHT: u32 = 168;

const SMALL_PAD_POSITIONS: &'static [(f32, f32)] = &[
    (0.0, -4240.0),
    (-1792.0, -4184.0),
    (1792.0, -4184.0),
    (-940.0, -3308.07),
    (940.0, -3308.0),
    (0.0, -2816.0),
    (-3584.0, -2484.0),
    (3584.0, -2484.0),
    (-1788.0, -2300.0),
    (1788.0, -2300.0),
    (-2048.0, -1036.0),
    (0.0, -1024.0),
    (2048.0, -1036.0),
    (-1024.0, 0.0),
    (1024.0, 0.0),
    (-2048.0, 1036.0),
    (0.0, 1024.0),
    (2048.0, 1036.0),
    (-1788.0, 2300.0),
    (1788.0, 2300.0),
    (-3584.0, 2484.0),
    (3584.0, 2484.0),
    (0.0, 2816.0),
    (-940.0, 3310.0),
    (940.0, 3308.0),
    (-1792.0, 4184.0),
    (1792.0, 4184.0),
    (0.0, 4240.0),
];

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Location {
    time_half_0: f32,
    time_half_1: f32,
}

impl Location {
    pub fn new() -> Self {
        Self {
            time_half_0: 0.0,
            time_half_1: 0.0,
        }
    }

    pub fn update(&mut self, processor: &ReplayProcessor, player_id: &PlayerId) {
        if let Ok(rb) = processor.get_player_rigid_body(player_id) {
            if rb.location.y <= MAP_THIRD as f32 {
                self.time_half_0 += TIME_PER_FRAME;
            } else if rb.location.y >= MAP_Y as f32 - MAP_THIRD {
                self.time_half_1 += TIME_PER_FRAME;
            }
        }
    }
}
