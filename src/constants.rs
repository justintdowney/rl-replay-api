use crate::util::BoostPad;

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
pub const STANDARD_FIELD_LENGTH: u32 = 5120;
pub const STANDARD_FIELD_WIDTH: u32 = 4096;
pub const STANDARD_GOAL_WIDTH_HALF: u32 = 893;
pub const BALL_SIZE: f32 = 92.75;
pub const BALL_GROUND_HEIGHT: u32 = 95;
pub const CAR_GROUND: u32 = 20;
pub const GOAL_HEIGHT: u32 = 840;
pub const MAP_Y: u32 = 10240;
pub const MAP_X: u32 = 8192;
pub const MAP_Z: u32 = 2044;
pub const GOAL_X: u32 = 1786;
pub const GOAL_Y: u32 = 880;
pub const GOAL_Z: f32 = 642.775;
pub const MAP_THIRD: f32 = MAP_Y as f32 / 3.0;
pub const MAP_SIXTH: f32 = MAP_THIRD / 2.0;
pub const BACK_WALL: f32 = 5120.0;

/// Small boost pad definitions by in-game location
pub static SMALL_BOOST_PADS: [BoostPad; 28] = [
    BoostPad {
        id: 0,
        x: 0.0,
        y: -4240.0,
    },
    BoostPad {
        id: 1,
        x: -1792.0,
        y: -4184.0,
    },
    BoostPad {
        id: 2,
        x: 1792.0,
        y: -4184.0,
    },
    BoostPad {
        id: 3,
        x: -940.0,
        y: -3308.07,
    },
    BoostPad {
        id: 4,
        x: 940.0,
        y: -3308.0,
    },
    BoostPad {
        id: 5,
        x: 0.0,
        y: -2816.0,
    },
    BoostPad {
        id: 6,
        x: -3584.0,
        y: -2484.0,
    },
    BoostPad {
        id: 7,
        x: 3584.0,
        y: -2484.0,
    },
    BoostPad {
        id: 8,
        x: -1788.0,
        y: -2300.0,
    },
    BoostPad {
        id: 9,
        x: 1788.0,
        y: -2300.0,
    },
    BoostPad {
        id: 10,
        x: -2048.0,
        y: -1036.0,
    },
    BoostPad {
        id: 11,
        x: 0.0,
        y: -1024.0,
    },
    BoostPad {
        id: 12,
        x: 2048.0,
        y: -1036.0,
    },
    BoostPad {
        id: 13,
        x: -1024.0,
        y: 0.0,
    },
    BoostPad {
        id: 14,
        x: 1024.0,
        y: 0.0,
    },
    BoostPad {
        id: 15,
        x: -2048.0,
        y: 1036.0,
    },
    BoostPad {
        id: 16,
        x: 0.0,
        y: 1024.0,
    },
    BoostPad {
        id: 17,
        x: 2048.0,
        y: 1036.0,
    },
    BoostPad {
        id: 18,
        x: -1788.0,
        y: 2300.0,
    },
    BoostPad {
        id: 19,
        x: 1788.0,
        y: 2300.0,
    },
    BoostPad {
        id: 20,
        x: -3584.0,
        y: 2484.0,
    },
    BoostPad {
        id: 21,
        x: 3584.0,
        y: 2484.0,
    },
    BoostPad {
        id: 22,
        x: 0.0,
        y: 2816.0,
    },
    BoostPad {
        id: 23,
        x: -940.0,
        y: 3310.0,
    },
    BoostPad {
        id: 24,
        x: 940.0,
        y: 3308.0,
    },
    BoostPad {
        id: 25,
        x: -1792.0,
        y: 4184.0,
    },
    BoostPad {
        id: 26,
        x: 1792.0,
        y: 4184.0,
    },
    BoostPad {
        id: 27,
        x: 0.0,
        y: 4240.0,
    },
];

/// Large boost pad definitions by in-game location
pub static LARGE_BOOST_PADS: [BoostPad; 6] = [
    BoostPad {
        id: 28,
        x: 3072.0,
        y: -4096.0,
    },
    BoostPad {
        id: 29,
        x: -3072.0,
        y: -4096.0,
    },
    BoostPad {
        id: 30,
        x: 3584.0,
        y: 0.0,
    },
    BoostPad {
        id: 31,
        x: -3584.0,
        y: 0.0,
    },
    BoostPad {
        id: 32,
        x: 3072.0,
        y: 4096.0,
    },
    BoostPad {
        id: 33,
        x: -3072.0,
        y: 4096.0,
    },
];
