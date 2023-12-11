use crate::{
    model::stats::{boost::Boost, location::Location, movement::Movement, Stat},
    stat_collector::{PickupHandler, PositionHandler},
};
use serde::Serialize;
use subtr_actor::{PlayerId, ReplayProcessor};

#[derive(Serialize)]
pub struct PlayerData {
    pub players: Vec<Player>,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn get_player_data(self) -> Vec<Player> {
        self.players
    }
}

#[derive(Serialize, Copy, Clone)]
pub enum Team {
    Zero,
    One,
}

#[derive(Serialize)]
pub struct Player {
    pub name: String,
    pub team: Team,
    pub id: PlayerId,
    pub stats: Vec<Box<dyn Stat>>,
}

impl Player {
    //can pass in processor to allow each stat to calculate in its own way
    pub fn new_from_processor(processor: &ReplayProcessor, id: &PlayerId) -> Self {
        Player {
            name: processor.get_player_name(id).unwrap(),
            id: id.clone(),
            team: match processor.get_player_is_team_0(id).unwrap() {
                true => Team::Zero,
                false => Team::One,
            },
            stats: vec![
                Box::new(Boost::new()),
                Box::new(Location::new()),
                Box::new(Movement::new()),
            ],
        }
    }

    pub fn update_stats(
        &mut self,
        processor: &ReplayProcessor,
        pickup_map: &mut PickupHandler,
        position_handler: &PositionHandler,
    ) {
        self.stats
            .iter_mut()
            .map(|x| x.update(processor, pickup_map, position_handler, &self.id))
            .collect()
    }

    pub fn get_stats(&self) -> &Vec<Box<dyn Stat>> {
        &self.stats
    }
}
