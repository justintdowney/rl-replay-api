use crate::model::player::{Player, PlayerData};
use crate::payload::{Payload, PayloadDataType, PlayerFrame, PlayerFrameData};
use subtr_actor::{Collector, ReplayProcessor, SubtrActorResult, TimeAdvance};

/// StatCollector model:
/// For implementation of Collector<T> provided by subtr_actor.
pub struct StatCollector {
    player_data: PlayerData,
}

impl StatCollector {
    pub fn new() -> Self {
        Self {
            player_data: PlayerData::new(),
        }
    }

    /// Iterates long enough to obtain all relevant `PlayerId` and initializes the `PlayerData` member.
    ///
    /// # Arguments
    ///
    /// * `processor` - a reference to the `ReplayProcessor`
    fn init_players(&mut self, processor: &mut ReplayProcessor) -> SubtrActorResult<()> {
        processor.process_long_enough_to_get_actor_ids()?;
        self.player_data.players.extend(
            processor
                .iter_player_ids_in_order()
                .map(|id| Player::new_from_processor(&processor, id)),
        );
        processor.reset();
        Ok(())
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
        self.init_players(&mut processor)?;
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
        let mut payload = Payload::new();

        let mut player_frames = PlayerFrameData::new();
        for player in self.player_data.players.iter() {
            player_frames.add_frame(
                &player.id,
                &PlayerFrame::new_from_processor(processor, &player.id, current_time),
            )
        }
        payload.add_data(PayloadDataType::Player(player_frames));

        for player in self.player_data.players.iter_mut() {
            player.update_stats(&mut payload);
        }

        Ok(TimeAdvance::NextFrame)
    }
}
