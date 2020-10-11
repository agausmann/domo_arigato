mod handshake;
mod login;
mod play;
mod status;

pub use self::handshake::Handshake;
pub use self::login::Login;
pub use self::play::Play;
pub use self::status::Status;

use std::net::TcpStream;

pub fn connect(host: String, port: u16, version: i32) -> anyhow::Result<Handshake> {
    let reader = TcpStream::connect((host.as_str(), port))?;
    let writer = reader.try_clone()?;
    Ok(Handshake::new(reader, writer, host, port, version))
}
