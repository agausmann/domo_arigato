use crate::proto::status::{Clientbound, Serverbound, StatusData};
use crate::proto::types::*;
use crate::proto::TransportSession;
use std::io;
use std::net::TcpStream;
use std::time::{Duration, Instant, UNIX_EPOCH};

pub struct Status<R = TcpStream, W = TcpStream> {
    session: TransportSession<R, W>,
}

impl<R, W> Status<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(session: TransportSession<R, W>) -> Self {
        Self { session }
    }

    pub fn query(mut self) -> anyhow::Result<(StatusData, Duration)> {
        let start = Instant::now();
        let timestamp = UNIX_EPOCH.elapsed().expect("timestamp error").as_millis() as Long;
        self.session.write_packet(&Serverbound::Request)?;
        self.session
            .write_packet(&Serverbound::Ping { payload: timestamp })?;

        let data = match self.session.read_packet()? {
            Clientbound::Response { data } => data,
            _ => return Err(anyhow::Error::msg("unexpected packet from server")),
        };
        match self.session.read_packet()? {
            Clientbound::Pong { payload } => {
                if payload != timestamp {
                    return Err(anyhow::Error::msg("ping mismatch"));
                }
            }
            _ => return Err(anyhow::Error::msg("unexpected packet from server")),
        }
        Ok((data, start.elapsed()))
    }
}
