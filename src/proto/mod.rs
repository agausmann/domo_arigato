pub mod types;

use self::types::VarInt;
use std::convert::TryInto;
use std::{io, mem};

pub trait Serialize {
    type Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write;
}

pub trait Deserialize: Sized {
    type Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read;
}

impl Serialize for bool {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        u8::from(*self).serialize(writer)
    }
}

impl Deserialize for bool {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        u8::deserialize(reader).and_then(|x| match x {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected value found for boolean",
            )),
        })
    }
}

macro_rules! impl_for_int {
    ($($t:ty)*) => {$(
        impl Serialize for $t {
            type Error = io::Error;

            fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
            where
                W: io::Write,
            {
                writer.write_all(&self.to_be_bytes())
            }
        }

        impl Deserialize for $t {
            type Error = io::Error;

            fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
            where
                R: io::Read,
            {
                let mut buf = [0; mem::size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                Ok(Self::from_be_bytes(buf))
            }
        }
    )*};
}

impl_for_int! {
    i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 f32 f64
}

impl Serialize for String {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        let len: i32 = self
            .len()
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        VarInt(len).serialize(writer)?;
        Ok(())
    }
}

impl Deserialize for String {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        let len: usize = VarInt::deserialize(reader)?
            .0
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;

        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
