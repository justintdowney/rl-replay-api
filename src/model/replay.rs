use crate::model::player::PlayerData;
use serde::Serialize;

pub const TIME_PER_FRAME: f32 = 1.0 / 30.0;

#[derive(Serialize)]
pub struct ReplayData {
    pub player_data: PlayerData,
}
