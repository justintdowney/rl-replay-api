pub mod boost;
pub mod location;
pub mod movement;

use crate::payload::Payload;
use subtr_actor::PlayerId;

#[typetag::serde]
pub trait Stat {
    fn update(&mut self, payload: &mut Payload, player_id: &PlayerId);
}
