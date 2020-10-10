use crate::nbt::Nbt;
use declio::ctx::Len;
use declio::{Decode, Encode};
use std::convert::{TryFrom, TryInto};
use std::io;
use std::num::TryFromIntError;

pub type Boolean = bool;
pub type Byte = i8;
pub type UByte = u8;
pub type Short = i16;
pub type UShort = u16;
pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
pub type ByteArray = Vec<u8>;

#[derive(Debug, Clone, PartialEq)]
pub struct String(pub std::string::String);

impl From<std::string::String> for String {
    fn from(x: std::string::String) -> Self {
        Self(x)
    }
}

impl From<String> for std::string::String {
    fn from(String(x): String) -> Self {
        x
    }
}

impl Encode for String {
    fn encode<W>(&self, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        VarInt(self.0.len() as i32).encode((), writer)?;
        self.0.as_bytes().encode((), writer)?;
        Ok(())
    }
}

impl Decode for String {
    fn decode<R>(_: (), reader: &mut R) -> Result<Self, declio::Error>
    where
        R: io::Read,
    {
        let len = VarInt::decode((), reader)?.0;
        let bytes = Vec::decode(Len::try_from(len)?, reader)?;
        let string = std::string::String::from_utf8(bytes)?;
        Ok(String(string))
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Chat(pub String);

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VarInt(pub i32);

impl From<i32> for VarInt {
    fn from(x: i32) -> Self {
        Self(x)
    }
}

impl From<VarInt> for i32 {
    fn from(VarInt(x): VarInt) -> Self {
        x
    }
}

impl TryFrom<Len> for VarInt {
    type Error = TryFromIntError;

    fn try_from(len: Len) -> Result<Self, Self::Error> {
        Ok(Self(len.try_into()?))
    }
}

impl TryFrom<VarInt> for Len {
    type Error = TryFromIntError;

    fn try_from(VarInt(x): VarInt) -> Result<Self, Self::Error> {
        x.try_into()
    }
}

impl Encode for VarInt {
    fn encode<W>(&self, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        let mut acc = self.0 as u32;
        loop {
            let mut byte = (acc & 0x7f) as u8;
            acc >>= 7;
            if acc != 0 {
                byte |= 0x80;
            }
            u8::encode(&byte, (), writer)?;

            if acc == 0 {
                break;
            }
        }
        Ok(())
    }
}

impl Decode for VarInt {
    fn decode<R>(_: (), reader: &mut R) -> Result<Self, declio::Error>
    where
        R: io::Read,
    {
        let mut len = 0;
        let mut acc = 0;
        loop {
            if len > 5 {
                return Err(declio::Error::new("VarInt overflow"));
            }
            let byte = u8::decode((), reader)?;
            acc |= ((byte & 0x7f) as i32) << (7 * len);
            len += 1;

            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(Self(acc))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VarLong(pub i64);

impl From<i64> for VarLong {
    fn from(x: i64) -> Self {
        Self(x)
    }
}

impl From<VarLong> for i64 {
    fn from(VarLong(x): VarLong) -> Self {
        x
    }
}

impl Encode for VarLong {
    fn encode<W>(&self, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        let mut acc = self.0 as u64;
        loop {
            let mut byte = (acc & 0x7f) as u8;
            acc >>= 7;
            if acc != 0 {
                byte |= 0x80;
            }
            u8::encode(&byte, (), writer)?;

            if acc == 0 {
                break;
            }
        }
        Ok(())
    }
}

impl Decode for VarLong {
    fn decode<R>(_: (), reader: &mut R) -> Result<Self, declio::Error>
    where
        R: io::Read,
    {
        let mut len = 0;
        let mut acc = 0;
        loop {
            if len > 10 {
                return Err(declio::Error::new("VarInt overflow"));
            }
            let byte = u8::decode((), reader)?;
            acc |= ((byte & 0x7f) as i64) << (7 * len);
            len += 1;

            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(Self(acc))
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "bool")]
pub enum Slot {
    #[declio(id = "true")]
    Present {
        item_id: VarInt,
        item_count: i8,
        nbt: Nbt,
    },
    #[declio(id = "false")]
    NotPresent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub z: i32,
    pub y: i32,
}

impl Encode for Position {
    fn encode<W>(&self, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        let Self { x, y, z } = *self;
        if x >= 2i32.pow(25)
            || x < -2i32.pow(25)
            || z >= 2i32.pow(25)
            || z < -2i32.pow(25)
            || y >= 2i32.pow(11)
            || y < -2i32.pow(11)
        {
            return Err(declio::Error::new("position out of range"));
        }

        // 63    56 55    48 47    40 39    32 31    24 23    16 15     8 7      0
        // xxxxxxxx xxxxxxxx xxxxxxxx xxzzzzzz zzzzzzzz zzzzzzzz zzzzyyyy yyyyyyyy
        let packed = (((x as i64) & 0x3ffffff) << 38)
            | (((z as i64) & 0x3ffffff) << 12)
            | (((y as i64) & 0xfff) << 0);

        i64::encode(&packed, (), writer)
    }
}

impl Decode for Position {
    fn decode<R>(_: (), reader: &mut R) -> Result<Self, declio::Error>
    where
        R: io::Read,
    {
        // 63    56 55    48 47    40 39    32 31    24 23    16 15     8 7      0
        // xxxxxxxx xxxxxxxx xxxxxxxx xxzzzzzz zzzzzzzz zzzzzzzz zzzzyyyy yyyyyyyy
        let packed = i64::decode((), reader)?;

        // sign-extension hack:
        // - shl to align MSB of component with MSB of integer type,
        // - shr (arithmetic) to align LSB of component with LSB of integer type, which will
        // preserve the sign.
        Ok(Position {
            x: ((packed << 0) >> 38) as i32,
            z: ((packed << 26) >> 38) as i32,
            y: ((packed << 52) >> 52) as i32,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Angle(pub u8);

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Uuid(pub u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_varints() {
        let cases = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (2, vec![0x02]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ];

        for (int, bytes) in cases {
            let mut enc_output = Vec::new();
            VarInt(int).encode((), &mut enc_output).unwrap();
            assert_eq!(enc_output, bytes);

            let mut dec_input = bytes.as_slice();
            let dec_output = VarInt::decode((), &mut dec_input).unwrap().0;
            assert_eq!(dec_output, int);
        }
    }

    #[test]
    fn sample_varlongs() {
        let cases = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (2, vec![0x02]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (
                9223372036854775807,
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
            ),
            (
                -1,
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
            (
                -2147483648,
                vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
            (
                -9223372036854775808,
                vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            ),
        ];

        for (int, bytes) in cases {
            let mut enc_output = Vec::new();
            VarLong(int).encode((), &mut enc_output).unwrap();
            assert_eq!(enc_output, bytes);

            let mut dec_input = bytes.as_slice();
            let dec_output = VarLong::decode((), &mut dec_input).unwrap().0;
            assert_eq!(dec_output, int);
        }
    }
}
