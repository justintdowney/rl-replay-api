use crate::constants::{
    BOOST_COOLDOWN, LARGE_BOOST_HEIGHT, LARGE_BOOST_PADS, LARGE_BOOST_RADIUS, SMALL_BOOST_HEIGHT,
    SMALL_BOOST_PADS, SMALL_BOOST_RADIUS,
};
use crate::{
    model::player::{Player, PlayerData},
    util::{BoostPad, BoostPadSize},
};
use boxcars::RigidBody;
use std::collections::HashMap;
use subtr_actor::{Collector, PlayerId, ReplayProcessor, SubtrActorResult, TimeAdvance};

/// StatCollector model:
/// For implementation of Collector<T> provided by subtr_actor.
pub struct StatCollector {
    player_data: PlayerData,
    pickup_map: PickupHandler,
    position_handler: PositionHandler,
}

impl StatCollector {
    pub fn new() -> Self {
        Self {
            player_data: PlayerData::new(),
            pickup_map: PickupHandler::new(),
            position_handler: PositionHandler::new(),
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
        for player in self.player_data.players.iter_mut() {
            player.update_stats(processor, &mut self.pickup_map, &self.position_handler);
            if let Ok(player_rb) = processor.get_player_rigid_body(&player.id) {
                self.position_handler.update(&player.id, player_rb);
            }
        }

        self.pickup_map.update(current_time);
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
    pub fn try_pickup(&mut self, player_rb: &RigidBody) -> Option<BoostPadSize> {
        if let Some(small_boost_pad) = self.check_small_pad_collision(player_rb) {
            if !self.disabled_boost_pads.contains_key(small_boost_pad) {
                self.disabled_boost_pads
                    .insert(small_boost_pad.clone(), self.current_time);
                return Some(BoostPadSize::Small);
            }
        } else if let Some(large_boost_pad) = self.check_large_pad_collision(player_rb) {
            if !self.disabled_boost_pads.contains_key(large_boost_pad) {
                self.disabled_boost_pads
                    .insert(large_boost_pad.clone(), self.current_time);
                return Some(BoostPadSize::Large);
            }
        }
        None
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

pub struct PositionHandler {
    player_positions: HashMap<PlayerId, RigidBody>,
}

impl PositionHandler {
    pub fn new() -> Self {
        Self {
            player_positions: HashMap::new(),
        }
    }

    pub fn get_player_positions(&self) -> &HashMap<PlayerId, RigidBody> {
        &self.player_positions
    }

    pub fn update(&mut self, player_id: &PlayerId, player_rb: &RigidBody) {
        // should not have magic number i.e. 6, player_count
        if self.player_positions.len() > 6 as usize {
            self.player_positions.clear();
        }

        self.player_positions
            .insert(player_id.clone(), player_rb.clone());
    }
}
