mod tests {
    use crate::constants::{LARGE_BOOST_PADS, SMALL_BOOST_PADS};
    use crate::stat_collector::PickupMap;
    use crate::util::BoostPadSize;
    use boxcars::{Quaternion, RigidBody, Vector3f};

    #[test]
    fn detect_small_boost_pickup() {
        let mut pickup_map = PickupMap::new();
        let rb = RigidBody {
            sleeping: false,
            location: Vector3f {
                x: SMALL_BOOST_PADS[0].x,
                y: SMALL_BOOST_PADS[0].y,
                z: 0.0,
            },
            rotation: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            linear_velocity: None,
            angular_velocity: None,
        };
        let result = pickup_map.try_pickup(&rb);
        assert!(result == Some(BoostPadSize::Small));
    }

    #[test]
    fn detect_large_boost_pickup() {
        let mut pickup_map = PickupMap::new();
        let rb = RigidBody {
            sleeping: false,
            location: Vector3f {
                x: LARGE_BOOST_PADS[0].x,
                y: LARGE_BOOST_PADS[0].y,
                z: 0.0,
            },
            rotation: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            linear_velocity: None,
            angular_velocity: None,
        };
        let result = pickup_map.try_pickup(&rb);
        assert!(result == Some(BoostPadSize::Large));
    }

    #[test]
    fn detect_no_boost_pickup() {
        let mut pickup_map = PickupMap::new();
        let rb = RigidBody {
            sleeping: false,
            location: Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            linear_velocity: None,
            angular_velocity: None,
        };
        let result = pickup_map.try_pickup(&rb);
        assert!(result == None);
    }
}
