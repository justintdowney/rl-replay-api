pub use crate::model::*;

pub mod stat_collector;
pub mod model;

#[cfg(test)]
mod tests {
    use boxcars::{RigidBody, Vector3f, Quaternion};

    use crate::pickup::PickupHandler;

    #[test]
    fn detect_player_on_boost_pad() {
        let pickup_map = PickupHandler::new();
        let rb = RigidBody {
            sleeping: false,
            location: Vector3f { x: -940.0, y: 0.0, z: 3310.0},
            rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            linear_velocity: None,
            angular_velocity: None
        };
        let result = pickup_map.check_large_pad_collision(&rb);
        assert!(result != None);
    }
}