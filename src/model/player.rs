use std::rc::Rc;

use crate::{pickup::PickupHandler, boost::Boost, core::Core};
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

#[derive(Serialize)]
pub struct Player {
    pub name: String,
    pub id: PlayerId,
    pub stats: Vec<Box<dyn Stat>>
}

impl Player {
    //can pass in processor to allow each stat to calculate in its own way
    pub fn new_from_processor(processor: &ReplayProcessor, _id: &PlayerId, pickup_map: Rc<PickupHandler>) -> Self {
        Player {
            name: processor.get_player_name(_id).unwrap(),
            id: _id.clone(),
            stats: vec![
                Box::new(Boost::new(pickup_map)),
/*                 Box::new(Core::new()),
                Box::new(Location::new()),
                Box::new(Movement::new()) */
            ]
        }
    }

    pub fn update_stats(&mut self, processor: &ReplayProcessor) {
        self.stats
        .iter_mut()
        .for_each(|x| x.update(processor, &self.id));
    }
}

#[typetag::serde]
pub trait Stat {
    fn update(&mut self, processor: &ReplayProcessor, player_id: &PlayerId);
}
