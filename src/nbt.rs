use deku::ctx;
use deku::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;

const ENDIAN: ctx::Endian = ctx::Endian::Big;

/// A top-level NBT structure.
#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct Nbt {
    tag_type: Tag,
    #[deku(reader = "read_string(rest)", writer = "write_string(&self.name)")]
    name: String,
    #[deku(ctx = "*tag_type")]
    value: Value,
}

impl Nbt {
    pub fn new(name: String, value: Value) -> Nbt {
        Nbt {
            tag_type: value.tag(),
            name,
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
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
                let (input, end_tag) = Tag::read(input, ())?;
                if end_tag != Tag::End {
                    return Err(DekuError::Unexpected(format!(
                        "expected End tag, found {:?}",
                        end_tag
                    )));
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

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    fn test_bidir(bytes: &[u8], nbt: Nbt) {
        let bits = bytes.view_bits();

        let (rest, nbt_out) = Nbt::read(bits, ()).unwrap();
        assert!(rest.is_empty());
        assert_eq!(nbt, nbt_out);

        // Comparing write outputs bit-by-bit doesn't work; compounds don't have a guaranteed
        // order. Perform a roundtrip instead.
        let bits_out = nbt.write(()).unwrap();
        let (rest, roundtrip) = Nbt::read(&bits_out, ()).unwrap();
        assert!(rest.is_empty());
        assert_eq!(nbt, roundtrip);
    }

    #[test]
    fn test_nbt() {
        test_bidir(
            &include_bytes!("test.nbt")[..],
            Nbt::new(
                "hello world".into(),
                Value::Compound(hashmap![
                    "name".into() => Value::String("Bananrama".into()),
                ]),
            ),
        );
    }

    #[test]
    fn bigtest_nbt() {
        test_bidir(
            &include_bytes!("bigtest.nbt")[..],
            Nbt::new(
                "Level".into(),
                Value::Compound(hashmap![
                    "nested compound test".into() => Value::Compound(hashmap![
                        "egg".into() => Value::Compound(hashmap![
                            "name".into() => Value::String("Eggbert".into()),
                            "value".into() => Value::Float(0.5),
                        ]),
                        "ham".into() => Value::Compound(hashmap![
                            "name".into() => Value::String("Hampus".into()),
                            "value".into() => Value::Float(0.75),
                        ]),
                    ]),
                    "intTest".into() => Value::Int(2147483647),
                    "byteTest".into() => Value::Byte(127),
                    "stringTest".into() => Value::String(
                        "HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".into()
                    ),
                    "listTest (long)".into() => Value::List(vec![
                        Value::Long(11),
                        Value::Long(12),
                        Value::Long(13),
                        Value::Long(14),
                        Value::Long(15),
                    ]),
                    "doubleTest".into() => Value::Double(0.49312871321823148),
                    "floatTest".into() => Value::Float(0.49823147058486938),
                    "longTest".into() => Value::Long(9223372036854775807),
                    "listTest (compound)".into() => Value::List(vec![
                        Value::Compound(hashmap![
                            "created-on".into() => Value::Long(1264099775885),
                            "name".into() => Value::String("Compound tag #0".into()),
                        ]),
                        Value::Compound(hashmap![
                            "created-on".into() => Value::Long(1264099775885),
                            "name".into() => Value::String("Compound tag #1".into()),
                        ]),
                    ]),
                    "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))".into() => Value::ByteArray(vec![
                        0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46,
                        88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54,
                        86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72,
                        94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0,
                        12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38,
                        40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78,
                        80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26,
                        18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84,
                        66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52,
                        24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92,
                        64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70,
                        32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58,
                        10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56,
                        98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64,
                        96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4,
                        36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22,
                        44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50,
                        62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88,
                        90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36,
                        28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94,
                        76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62,
                        34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40,
                        2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28,
                        80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26,
                        68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34,
                        66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52,
                        74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92,
                        14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20,
                        32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58,
                        60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6,
                        98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64,
                        46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4,
                        86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72,
                        44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50,
                        12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38,
                        90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78,
                        30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76,
                        18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84,
                        16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2,
                        24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30,
                        42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68,
                        70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8,
                        10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56,
                        48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14,
                        96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82,
                        54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60,
                        22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48,
                        0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46,
                        88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54,
                        86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72,
                        94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0,
                        12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38,
                        40, 52, 74, 6, 48,
                    ]),
                    "shortTest".into() => Value::Short(32767),
                ]),
            ),
        );
    }
}
