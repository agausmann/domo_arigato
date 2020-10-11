use crate::proto::login::{Clientbound, Serverbound};
use crate::proto::TransportSession;
use crate::state::Play;
use rsa::{PaddingScheme, PublicKey, RSAPublicKey};
use serde_json::json;
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fmt::Write;
use std::io;
use std::net::TcpStream;

pub struct Login<R = TcpStream, W = TcpStream> {
    session: TransportSession<R, W>,
}

impl<R, W> Login<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(session: TransportSession<R, W>) -> Self {
        Self { session }
    }

    pub fn login(
        mut self,
        name: String,
        uuid: String,
        access_token: String,
    ) -> anyhow::Result<Play<R, W>> {
        self.session
            .write_packet(&Serverbound::LoginStart { name: name.into() })?;

        loop {
            match self.session.read_packet()? {
                Clientbound::Disconnect { reason } => {
                    return Err(anyhow::Error::msg(format!("{:?}", reason)));
                }
                Clientbound::EncryptionRequest {
                    server_id,
                    public_key_der,
                    verify_token,
                } => {
                    let public_key_pem = format!(
                        "-----BEGIN PUBLIC KEY-----{}-----END PUBLIC KEY-----",
                        base64::encode(&public_key_der),
                    );
                    let public_key = RSAPublicKey::from_pkcs1(public_key_pem.as_bytes())?;
                    let shared_secret: [u8; 16] = rand::random();

                    let shared_secret_encrypted = public_key.encrypt(
                        &mut rand::thread_rng(),
                        PaddingScheme::PKCS1v15Encrypt,
                        &shared_secret,
                    )?;
                    let verify_token_encrypted = public_key.encrypt(
                        &mut rand::thread_rng(),
                        PaddingScheme::PKCS1v15Encrypt,
                        &verify_token,
                    )?;

                    let mut hash = Sha1::new()
                        .chain(&server_id.0)
                        .chain(&shared_secret)
                        .chain(&public_key_der)
                        .finalize();

                    let mut hexdigest = String::with_capacity(41);
                    // twos complement hexdigest, because why not?
                    // let's make this protocol as convoluted as possible.
                    if hash[0] & 0x80 != 0 {
                        hexdigest.push('-');
                        let mut carry = true;
                        for byte in hash.as_mut_slice().iter_mut().rev() {
                            *byte = !*byte;
                            if carry {
                                let (next_byte, next_carry) = byte.overflowing_add(1);
                                *byte = next_byte;
                                carry = next_carry;
                            }
                        }
                    }
                    for byte in hash.as_slice() {
                        write!(hexdigest, "{:2x}", byte).unwrap();
                    }

                    let client = reqwest::blocking::Client::new();
                    client
                        .post("https://sessionserver.mojang.com/session/minecraft/join")
                        .json(&json!({
                            "accessToken": access_token,
                            "selectedProfile": uuid,
                            "serverId": hexdigest,
                        }))
                        .send()?
                        .error_for_status()?;

                    self.session
                        .write_packet(&Serverbound::EncryptionResponse {
                            shared_secret: shared_secret_encrypted,
                            verify_token: verify_token_encrypted,
                        })?;

                    self.session.enable_encryption(shared_secret)?;
                }
                Clientbound::LoginSuccess { uuid, username } => {
                    return Ok(Play::new(self.session, uuid.into(), username.into()));
                }
                Clientbound::SetCompression { threshold } => {
                    let threshold = if threshold.0 < 0 {
                        None
                    } else {
                        Some(threshold.0.try_into()?)
                    };
                    self.session.set_compression_threshold(threshold);
                }
                Clientbound::LoginPluginRequest { message_id, .. } => {
                    self.session
                        .write_packet(&Serverbound::LoginPluginResponse {
                            message_id,
                            success: false,
                            data: vec![],
                        })?;
                }
            }
        }
    }
}
