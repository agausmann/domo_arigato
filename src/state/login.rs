use std::io;
use std::net::TcpStream;

pub struct Login<R = TcpStream, W = TcpStream> {
    reader: R,
    writer: W,
}

impl<R, W> Login<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}
