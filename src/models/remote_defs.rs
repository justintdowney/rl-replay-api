use serde_as::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "BallFrame")]
pub enum BallFrameDef {
    Empty,
    Data { rigid_body: boxcars::RigidBody },
}

impl SerializeAs<BallFrame> for BallFrameDef {
    fn serialize_as<S>(value: &BallFrame, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {  
        BallFrameDef::serialize(value, serializer)
    }
}