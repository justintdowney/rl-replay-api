use std::collections::HashMap;

use subtr_actor::{PlayerId, ReplayProcessor};

use crate::{model::player::Team, stat_collector::PickupHandler};

// Define a trait for extensible payload data
trait PayloadData {}

#[derive(Clone)]
pub struct PlayerFrame {
    pub rigid_body: Option<boxcars::RigidBody>,
    pub boost_amount: Option<f32>,
    pub boost_active: bool,
    pub jump_active: bool,
    pub double_jump_active: bool,
    pub dodge_active: bool,
}

impl PlayerFrame {
    pub fn new_from_processor(
        processor: &ReplayProcessor,
        player_id: &PlayerId,
        current_time: f32,
    ) -> Self {
        let rigid_body = processor
            .get_interpolated_player_rigid_body(player_id, current_time, 0.0)
            .ok();

        let team = match processor.get_player_is_team_0(player_id).unwrap() {
            true => Team::Zero,
            false => Team::One,
        };

        let boost_amount = processor.get_player_boost_level(player_id).ok();
        let boost_active = processor.get_boost_active(player_id).unwrap_or(0) % 2 == 1;
        let jump_active = processor.get_jump_active(player_id).unwrap_or(0) % 2 == 1;
        let double_jump_active = processor.get_double_jump_active(player_id).unwrap_or(0) % 2 == 1;
        let dodge_active = processor.get_dodge_active(player_id).unwrap_or(0) % 2 == 1;

        Self {
            rigid_body,
            boost_amount,
            boost_active,
            jump_active,
            double_jump_active,
            dodge_active,
        }
    }
}

// Example implementation of a payload data type
pub struct PlayerFrameData {
    frames: HashMap<PlayerId, Vec<PlayerFrame>>,
}

impl PlayerFrameData {
    pub fn new() -> Self {
        Self {
            frames: HashMap::new(),
        }
    }

    pub fn add_frame(&mut self, id: &PlayerId, frame: &PlayerFrame) {
        self.frames
            .entry(id.clone())
            .and_modify(|frames| frames.push(frame.clone()))
            .or_insert(vec![frame.clone()]);
    }

    pub fn frames(&self) -> &HashMap<PlayerId, Vec<PlayerFrame>> {
        &self.frames
    }
}

impl PayloadData for PlayerFrameData {}

#[derive(Clone, Copy)]
pub struct BallFrame {
    pub rigid_body: Option<boxcars::RigidBody>,
}

impl BallFrame {
    fn new_from_processor(processor: &ReplayProcessor) -> Self {
        let rigid_body = processor.get_ball_rigid_body().ok();
        Self {
            rigid_body: rigid_body.copied(),
        }
    }
}

pub struct BallFrameData {
    frames: Vec<BallFrame>,
}

impl BallFrameData {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn add_frame(&mut self, frame: &BallFrame) {
        self.frames.push(frame.clone());
    }
}

// Enum to represent different payload data types
pub enum PayloadDataType<'pl> {
    Player(PlayerFrameData),
    Ball(BallFrameData),
    Pickup(&'pl PickupHandler), // Add other payload data types here
}

// Payload struct containing a vector of payload data
pub struct Payload<'pl> {
    pub data: Vec<PayloadDataType<'pl>>,
    // Add any other context-related data here
}

impl Payload<'_> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_data(&mut self, payload_data: PayloadDataType) {
        // Result here most likely
        self.data.push(payload_data);
    }
}
