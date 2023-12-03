use crate::model::{player::*, pickup::PickupHandler};
use std::rc::Rc;
use subtr_actor::{Collector, ReplayProcessor, SubtrActorResult, TimeAdvance};

pub struct StatCollector {
    player_data: PlayerData,
    pickup_map: Rc<PickupHandler>,
}

impl StatCollector {
    pub fn new() -> Self {
        Self {
            player_data: PlayerData::new(),
            pickup_map: Rc::new(PickupHandler::new()),
        }
    }

    pub fn update_player_stats(&mut self, processor: &ReplayProcessor) {
        for player_id in processor.iter_player_ids_in_order() {
            self.player_data
                .players
                .iter_mut()
                .find(|player| player.id == *player_id)
                .unwrap()
                .update_stats(processor);
        }
    }

    fn init_players(&mut self, processor: &mut ReplayProcessor) {
        processor.process_long_enough_to_get_actor_ids().unwrap();
        self.player_data.players.extend(
            processor
                .iter_player_ids_in_order()
                .map(|id| Player::new_from_processor(&processor, id, Rc::clone(&self.pickup_map))),
        );

        processor.reset();
    }

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
        self.update_player_stats(&processor);
        self.pickup_map.update(current_time);
        Ok(TimeAdvance::NextFrame)
    }
}
