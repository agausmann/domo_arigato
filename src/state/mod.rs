mod handshake;
mod login;
mod play;
mod status;

pub use self::handshake::Handshake;
pub use self::login::Login;
pub use self::play::Play;
pub use self::status::Status;

use crate::proto::TransportSession;

pub fn connect(host: String, port: u16, version: i32) -> anyhow::Result<Handshake> {
    Ok(Handshake::new(
        TransportSession::connect(host.as_str(), port)?,
        host,
        port,
        version,
    ))
}
