use std::collections::HashMap;

use subtr_actor::{BoostPickup, PlayerId, ReplayProcessor};

use crate::model::player::Team;

#[derive(Debug, Clone)]
pub struct PlayerFrame {
    pub id: PlayerId,
    pub team: Team,
    pub rigid_body: Option<boxcars::RigidBody>,
    pub current_boost_amount: Option<f32>,
    pub previous_boost_amount: Option<f32>,
    pub boost_pickup: BoostPickup,
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
        let id = player_id.clone();

        let rigid_body = processor.get_player_rigid_body(player_id).copied().ok();

        let team = match processor.get_player_is_team_0(player_id).unwrap() {
            true => Team::Zero,
            false => Team::One,
        };

        let current_boost_amount = processor.get_player_boost_level(player_id).ok();
        let previous_boost_amount = processor.get_player_previous_boost_amount(player_id);
        let boost_pickup = processor
            .get_player_boost_pickup(player_id)
            .unwrap_or(BoostPickup::None);
        let boost_active = processor.get_boost_active(player_id).unwrap_or(0) % 2 == 1;
        let jump_active = processor.get_jump_active(player_id).unwrap_or(0) % 2 == 1;
        let double_jump_active = processor.get_double_jump_active(player_id).unwrap_or(0) % 2 == 1;
        let dodge_active = processor.get_dodge_active(player_id).unwrap_or(0) % 2 == 1;

        Self {
            id,
            team,
            rigid_body,
            current_boost_amount,
            previous_boost_amount,
            boost_pickup,
            boost_active,
            jump_active,
            double_jump_active,
            dodge_active,
        }
    }
}

pub struct PlayerFrameData {
    frames: HashMap<PlayerId, PlayerFrame>,
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
            .and_modify(|prev_frame| *prev_frame = frame.clone())
            .or_insert(frame.clone());
    }

    pub fn frames(&self) -> &HashMap<PlayerId, PlayerFrame> {
        &self.frames
    }
}

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
pub enum PayloadDataType {
    Player(PlayerFrameData),
    Ball(BallFrameData),
}

// Payload struct containing a vector of payload data
pub struct Payload {
    pub data: Vec<PayloadDataType>,
    // Add any other context-related data here
}

impl Payload {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_data(&mut self, payload_data: PayloadDataType) {
        // Result here most likely
        self.data.push(payload_data);
    }
}
