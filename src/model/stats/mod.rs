pub mod boost;
pub mod location;
pub mod movement;

use crate::payload::Payload;
use subtr_actor::PlayerId;

// Define a trait for stats
#[typetag::serde]
pub trait Stat {
    // Define the update function that returns the specific PayloadResult
    fn update(&mut self, payload: &mut Payload, player_id: &PlayerId);
}

