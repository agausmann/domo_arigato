pub mod handshake;
pub mod login;
pub mod play;
pub mod status;
pub mod types;

use self::types::VarInt;
use declio::{Decode, Encode};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::convert::TryInto;
use std::io::{self, Write};

pub fn read_packet<T, R>(mut reader: R) -> Result<T, declio::Error>
where
    T: Decode,
    R: io::Read,
{
    let len = VarInt::decode((), &mut reader)?.0;
    let mut reader = reader.take(len.try_into()?);
    T::decode((), &mut reader)
}

pub fn write_packet<T, W>(packet: &T, mut writer: W) -> Result<(), declio::Error>
where
    T: Encode,
    W: io::Write,
{
    let mut buf = Vec::new();
    packet.encode((), &mut buf)?;
    VarInt(buf.len().try_into()?).encode((), &mut writer)?;
    writer.write_all(&buf)?;
    Ok(())
}

pub fn read_compressed_packet<T, R>(mut reader: R) -> Result<T, declio::Error>
where
    T: Decode,
    R: io::Read,
{
    let len = VarInt::decode((), &mut reader)?.0;
    let mut reader = reader.take(len.try_into()?);
    let uncompressed_len = VarInt::decode((), &mut reader)?.0;
    if uncompressed_len == 0 {
        T::decode((), &mut reader)
    } else {
        let mut reader = ZlibDecoder::new(reader);
        T::decode((), &mut reader)
    }
}

pub fn write_compressed_packet<T, W>(
    packet: &T,
    mut writer: W,
    threshold: usize,
) -> Result<(), declio::Error>
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
    Ok(())
}
