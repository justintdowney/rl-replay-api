use std::hash::{Hash, Hasher};

/// `BoostPad` models the in-game position of boost pads, with a distinct id for identification.
#[derive(Clone, Debug)]
pub struct BoostPad {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

impl BoostPad {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        Self { id, x, y }
    }
}

impl Hash for BoostPad {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for BoostPad {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BoostPad {}

#[derive(PartialEq)]
pub enum BoostPickupEvent {
    None,
    Small,
    Large,
}

pub enum GameMode {
    Standard,
}
