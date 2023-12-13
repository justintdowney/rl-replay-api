use std::collections::HashMap;

// Define a trait for extensible payload data
trait PayloadData {
    // Define methods or fields common to all payload data types
}

// Example implementation of a payload data type
struct PlayerFrameData {
    pub rigid_body: Option<boxcars::RigidBody>,
    pub boost_amount: Option<f32>,
    pub boost_active: bool,
    pub jump_active: bool,
    pub double_jump_active: bool,
    pub dodge_active: bool,
}

impl PayloadData for PlayerFrameData {}

// Enum to represent different payload data types
enum PayloadDataType {
    Player(PlayerFrameData),
    // Add other payload data types here
}

// Payload struct containing a vector of payload data
struct Payload {
    data: Vec<PayloadDataType>,
    // Add any other context-related data here
}

// Example implementation of Payload with extensible data types
fn main() {
    let payload = Payload {
        data: vec![
            PayloadDataType::Player(PlayerFrameData { rigid_body: (), boost_amount: (), boost_active: (), jump_active: (), double_jump_active: (), dodge_active: () } {
            ),
            // Add other payload data types here
        ],
    };

    // Use the payload data
    for payload_data in payload.data {
        match payload_data {
            PayloadDataType::Example(example_data) => {
                // Handle ExamplePayloadData
                println!("Example Data: {}", example_data.data);
            } // Handle other payload data types here
        }
    }
}
