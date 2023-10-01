use serde_with::*;
use subtr_actor::{BallFrame, BallData};
use boxcars::RigidBody;

#[derive(serde::Serialize)]
#[serde(remote = "BallFrame", untagged)]
pub enum BallFrameDef {
    Empty,
    Data { rigid_body: RigidBody },
}

impl SerializeAs<BallFrame> for BallFrameDef {
    fn serialize_as<S>(value: &BallFrame, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {  
        BallFrameDef::serialize(value, serializer)
    }
}


