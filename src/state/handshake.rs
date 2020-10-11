use crate::proto::handshake::{NextState, Serverbound};
use crate::proto::TransportSession;
use crate::state::{Login, Status};
use std::io;
use std::net::TcpStream;

pub struct Handshake<R = TcpStream, W = TcpStream> {
    session: TransportSession<R, W>,
    host: String,
    port: u16,
    version: i32,
}

impl<R, W> Handshake<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(session: TransportSession<R, W>, host: String, port: u16, version: i32) -> Self {
        Self {
            session,
            host,
            port,
            version,
        }
    }

    pub fn status(mut self) -> anyhow::Result<Status<R, W>> {
        self.session.write_packet(&Serverbound::Handshake {
            protocol_version: self.version.into(),
            server_address: self.host.into(),
            server_port: self.port,
            next_state: NextState::Status,
        })?;

        Ok(Status::new(self.session))
    }

    pub fn login(mut self) -> anyhow::Result<Login<R, W>> {
        self.session.write_packet(&Serverbound::Handshake {
            protocol_version: self.version.into(),
            server_address: self.host.into(),
            server_port: self.port.into(),
            next_state: NextState::Login,
        })?;
        Ok(Login::new(self.session))
    }
}
