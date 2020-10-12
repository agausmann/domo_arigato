use crate::proto::play::{Clientbound, Serverbound};
use crate::proto::types::Uuid;
use crate::proto::TransportSession;
use std::io;
use std::net::TcpStream;

pub struct Play<R = TcpStream, W = TcpStream> {
    session: TransportSession<R, W>,
    uuid: Uuid,
    username: String,
}

impl<R, W> Play<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(session: TransportSession<R, W>, uuid: Uuid, username: String) -> Self {
        Self {
            session,
            uuid,
            username,
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            let packet: Clientbound = self.session.read_packet()?;
            match &packet {
                Clientbound::KeepAlive { keepalive_id } => {
                    self.session.write_packet(&Serverbound::KeepAlive {
                        keepalive_id: *keepalive_id,
                    })?;
                }
                _ => {}
            }
        }
    }
}
