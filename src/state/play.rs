use std::net::TcpStream;

pub struct Play<R = TcpStream, W = TcpStream> {
    reader: R,
    writer: W,
}
