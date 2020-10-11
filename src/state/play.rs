use crate::proto::TransportSession;
use std::net::TcpStream;

pub struct Play<R = TcpStream, W = TcpStream> {
    session: TransportSession<R, W>,
    uuid: String,
    username: String,
}

impl<R, W> Play<R, W> {
    pub fn new(session: TransportSession<R, W>, uuid: String, username: String) -> Self {
        Self {
            session,
            uuid,
            username,
        }
    }
}
