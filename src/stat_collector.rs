use std::rc::Rc;
use crate::{model::{player::Player, player::PlayerData, replay::ReplayData}, pickup::PickupHandler};
use subtr_actor::{Collector, ReplayProcessor, SubtrActorResult, TimeAdvance};

pub struct StatCollector {
    player_data: PlayerData,
    pickup_map: Rc<PickupHandler>,
    has_printed: bool
}

impl StatCollector {
    pub fn new() -> Self {
        Self {
            player_data: PlayerData::new(),
            pickup_map: Rc::new(PickupHandler::new()),
            has_printed: false
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
    
    pub fn get_player_data(self) -> PlayerData {
        self.player_data
    }

    fn init_players(&mut self, processor: &mut ReplayProcessor) {
        processor.process_long_enough_to_get_actor_ids().unwrap();
        self.player_data.players.extend(
            processor
                .iter_player_ids_in_order()
                .map(|id| Player::new_from_processor(&processor, id, Rc::clone(&self.pickup_map)))
        );

        processor.reset();
    }

    pub fn get_stat_data(mut self, replay: &boxcars::Replay) -> SubtrActorResult<ReplayData> {
        let mut processor = ReplayProcessor::new(replay)?;
        self.init_players(&mut processor);
        processor.process(&mut self)?;
        Ok(ReplayData {
            player_data: self.get_player_data(),
        })
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

        if self.has_printed == false {
            self.has_printed = true;
            println!("{}", serde_json::to_string_pretty(&processor.get_replay_meta().unwrap()).unwrap());
        }

        Ok(TimeAdvance::NextFrame)
    }
}
