pub mod boost;
pub mod location;
pub mod movement;

use crate::stat_collector::{PickupHandler, PlayerPayload};
use subtr_actor::{PlayerId};

#[typetag::serde]
pub trait Stat {
    fn update(
        &mut self,
        player_payload: &PlayerPayload,
        pickup_handler: &mut PickupHandler,
        player_id: &PlayerId,
    );
}
