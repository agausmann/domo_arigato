use crate::proto::types::*;
use declio::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Clientbound {
    #[declio(id = "VarInt(0x00)")]
    Response { data: StatusData },

    #[declio(id = "VarInt(0x01)")]
    Pong { payload: Long },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Serverbound {
    #[declio(id = "VarInt(0x00)")]
    Request,

    #[declio(id = "VarInt(0x01)")]
    Ping { payload: Long },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusData {
    pub version: Version,
    pub players: Players,
    pub description: Chat,
    pub favicon: std::string::String,
}

impl_declio_from_json!(StatusData);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    name: std::string::String,
    protocol: Int,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<Player>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: std::string::String,
    pub id: std::string::String,
}
