use crate::proto::types::*;
use crate::util::{Greedy, LengthPrefix};
use declio::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Clientbound {
    #[declio(id = "VarInt(0x00)")]
    Disconnect { reason: Chat },

    #[declio(id = "VarInt(0x01)")]
    EncryptionRequest {
        server_id: String,
        #[declio(with = "LengthPrefix::<VarInt>")]
        public_key_der: ByteArray,
        #[declio(with = "LengthPrefix::<VarInt>")]
        verify_token: ByteArray,
    },

    #[declio(id = "VarInt(0x02)")]
    LoginSuccess { uuid: Uuid, username: String },

    #[declio(id = "VarInt(0x03)")]
    SetCompression { threshold: VarInt },

    #[declio(id = "VarInt(0x04)")]
    LoginPluginRequest {
        message_id: VarInt,
        channel: Identifier,
        #[declio(with = "Greedy")]
        data: ByteArray,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Serverbound {
    #[declio(id = "VarInt(0x00)")]
    LoginStart { name: String },

    #[declio(id = "VarInt(0x01)")]
    EncryptionResponse {
        #[declio(with = "LengthPrefix::<VarInt>")]
        shared_secret: ByteArray,
        #[declio(with = "LengthPrefix::<VarInt>")]
        verify_token: ByteArray,
    },

    #[declio(id = "VarInt(0x02)")]
    LoginPluginResponse {
        message_id: VarInt,
        success: Boolean,
        #[declio(with = "Greedy")]
        data: ByteArray,
    },
}
