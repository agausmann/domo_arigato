use crate::proto::status::{Clientbound, Serverbound, StatusData};
use crate::proto::types::*;
use crate::proto::{read_packet, write_packet};
use std::io;
use std::net::TcpStream;
use std::time::{Duration, Instant, UNIX_EPOCH};

pub struct Status<R = TcpStream, W = TcpStream> {
    reader: R,
    writer: W,
}

impl<R, W> Status<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    pub fn query(mut self) -> anyhow::Result<(StatusData, Duration)> {
        let start = Instant::now();
        let timestamp = UNIX_EPOCH.elapsed().expect("timestamp error").as_millis() as Long;
        write_packet(&Serverbound::Request, &mut self.writer)?;
        write_packet(&Serverbound::Ping { payload: timestamp }, &mut self.writer)?;

        let data = match read_packet(&mut self.reader)? {
            Clientbound::Response { data } => data,
            _ => return Err(anyhow::Error::msg("unexpected packet from server")),
        };
        match read_packet(&mut self.reader)? {
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
