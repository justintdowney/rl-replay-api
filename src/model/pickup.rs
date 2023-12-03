use std::cell::RefCell;

use boxcars::RigidBody;

const LARGE_BOOST_RADIUS: u32 = 208;
const SMALL_BOOST_RADIUS: u32 = 149;
const SMALL_BOOST_HEIGHT: u32 = 165;
const LARGE_BOOST_HEIGHT: u32 = 168;

const BOOST_COOLDOWN: f32 = 5.00;

#[derive(PartialEq, Clone, Copy)]
pub struct BoostPad {
    pub x: f32,
    pub y: f32,
}

impl BoostPad {
    pub fn new(_x: f32, _y: f32) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}

/*
*   The PickupHandler struct uses internal mutability through Rc<RefCell<T>> to allow for outward mutability from external clients. 
*   This has it's tradeoffs, obviously in complexity and concurrency but for now it will have to suffice as a solution until
*   I can take the time to refactor the code.
*/

#[derive(Default, Clone)]
pub struct PickupHandler {
    small_boost_pads: Vec<BoostPad>,
    large_boost_pads: Vec<BoostPad>,
    disabled_boost_pads: RefCell<Vec<(BoostPad, f32)>>
}

impl PickupHandler {
    pub fn new() -> Self {
        Self {
            small_boost_pads: vec![
                BoostPad::new(0.0, -4240.0),
                BoostPad::new(-1792.0, -4184.0),
                BoostPad::new(1792.0, -4184.0),
                BoostPad::new(-940.0, -3308.07),
                BoostPad::new(940.0, -3308.0),
                BoostPad::new(0.0, -2816.0),
                BoostPad::new(-3584.0, -2484.0),
                BoostPad::new(3584.0, -2484.0),
                BoostPad::new(-1788.0, -2300.0),
                BoostPad::new(1788.0, -2300.0),
                BoostPad::new(-2048.0, -1036.0),
                BoostPad::new(0.0, -1024.0),
                BoostPad::new(2048.0, -1036.0),
                BoostPad::new(-1024.0, 0.0),
                BoostPad::new(1024.0, 0.0),
                BoostPad::new(-2048.0, 1036.0),
                BoostPad::new(0.0, 1024.0),
                BoostPad::new(2048.0, 1036.0),
                BoostPad::new(-1788.0, 2300.0),
                BoostPad::new(1788.0, 2300.0),
                BoostPad::new(-3584.0, 2484.0),
                BoostPad::new(3584.0, 2484.0),
                BoostPad::new(0.0, 2816.0),
                BoostPad::new(-940.0, 3310.0),
                BoostPad::new(940.0, 3308.0),
                BoostPad::new(-1792.0, 4184.0),
                BoostPad::new(1792.0, 4184.0),
                BoostPad::new(0.0, 4240.0)
            ],
            large_boost_pads: vec![
                BoostPad::new(3082.0, 4098.0),
                BoostPad::new(3072.0, -4096.0),
                BoostPad::new(-3072.0, -4096.0),
                BoostPad::new(3584.0, 0.0),
                BoostPad::new(-3584.0, 0.0),
                BoostPad::new(3072.0, 4096.0),
                BoostPad::new(-3072.0, 4096.0)
            ],
            disabled_boost_pads: RefCell::new(Vec::new())
        }
    }

    pub fn try_pickup(&self, pad: BoostPad, time: f32) -> bool {
        // Check for any active pads
        let mut disabled_pads = self.disabled_boost_pads.borrow_mut();
        let pad_disabled = disabled_pads
        .iter()
        .any(|x| x.0 == pad);

        if pad_disabled == false {
            disabled_pads.push((pad, time));
        }

        return !pad_disabled;
    }

    pub fn check_small_pad_collision(&self, rb: &RigidBody) -> Option<&BoostPad> {
        self.small_boost_pads
        .iter()
        .find(|boost_pad| { 
            rb.location.y <= boost_pad.y + SMALL_BOOST_RADIUS as f32
            && rb.location.y >= boost_pad.y - SMALL_BOOST_RADIUS as f32
            && rb.location.x <= boost_pad.x + SMALL_BOOST_RADIUS as f32
            && rb.location.x >= boost_pad.x - SMALL_BOOST_RADIUS as f32
            && rb.location.z <= SMALL_BOOST_HEIGHT as f32
        })
    }

    pub fn check_large_pad_collision(&self, rb: &RigidBody) -> Option<&BoostPad> {
        self.large_boost_pads
        .iter()
        .find(|boost_pad| { 
            rb.location.y <= boost_pad.y + LARGE_BOOST_RADIUS as f32
            && rb.location.y >= boost_pad.y - LARGE_BOOST_RADIUS as f32
            && rb.location.x <= boost_pad.x + LARGE_BOOST_RADIUS as f32
            && rb.location.x >= boost_pad.x - LARGE_BOOST_RADIUS as f32
            && rb.location.z <= LARGE_BOOST_HEIGHT as f32
        })
    }

    pub fn update(&self, current_time: f32) {
        self.disabled_boost_pads.borrow_mut()
        .retain(|(_, delta)| current_time - delta < BOOST_COOLDOWN);       
    }
}