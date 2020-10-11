use crate::proto::handshake::{NextState, Serverbound};
use crate::proto::write_packet;
use crate::state::{Login, Status};
use std::io;
use std::net::TcpStream;

pub struct Handshake<R = TcpStream, W = TcpStream> {
    reader: R,
    writer: W,
    host: String,
    port: u16,
    version: i32,
}

impl<R, W> Handshake<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(reader: R, writer: W, host: String, port: u16, version: i32) -> Self {
        Self {
            reader,
            writer,
            host,
            port,
            version,
        }
    }

    pub fn status(mut self) -> anyhow::Result<Status<R, W>> {
        write_packet(
            &Serverbound::Handshake {
                protocol_version: self.version.into(),
                server_address: self.host.into(),
                server_port: self.port,
                next_state: NextState::Status,
            },
            &mut self.writer,
        )?;

        Ok(Status::new(self.reader, self.writer))
    }

    pub fn login(mut self) -> anyhow::Result<Login<R, W>> {
        write_packet(
            &Serverbound::Handshake {
                protocol_version: self.version.into(),
                server_address: self.host.into(),
                server_port: self.port.into(),
                next_state: NextState::Login,
            },
            &mut self.writer,
        )?;
        Ok(Login::new(self.reader, self.writer))
    }
}
