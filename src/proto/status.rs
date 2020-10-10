use crate::proto::types::*;
use declio::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Clientbound {
    #[declio(id = "VarInt(0x00)")]
    Response { json: String },

    #[declio(id = "VarInt(0x01)")]
    Pong { payload: Long },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Serverbound {
    #[declio(id = "VarInt(0x00)")]
    Status,

    #[declio(id = "VarInt(0x01)")]
    Ping { payload: Long },
}
