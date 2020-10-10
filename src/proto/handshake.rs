use crate::proto::types::*;
use declio::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Serverbound {
    #[declio(id = "VarInt(0x00)")]
    Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: NextState,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum NextState {
    #[declio(id = "VarInt(1)")]
    Status,
    #[declio(id = "VarInt(2)")]
    Login,
}
