use deku::ctx;
use deku::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;

const ENDIAN: ctx::Endian = ctx::Endian::Big;

/// A top-level NBT structure.
#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Nbt {
    tag_type: Tag,
    #[deku(reader = "read_string(rest)", writer = "write_string(&self.name)")]
    name: String,
    #[deku(ctx = "*tag_type")]
    value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big", id_type = "u8")]
pub enum Tag {
    #[deku(id = "0")]
    End,
    #[deku(id = "1")]
    Byte,
    #[deku(id = "2")]
    Short,
    #[deku(id = "3")]
    Int,
    #[deku(id = "4")]
    Long,
    #[deku(id = "5")]
    Float,
    #[deku(id = "6")]
    Double,
    #[deku(id = "7")]
    ByteArray,
    #[deku(id = "8")]
    String,
    #[deku(id = "9")]
    List,
    #[deku(id = "10")]
    Compound,
    #[deku(id = "11")]
    IntArray,
    #[deku(id = "12")]
    LongArray,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(HashMap<String, Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    pub fn tag(&self) -> Tag {
        match self {
            Self::End => Tag::End,
            Self::Byte(..) => Tag::Byte,
            Self::Short(..) => Tag::Short,
            Self::Int(..) => Tag::Int,
            Self::Long(..) => Tag::Long,
            Self::Float(..) => Tag::Float,
            Self::Double(..) => Tag::Double,
            Self::ByteArray(..) => Tag::ByteArray,
            Self::String(..) => Tag::String,
            Self::List(..) => Tag::List,
            Self::Compound(..) => Tag::Compound,
            Self::IntArray(..) => Tag::IntArray,
            Self::LongArray(..) => Tag::LongArray,
        }
    }
}

impl DekuRead<Tag> for Value {
    fn read(
        input: &BitSlice<Msb0, u8>,
        type_tag: Tag,
    ) -> Result<(&BitSlice<Msb0, u8>, Self), DekuError> {
        match type_tag {
            Tag::End => Ok((input, Self::End)),
            Tag::Byte => DekuRead::read(input, ENDIAN).map(|(r, v)| (r, Self::Byte(v))),
            Tag::Short => DekuRead::read(input, ENDIAN).map(|(r, v)| (r, Self::Short(v))),
            Tag::Int => DekuRead::read(input, ENDIAN).map(|(r, v)| (r, Self::Int(v))),
            Tag::Long => DekuRead::read(input, ENDIAN).map(|(r, v)| (r, Self::Long(v))),
            Tag::Float => DekuRead::read(input, ENDIAN).map(|(r, v)| (r, Self::Float(v))),
            Tag::Double => DekuRead::read(input, ENDIAN).map(|(r, v)| (r, Self::Double(v))),
            Tag::ByteArray => {
                let (input, len) = i32::read(input, ENDIAN)?;
                let len: usize = len.try_into()?;
                DekuRead::read(input, (len.into(), ENDIAN)).map(|(r, v)| (r, Self::ByteArray(v)))
            }
            Tag::String => {
                let (input, len) = u16::read(input, ENDIAN)?;
                let len: usize = len.try_into()?;
                let (input, bytes) = Vec::read(input, (len.into(), ENDIAN))?;
                let string = cesu8::from_java_cesu8(&bytes)
                    .map_err(|e| DekuError::Parse(e.to_string()))?
                    .into_owned();
                Ok((input, Self::String(string)))
            }
            Tag::List => {
                let (input, type_tag) = Tag::read(input, ())?;
                let (input, len) = i32::read(input, ENDIAN)?;
                let len: usize = len.try_into()?;
                DekuRead::read(input, (len.into(), type_tag)).map(|(r, v)| (r, Self::List(v)))
            }
            Tag::Compound => {
                let mut input = input;
                let mut compound = HashMap::new();
                loop {
                    let (entry_input, type_tag) = Tag::read(input, ())?;
                    if type_tag == Tag::End {
                        break;
                    }
                    let (entry_input, name) = read_string(entry_input)?;
                    let (entry_input, value) = Self::read(entry_input, type_tag)?;
                    input = entry_input;
                    compound.insert(name, value);
                }
                Ok((input, Self::Compound(compound)))
            }
            Tag::IntArray => {
                let (input, len) = i32::read(input, ENDIAN)?;
                let len: usize = len.try_into()?;
                DekuRead::read(input, (len.into(), ENDIAN)).map(|(r, v)| (r, Self::IntArray(v)))
            }
            Tag::LongArray => {
                let (input, len) = i32::read(input, ENDIAN)?;
                let len: usize = len.try_into()?;
                DekuRead::read(input, (len.into(), ENDIAN)).map(|(r, v)| (r, Self::LongArray(v)))
            }
        }
    }
}

impl DekuWrite<Tag> for Value {
    fn write(&self, tag_type: Tag) -> Result<BitVec<Msb0, u8>, DekuError> {
        assert!(tag_type == self.tag());
        self.write(())
    }
}

impl DekuWrite for Value {
    fn write(&self, _: ()) -> Result<BitVec<Msb0, u8>, DekuError> {
        match self {
            Self::End => Ok(BitVec::new()),
            Self::Byte(v) => v.write(ENDIAN),
            Self::Short(v) => v.write(ENDIAN),
            Self::Int(v) => v.write(ENDIAN),
            Self::Long(v) => v.write(ENDIAN),
            Self::Float(v) => v.write(ENDIAN),
            Self::Double(v) => v.write(ENDIAN),
            Self::ByteArray(v) => {
                let len: i32 = v.len().try_into()?;
                let mut acc = BitVec::new();
                acc.extend(len.write(ENDIAN)?);
                acc.extend(v.write(ENDIAN)?);
                Ok(acc)
            }
            Self::String(v) => write_string(v),
            Self::List(v) => {
                let type_tag = v.first().map(Self::tag).unwrap_or(Tag::End);
                let len: i32 = v.len().try_into()?;
                if v.iter().any(|val| val.tag() != type_tag) {
                    return Err(DekuError::InvalidParam("list type mismatch".to_string()));
                }
                let mut acc = BitVec::new();
                acc.extend(type_tag.write(())?);
                acc.extend(len.write(ENDIAN)?);
                acc.extend(v.write(())?);
                Ok(acc)
            }
            Self::Compound(v) => {
                let mut acc = BitVec::new();
                for (name, value) in v {
                    let type_tag = value.tag();
                    acc.extend(type_tag.write(())?);
                    acc.extend(write_string(name)?);
                    acc.extend(value.write(())?);
                }
                acc.extend(Tag::End.write(())?);
                Ok(acc)
            }
            Self::IntArray(v) => {
                let len: i32 = v.len().try_into()?;
                let mut acc = BitVec::new();
                acc.extend(len.write(ENDIAN)?);
                acc.extend(v.write(ENDIAN)?);
                Ok(acc)
            }
            Self::LongArray(v) => {
                let len: i32 = v.len().try_into()?;
                let mut acc = BitVec::new();
                acc.extend(len.write(ENDIAN)?);
                acc.extend(v.write(ENDIAN)?);
                Ok(acc)
            }
        }
    }
}

fn read_string(input: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
    let (input, len) = u16::read(input, ENDIAN)?;
    let len: usize = len.try_into()?;
    let (input, bytes) = Vec::read(input, (len.into(), ENDIAN))?;
    let string = cesu8::from_java_cesu8(&bytes)
        .map_err(|e| DekuError::Parse(e.to_string()))?
        .into_owned();
    Ok((input, string))
}

fn write_string(s: &String) -> Result<BitVec<Msb0, u8>, DekuError> {
    let len: u16 = s.len().try_into()?;
    let bytes = cesu8::to_java_cesu8(&s).into_owned();
    let mut acc = BitVec::new();
    acc.extend(len.write(ENDIAN)?);
    acc.extend(bytes.write(ENDIAN)?);
    Ok(acc)
}
