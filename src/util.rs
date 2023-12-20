use std::collections::HashMap;

use boxcars::RigidBody;
use subtr_actor::PlayerId;

use crate::{
    constants::{
        DEFENSIVE_THIRD_TEAM_ONE, DEFENSIVE_THIRD_TEAM_ZERO, OFFENSIVE_THIRD_TEAM_ONE,
        OFFENSIVE_THIRD_TEAM_ZERO,
    },
    model::player::Team,
    payload::PlayerFrame,
};

pub enum GameMode {
    Standard,
}

pub(crate) fn is_defensive_half(team: &Team, player_rb: &RigidBody) -> bool {
    (team == &Team::Zero && player_rb.location.y > 0.0)
        || (team == &Team::One && player_rb.location.y <= 0.0)
}

pub(crate) fn is_offensive_half(team: &Team, player_rb: &RigidBody) -> bool {
    (team == &Team::Zero && player_rb.location.y < 0.0)
        || (team == &Team::One && player_rb.location.y >= 0.0)
}

pub(crate) fn is_defensive_third(team: &Team, player_rb: &RigidBody) -> bool {
    match team {
        Team::Zero => player_rb.location.y > DEFENSIVE_THIRD_TEAM_ZERO,
        Team::One => player_rb.location.y < DEFENSIVE_THIRD_TEAM_ONE,
    }
}

pub(crate) fn is_offensive_third(team: &Team, player_rb: &RigidBody) -> bool {
    match team {
        Team::Zero => player_rb.location.y < OFFENSIVE_THIRD_TEAM_ZERO,
        Team::One => player_rb.location.y > OFFENSIVE_THIRD_TEAM_ONE,
    }
}

pub(crate) fn is_neutral_third(team: &Team, player_rb: &RigidBody) -> bool {
    !(is_defensive_third(team, player_rb) && is_offensive_third(team, player_rb))
}

pub(crate) fn is_farthest_back(
    team: &Team,
    target_player: &PlayerId,
    player_frames: &HashMap<PlayerId, PlayerFrame>,
) -> bool {
    let comparison_player = player_frames
        .keys()
        .zip(
            player_frames
                .values()
                .filter(|frame| frame.team == *team)
                .filter_map(|frame| frame.rigid_body),
        )
        .min_by(|x, y| f32::abs(x.1.location.y).total_cmp(&f32::abs(y.1.location.y)));

    if let Some(valid_player) = comparison_player {
        valid_player.0 == target_player
    } else {
        false
    }
}

pub(crate) fn is_farthest_forward(
    team: &Team,
    target_player: &PlayerId,
    player_frames: &HashMap<PlayerId, PlayerFrame>,
) -> bool {
    let comparison_player = player_frames
        .keys()
        .zip(
            player_frames
                .values()
                .filter(|frame| frame.team == *team)
                .filter_map(|frame| frame.rigid_body),
        )
        .max_by(|x, y| f32::abs(x.1.location.y).total_cmp(&f32::abs(y.1.location.y)));

    if let Some(valid_player) = comparison_player {
        valid_player.0 == target_player
    } else {
        false
    }
}
