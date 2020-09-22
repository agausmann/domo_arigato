use crate::proto::{Deserialize, Serialize};
use std::io;

/// A variable-length encoding for signed 32-bit integer.
pub struct VarInt(pub i32);

impl Serialize for VarInt {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        let mut buffer = [0; 5];
        let mut len = 0;
        let mut acc = self.0 as u32;

        while acc != 0 {
            buffer[len] = (acc & 0x7f) as u8;
            acc >>= 7;
            if acc != 0 {
                buffer[len] |= 0x80;
            }
            len += 1;
        }

        writer.write_all(&buffer[..len])
    }
}

impl Deserialize for VarInt {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        let mut buffer = [0xff; 1];
        let mut len = 0;
        let mut acc: u32 = 0;

        while buffer[0] & 0x80 != 0 {
            reader.read_exact(&mut buffer)?;

            let shifted =
                ((buffer[0] & 0x7f) as u32)
                    .checked_shl(7 * len)
                    .ok_or(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "VarInt input overflow",
                    ))?;

            acc |= shifted;

            len += 1;
            if len > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt input overflow",
                ));
            }
        }
        Ok(Self(acc as i32))
    }
}

/// A variable length encoding for signed 64-bit integers.
pub struct VarLong(pub i64);

impl Serialize for VarLong {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        let mut buffer = [0; 10];
        let mut len = 0;
        let mut acc = self.0 as u64;

        while acc != 0 {
            buffer[len] = (acc & 0x7f) as u8;
            acc >>= 7;
            if acc != 0 {
                buffer[len] |= 0x80;
            }
            len += 1;
        }

        writer.write_all(&buffer[..len])
    }
}

impl Deserialize for VarLong {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        let mut buffer = [0xff; 1];
        let mut len = 0;
        let mut acc: u64 = 0;

        while buffer[0] & 0x80 != 0 {
            reader.read_exact(&mut buffer)?;

            let shifted =
                ((buffer[0] & 0x7f) as u64)
                    .checked_shl(7 * len)
                    .ok_or(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "VarLong input overflow",
                    ))?;

            acc |= shifted;

            len += 1;
            if len > 10 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarLong input overflow",
                ));
            }
        }
        Ok(Self(acc as i64))
    }
}

/// A chat string with a maximum length of 32767 bytes.
pub struct Chat(pub String);

impl Serialize for Chat {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        if self.0.len() <= 32767 {
            self.0.serialize(writer)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Chat string is too long",
            ))
        }
    }
}

impl Deserialize for Chat {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        let string = String::deserialize(reader)?;
        if string.len() <= 32767 {
            Ok(Self(string))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Chat string is too long",
            ))
        }
    }
}

/// An identifier string with a max length of 32767 bytes.
pub struct Identifier(pub String);

impl Serialize for Identifier {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        if self.0.len() <= 32767 {
            self.0.serialize(writer)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Identifier string is too long",
            ))
        }
    }
}

impl Deserialize for Identifier {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        let string = String::deserialize(reader)?;
        if string.len() <= 32767 {
            Ok(Self(string))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Identifier string is too long",
            ))
        }
    }
}

/// Integer block positions, with x, y, and z being 26-, 12-, and 26- bit signed integers,
/// respectively.
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Serialize for Position {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        if self.x >= 2i32.pow(25)
            || self.x < -(2i32.pow(25))
            || self.y >= 2i32.pow(11)
            || self.y < -(2i32.pow(11))
            || self.z >= 2i32.pow(25)
            || self.z < -(2i32.pow(25))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Position out of range",
            ));
        }

        let out = (((self.x as u64) & 0x3FFFFFF) << 38)
            | (((self.z as u64) & 0x3FFFFFF) << 12)
            | ((self.y as u64) & 0xFFF);
        out.serialize(writer)
    }
}

impl Deserialize for Position {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        let inp = u64::deserialize(reader)?;
        // Shifting of components is done in two parts:
        //
        // - Before casting to i32, shift the value so its MSB (sign bit) is aligned with the
        //   32-bit MSB.
        //
        // - After casting to i32, shift to align LSB, which will also sign-extend the value.
        Ok(Self {
            x: ((inp >> 32) as i32) >> 6,
            y: ((inp << 20) as i32) >> 20,
            z: ((inp >> 6) as i32) >> 6,
        })
    }
}

/// A rotation angle, in steps of 1/256 of a full turn.
pub struct Angle(pub u8);

impl Serialize for Angle {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        self.0.serialize(writer)
    }
}

impl Deserialize for Angle {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        u8::deserialize(reader).map(Self)
    }
}

/// A [UUID](http://en.wikipedia.org/wiki/Universally_unique_identifier).
pub struct Uuid(pub u128);

impl Serialize for Uuid {
    type Error = io::Error;

    fn serialize<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: io::Write,
    {
        self.0.serialize(writer)
    }
}

impl Deserialize for Uuid {
    type Error = io::Error;

    fn deserialize<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: io::Read,
    {
        u128::deserialize(reader).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    fn roundtrip<T>(val: &T) -> T
    where
        T: Serialize + Deserialize,
        <T as Serialize>::Error: fmt::Debug,
        <T as Deserialize>::Error: fmt::Debug,
    {
        let mut buf = Vec::new();
        val.serialize(&mut buf).expect("serialization failed");
        dbg!(&buf);
        T::deserialize(&mut buf.as_slice()).expect("deserialization failed")
    }

    #[test]
    fn varint_roundtrip() {
        let original = VarInt(-1272584588);
        let roundtrip = roundtrip(&original);
        assert_eq!(original.0, roundtrip.0);
    }

    #[test]
    fn varlong_roundtrip() {
        let original = VarLong(5706279124732675577);
        let roundtrip = roundtrip(&original);
        assert_eq!(original.0, roundtrip.0);
    }

    #[test]
    fn chat_roundtrip() {
        let original = Chat("hello world".to_string());
        let roundtrip = roundtrip(&original);
        assert_eq!(original.0, roundtrip.0);
    }

    #[test]
    fn identifier_roundtrip() {
        let original = Identifier("minecraft:stone".to_string());
        let roundtrip = roundtrip(&original);
        assert_eq!(original.0, roundtrip.0);
    }

    #[test]
    fn position_roundtrip() {
        let original = Position {
            x: 14449243,
            y: -920,
            z: 11968197,
        };
        let roundtrip = roundtrip(&original);
        assert_eq!(original.x, roundtrip.x);
        assert_eq!(original.y, roundtrip.y);
        assert_eq!(original.z, roundtrip.z);
    }

    #[test]
    fn angle_roundtrip() {
        let original = Angle(218);
        let roundtrip = roundtrip(&original);
        assert_eq!(original.0, roundtrip.0);
    }

    #[test]
    fn uuid_roundtrip() {
        let original = Uuid(32920761669734660208107371540327435677);
        let roundtrip = roundtrip(&original);
        assert_eq!(original.0, roundtrip.0);
    }
}
