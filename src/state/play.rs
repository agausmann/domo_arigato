use crate::proto::play::{Clientbound, Serverbound};
use crate::proto::types::Uuid;
use crate::proto::{Peekable, TransportSession};
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

    fn handle_packet(&mut self, packet: &Clientbound) -> anyhow::Result<Option<Event>> {
        match &packet {
            Clientbound::KeepAlive { keepalive_id } => {
                self.session.write_packet(&Serverbound::KeepAlive {
                    keepalive_id: *keepalive_id,
                })?;
            }
            _ => {}
        }
        Ok(None)
    }

    pub fn try_poll(&mut self) -> anyhow::Result<Option<Event>>
    where
        R: Peekable,
    {
        while let Some(packet) = self.session.try_read_packet()? {
            if let Some(event) = self.handle_packet(&packet)? {
                return Ok(Some(event));
            }
        }
        Ok(None)
    }

    pub fn poll(&mut self) -> anyhow::Result<Event> {
        loop {
            let packet = self.session.read_packet()?;
            if let Some(event) = self.handle_packet(&packet)? {
                return Ok(event);
            }
        }
    }
}

pub enum Event {}
