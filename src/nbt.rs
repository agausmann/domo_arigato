use declio::ctx::Len;
use declio::{Decode, Encode};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io;

/// A top-level NBT structure.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Nbt {
    tag: Tag,
    #[declio(with = "string")]
    name: String,
    #[declio(ctx(decode = "*tag"))]
    value: Value,
}

impl Nbt {
    pub fn new(name: String, value: Value) -> Nbt {
        Nbt {
            tag: value.tag(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Tag {
    #[declio(id = "0")]
    End,
    #[declio(id = "1")]
    Byte,
    #[declio(id = "2")]
    Short,
    #[declio(id = "3")]
    Int,
    #[declio(id = "4")]
    Long,
    #[declio(id = "5")]
    Float,
    #[declio(id = "6")]
    Double,
    #[declio(id = "7")]
    ByteArray,
    #[declio(id = "8")]
    String,
    #[declio(id = "9")]
    List,
    #[declio(id = "10")]
    Compound,
    #[declio(id = "11")]
    IntArray,
    #[declio(id = "12")]
    LongArray,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(ctx(decode = "tag: Tag"), id_expr = "tag")]
pub enum Value {
    #[declio(id = "Tag::End")]
    End,
    #[declio(id = "Tag::Byte")]
    Byte(i8),
    #[declio(id = "Tag::Short")]
    Short(i16),
    #[declio(id = "Tag::Int")]
    Int(i32),
    #[declio(id = "Tag::Long")]
    Long(i64),
    #[declio(id = "Tag::Float")]
    Float(f32),
    #[declio(id = "Tag::Double")]
    Double(f64),
    #[declio(id = "Tag::ByteArray")]
    ByteArray(#[declio(with = "array")] Vec<i8>),
    #[declio(id = "Tag::String")]
    String(#[declio(with = "string")] String),
    #[declio(id = "Tag::List")]
    List(#[declio(with = "list")] Vec<Value>),
    #[declio(id = "Tag::Compound")]
    Compound(#[declio(with = "compound")] HashMap<String, Value>),
    #[declio(id = "Tag::IntArray")]
    IntArray(#[declio(with = "array")] Vec<i32>),
    #[declio(id = "Tag::LongArray")]
    LongArray(#[declio(with = "array")] Vec<i64>),
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

mod array {
    use super::*;

    #[derive(Encode, Decode)]
    struct Helper<'a, T>
    where
        T: Encode + Decode + Clone,
    {
        len: i32,
        #[declio(ctx(decode = "Len::try_from(len)?"))]
        arr: Cow<'a, Vec<T>>,
    }

    pub fn encode<W, T>(arr: &Vec<T>, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
        T: Encode + Decode + Clone,
    {
        Helper {
            len: arr.len().try_into()?,
            arr: Cow::Borrowed(arr),
        }
        .encode((), writer)
    }

    pub fn decode<R, T>(_: (), reader: &mut R) -> Result<Vec<T>, declio::Error>
    where
        R: io::Read,
        T: Encode + Decode + Clone,
    {
        Helper::decode((), reader).map(|helper| helper.arr.into_owned())
    }
}

mod string {
    use super::*;

    #[derive(Encode, Decode)]
    struct Helper<'a> {
        len: u16,
        #[declio(ctx(decode = "Len::try_from(len)?"))]
        bytes: Cow<'a, [u8]>,
    }

    pub fn encode<W>(string: &String, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        let bytes = cesu8::to_java_cesu8(&string);
        Helper {
            len: bytes.len().try_into()?,
            bytes,
        }
        .encode((), writer)
    }

    pub fn decode<R>(_: (), reader: &mut R) -> Result<String, declio::Error>
    where
        R: io::Read,
    {
        Helper::decode((), reader).and_then(|helper| {
            cesu8::from_java_cesu8(&helper.bytes)
                .map(Cow::into_owned)
                .map_err(declio::Error::wrap)
        })
    }
}

mod list {
    use super::*;

    #[derive(Encode, Decode)]
    struct Helper<'a> {
        tag: Tag,
        len: i32,
        #[declio(ctx(decode = "(Len::try_from(len)?, *tag)"))]
        list: Cow<'a, Vec<Value>>,
    }

    pub fn encode<W>(list: &Vec<Value>, _: (), writer: &mut W) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        let tag = list.first().map(Value::tag).unwrap_or(Tag::End);
        for elem in list {
            if elem.tag() != tag {
                return Err(declio::Error::new("types of NBT elements do not match"));
            }
        }
        Helper {
            tag,
            len: list.len().try_into()?,
            list: Cow::Borrowed(list),
        }
        .encode((), writer)
    }

    pub fn decode<R>(_: (), reader: &mut R) -> Result<Vec<Value>, declio::Error>
    where
        R: io::Read,
    {
        Helper::decode((), reader).map(|helper| helper.list.into_owned())
    }
}

mod compound {
    use super::*;

    pub fn encode<W>(
        this: &HashMap<String, Value>,
        _: (),
        writer: &mut W,
    ) -> Result<(), declio::Error>
    where
        W: io::Write,
    {
        for (name, value) in this {
            eprintln!("{:?}", value.tag());
            value.tag().encode((), writer)?;
            string::encode(name, (), writer)?;
            value.encode((), writer)?;
        }
        Tag::End.encode((), writer)?;
        Ok(())
    }

    pub fn decode<R>(_: (), reader: &mut R) -> Result<HashMap<String, Value>, declio::Error>
    where
        R: io::Read,
    {
        let mut acc = HashMap::new();
        loop {
            let tag = Tag::decode((), reader)?;
            if tag == Tag::End {
                break;
            }
            let name = string::decode((), reader)?;
            let value = Value::decode(tag, reader)?;
            acc.insert(name, value);
        }
        Ok(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    fn test_bidir(bytes: &[u8], nbt: Nbt) {
        let mut dec_input = bytes;
        let dec_output = Nbt::decode((), &mut dec_input).unwrap();
        assert!(dec_input.is_empty());
        assert_eq!(nbt, dec_output);

        // Comparing write outputs bit-by-bit doesn't work; compounds don't have a guaranteed
        // order. Perform a roundtrip instead.
        let mut enc_output = Vec::new();
        nbt.encode((), &mut enc_output).unwrap();
        eprintln!("{:?}", &enc_output);
        let mut rt_input = enc_output.as_slice();
        let rt_output = Nbt::decode((), &mut rt_input).unwrap();
        assert!(rt_input.is_empty());
        assert_eq!(nbt, rt_output);
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
