use crate::constants::{
    BOOST_COOLDOWN, LARGE_BOOST_HEIGHT, LARGE_BOOST_PADS, LARGE_BOOST_RADIUS, SMALL_BOOST_HEIGHT,
    SMALL_BOOST_PADS, SMALL_BOOST_RADIUS,
};

use crate::model::player::Team;
use crate::{
    model::player::{Player, PlayerData},
    util::{BoostPad, BoostPickupEvent},
};
use boxcars::RigidBody;
use std::collections::HashMap;
use subtr_actor::{Collector, PlayerId, ReplayProcessor, SubtrActorResult, TimeAdvance};

#[derive(Clone, Copy)]
pub struct PlayerFrame {
    pub team: Team,
    pub rigid_body: Option<boxcars::RigidBody>,
    pub boost_amount: Option<f32>,
    pub boost_active: bool,
    pub jump_active: bool,
    pub double_jump_active: bool,
    pub dodge_active: bool,
}

impl PlayerFrame {
    fn new_from_processor(
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
            team,
            rigid_body,
            boost_amount,
            boost_active,
            jump_active,
            double_jump_active,
            dodge_active,
        }
    }
}

pub struct PlayerPayload {
    pub frames: HashMap<PlayerId, PlayerFrame>,
    pub ball_frame: Option<BallFrame>,
}

impl PlayerPayload {
    pub fn new() -> Self {
        Self {
            frames: HashMap::new(),
            ball_frame: None,
        }
    }

    pub fn load_payload(&mut self, processor: &ReplayProcessor, current_time: f32) {
        for player_id in processor.iter_player_ids_in_order() {
            self.add_frame(
                player_id.clone(),
                PlayerFrame::new_from_processor(processor, &player_id, current_time),
            );
        }

        self.ball_frame = Some(BallFrame::new_from_processor(processor));
    }

    pub fn add_frame(&mut self, player_id: PlayerId, player_frame: PlayerFrame) {
        self.frames.insert(player_id, player_frame);
    }

    pub fn get(&self, player_id: &PlayerId) -> Option<&PlayerFrame> {
        self.frames.get(player_id)
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
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

/// StatCollector model:
/// For implementation of Collector<T> provided by subtr_actor.
pub struct StatCollector {
    player_data: PlayerData,
    pickup_map: PickupHandler,
    player_payload: PlayerPayload,
}

impl StatCollector {
    pub fn new() -> Self {
        Self {
            player_data: PlayerData::new(),
            pickup_map: PickupHandler::new(),
            player_payload: PlayerPayload::new(),
        }
    }

    /// Iterates long enough to obtain all relevant `PlayerId` and initializes the `PlayerData` member.
    ///
    /// # Arguments
    ///
    /// * `processor` - a reference to the `ReplayProcessor`
    fn init_players(&mut self, processor: &mut ReplayProcessor) {
        processor.process_long_enough_to_get_actor_ids().unwrap();
        self.player_data.players.extend(
            processor
                .iter_player_ids_in_order()
                .map(|id| Player::new_from_processor(&processor, id)),
        );

        processor.reset();
    }

    /// The entry point to start processing the replay data.
    ///
    /// # Arguments
    ///
    /// * `processor` - a reference to the `ReplayProcessor`
    ///
    /// # Returns
    ///
    /// * `SubtrActorResult<PlayerData>` - the result of the processing
    pub fn analyze(mut self, replay: &boxcars::Replay) -> SubtrActorResult<PlayerData> {
        let mut processor = ReplayProcessor::new(replay)?;
        self.init_players(&mut processor);
        processor.process(&mut self)?;
        Ok(self.player_data)
    }
}

impl Collector for StatCollector {
    fn process_frame(
        &mut self,
        processor: &ReplayProcessor,
        _frame: &boxcars::Frame,
        _frame_number: usize,
        current_time: f32,
    ) -> SubtrActorResult<TimeAdvance> {
        self.player_payload.load_payload(processor, current_time);

        for player in self.player_data.players.iter_mut() {
            player.update_stats(&self.player_payload, &mut self.pickup_map);
        }

        self.pickup_map.update(current_time);
        self.player_payload.clear();

        Ok(TimeAdvance::NextFrame)
    }
}

/// `PickupMap` encapsulates pickup related data, providing access to view the currently disabled boost pads.
#[derive(Default)]
pub struct PickupHandler {
    current_time: f32,
    disabled_boost_pads: HashMap<BoostPad, f32>,
}

impl PickupHandler {
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            disabled_boost_pads: HashMap::new(),
        }
    }

    /// Checks if the boost pad the player collided with is currently on cooldown.
    ///
    /// # Arguments
    ///
    /// * `boost_pad` - reference to the `BoostPad` the player collided with
    ///
    /// * `time` - the replay time the boost was picked up
    ///
    /// # Returns
    ///
    /// * `bool` - true if the pickup was successful, or false if not
    pub fn try_pickup(&mut self, player_rb: &RigidBody) -> BoostPickupEvent {
        if let Some(small_boost_pad) = self.check_small_pad_collision(player_rb) {
            if !self.disabled_boost_pads.contains_key(small_boost_pad) {
                self.disabled_boost_pads
                    .insert(small_boost_pad.clone(), self.current_time);
                return BoostPickupEvent::Small;
            }
        } else if let Some(large_boost_pad) = self.check_large_pad_collision(player_rb) {
            if !self.disabled_boost_pads.contains_key(large_boost_pad) {
                self.disabled_boost_pads
                    .insert(large_boost_pad.clone(), self.current_time);
                return BoostPickupEvent::Large;
            }
        }
        BoostPickupEvent::None
    }

    fn check_large_pad_collision(&self, rb: &RigidBody) -> Option<&BoostPad> {
        LARGE_BOOST_PADS.iter().find(|boost_pad| {
            rb.location.y <= boost_pad.y + LARGE_BOOST_RADIUS
                && rb.location.y >= boost_pad.y - LARGE_BOOST_RADIUS
                && rb.location.x <= boost_pad.x + LARGE_BOOST_RADIUS
                && rb.location.x >= boost_pad.x - LARGE_BOOST_RADIUS
                && rb.location.z <= LARGE_BOOST_HEIGHT
        })
    }

    fn check_small_pad_collision(&self, rb: &RigidBody) -> Option<&BoostPad> {
        SMALL_BOOST_PADS.iter().find(|boost_pad| {
            rb.location.y <= boost_pad.y + SMALL_BOOST_RADIUS
                && rb.location.y >= boost_pad.y - SMALL_BOOST_RADIUS
                && rb.location.x <= boost_pad.x + SMALL_BOOST_RADIUS
                && rb.location.x >= boost_pad.x - SMALL_BOOST_RADIUS
                && rb.location.z <= SMALL_BOOST_HEIGHT
        })
    }

    /// Checks if any `BoostPad` on cooldown have expired, making them available again
    ///
    /// # Arguments
    ///
    /// * `current_time` - the current time in the replay
    pub fn update(&mut self, current_time: f32) {
        self.current_time = current_time;

        self.disabled_boost_pads
            .retain(|_, initial_time| current_time - *initial_time < BOOST_COOLDOWN);
    }
}
