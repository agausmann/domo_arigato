pub mod handshake;
pub mod login;
pub mod play;
pub mod status;
pub mod types;

use self::types::VarInt;
use aes::Aes128;
use cfb8::stream_cipher::{NewStreamCipher, StreamCipher};
use cfb8::Cfb8;
use declio::{Decode, Encode};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::net::TcpStream;

type AesCfb8 = Cfb8<Aes128>;

struct EncryptedReader<R> {
    inner: R,
    encryption: Option<AesCfb8>,
}

impl<R> EncryptedReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            encryption: None,
        }
    }

    fn set_cipher(&mut self, cipher: AesCfb8) -> anyhow::Result<()> {
        if self.encryption.is_some() {
            return Err(anyhow::Error::msg("encryption is already enabled"));
        }
        self.encryption = Some(cipher);
        Ok(())
    }
}

impl<R> io::Read for EncryptedReader<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.inner.read(buf)?;
        if let Some(cipher) = &mut self.encryption {
            cipher.decrypt(&mut buf[..len]);
        }
        Ok(len)
    }
}

struct EncryptedWriter<W> {
    inner: W,
    encryption: Option<AesCfb8>,
    buffer: Vec<u8>,
}

impl<W> EncryptedWriter<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            encryption: None,
            buffer: Vec::new(),
        }
    }

    fn set_cipher(&mut self, cipher: AesCfb8) -> anyhow::Result<()> {
        if self.encryption.is_some() {
            return Err(anyhow::Error::msg("encryption is already enabled"));
        }
        self.encryption = Some(cipher);
        Ok(())
    }
}

impl<W> io::Write for EncryptedWriter<W>
where
    W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(cipher) = &mut self.encryption {
            self.buffer.clear();
            self.buffer.extend(buf);
            cipher.encrypt(&mut self.buffer);
            self.inner.write_all(&self.buffer)?;
            Ok(buf.len())
        } else {
            self.inner.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub struct TransportSession<R = TcpStream, W = TcpStream> {
    reader: EncryptedReader<R>,
    writer: EncryptedWriter<W>,
    compression_threshold: Option<usize>,
}

impl TransportSession {
    pub fn connect(host: &str, port: u16) -> anyhow::Result<Self> {
        let reader = TcpStream::connect((host, port))?;
        let writer = reader.try_clone()?;
        Ok(Self::new(reader, writer))
    }
}

impl<R, W> TransportSession<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: EncryptedReader::new(reader),
            writer: EncryptedWriter::new(writer),
            compression_threshold: None,
        }
    }

    pub fn write_packet<T>(&mut self, packet: &T) -> anyhow::Result<()>
    where
        W: io::Write,
        T: Encode,
    {
        if let Some(threshold) = self.compression_threshold {
            write_compressed_packet(packet, &mut self.writer, threshold)
        } else {
            write_packet(packet, &mut self.writer)
        }
    }

    pub fn read_packet<T>(&mut self) -> anyhow::Result<T>
    where
        R: io::Read,
        T: Decode,
    {
        if let Some(_threshold) = self.compression_threshold {
            read_compressed_packet(&mut self.reader)
        } else {
            read_packet(&mut self.reader)
        }
    }

    pub fn enable_encryption(&mut self, shared_secret: [u8; 16]) -> anyhow::Result<()> {
        self.reader
            .set_cipher(AesCfb8::new_var(&shared_secret, &shared_secret).unwrap())?;
        self.writer
            .set_cipher(AesCfb8::new_var(&shared_secret, &shared_secret).unwrap())?;
        Ok(())
    }

    pub fn set_compression_threshold(&mut self, threshold: Option<usize>) {
        self.compression_threshold = threshold;
    }
}

pub fn read_packet<T, R>(mut reader: R) -> anyhow::Result<T>
where
    T: Decode,
    R: io::Read,
{
    let len = VarInt::decode((), &mut reader)?.0;
    let mut reader = reader.take(len.try_into()?);
    let packet = T::decode((), &mut reader)?;

    let remaining_bytes = reader.bytes().filter_map(Result::ok).count();
    debug_assert!(remaining_bytes == 0, "not all bytes read by packet parser");
    Ok(packet)
}

pub fn write_packet<T, W>(packet: &T, mut writer: W) -> anyhow::Result<()>
where
    T: Encode,
    W: io::Write,
{
    let mut buf = Vec::new();
    packet.encode((), &mut buf)?;
    VarInt(buf.len().try_into()?).encode((), &mut writer)?;
    writer.write_all(&buf)?;
    writer.flush()?;
    Ok(())
}

pub fn read_compressed_packet<T, R>(mut reader: R) -> anyhow::Result<T>
where
    T: Decode,
    R: io::Read,
{
    let len = VarInt::decode((), &mut reader)?.0;
    let mut reader = reader.take(len.try_into()?);
    let uncompressed_len = VarInt::decode((), &mut reader)?.0;
    let packet;
    if uncompressed_len == 0 {
        packet = T::decode((), &mut reader)?;
    } else {
        let mut reader = ZlibDecoder::new(&mut reader);
        packet = T::decode((), &mut reader)?;
    }
    let remaining_bytes = reader.bytes().filter_map(Result::ok).count();
    debug_assert!(remaining_bytes == 0, "not all bytes read by packet parser");
    Ok(packet)
}

pub fn write_compressed_packet<T, W>(
    packet: &T,
    mut writer: W,
    threshold: usize,
) -> anyhow::Result<()>
where
    T: Encode,
    W: io::Write,
{
    let mut data_buf = Vec::new();
    packet.encode((), &mut data_buf)?;

    let mut header_buf = Vec::new();
    if data_buf.len() < threshold {
        VarInt(0).encode((), &mut header_buf)?;
    } else {
        VarInt(data_buf.len().try_into()?).encode((), &mut header_buf)?;

        // Compress the data buffer, and replace it with the compressed version.
        let mut compressed_writer = ZlibEncoder::new(Vec::new(), Compression::default());
        compressed_writer.write_all(&data_buf)?;
        data_buf = compressed_writer.finish()?;
    }

    VarInt((header_buf.len() + data_buf.len()).try_into()?).encode((), &mut writer)?;
    writer.write_all(&header_buf)?;
    writer.write_all(&data_buf)?;
    writer.flush()?;
    Ok(())
}
