pub mod boost;
pub mod location;
pub mod movement;

use crate::stat_collector::{PickupHandler, PositionHandler};
use subtr_actor::{PlayerId, ReplayProcessor};

#[typetag::serde]
pub trait Stat {
    fn update(
        &mut self,
        processor: &ReplayProcessor,
        pickup_handler: &mut PickupHandler,
        position_handler: &PositionHandler,
        player_id: &PlayerId,
    );
}
