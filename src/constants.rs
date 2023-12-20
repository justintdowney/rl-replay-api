/// Boost related constants
pub const LARGE_BOOST_RADIUS: f32 = 208.0;
pub const SMALL_BOOST_RADIUS: f32 = 149.0;
pub const SMALL_BOOST_HEIGHT: f32 = 165.0;
pub const LARGE_BOOST_HEIGHT: f32 = 168.0;
pub const SMALL_BOOST_PICKUP_AMOUNT: f32 = 30.6;
pub const LARGE_BOOST_PICKUP_AMOUNT: f32 = 255.0;
pub const BOOST_COOLDOWN: f32 = 5.00;
pub const BOOST_PER_FRAME: f32 = 1.12;
pub const STARTING_BOOST_VALUE: f32 = 84.15;

/// Geometry related constants
pub const CAR_HEIGHT_ON_GROUND: f32 = 20.0;
pub const STANDARD_GOAL_WIDTH_HALF: f32 = 893.0;
pub const BALL_SIZE: f32 = 92.75;
pub const BALL_GROUND_HEIGHT: f32 = 95.0;
pub const CAR_GROUND: f32 = 20.0;
pub const GOAL_HEIGHT: f32 = 840.0;
pub const MAP_Y: f32 = 10240.0;
pub const MAP_X: f32 = 8192.0;
pub const MAP_Z: f32 = 2044.0;
pub const GOAL_X: f32 = 1786.0;
pub const GOAL_Y: f32 = 880.0;
pub const GOAL_Z: f32 = 642.775;
pub const MAP_THIRD: f32 = MAP_Y as f32 / 3.0;
pub const BACK_WALL: f32 = 5120.0;

pub const MAP_CENTER: f32 = 0.0;

// Team Zero's defensive third is negative
pub const DEFENSIVE_THIRD_TEAM_ZERO: f32 = MAP_CENTER - MAP_Y / 3.0;
pub const OFFENSIVE_THIRD_TEAM_ZERO: f32 = MAP_CENTER + MAP_Y / 3.0;

// Team One's defensive third is positive
pub const DEFENSIVE_THIRD_TEAM_ONE: f32 = MAP_CENTER + MAP_Y / 3.0;
pub const OFFENSIVE_THIRD_TEAM_ONE: f32 = MAP_CENTER - MAP_Y / 3.0;
