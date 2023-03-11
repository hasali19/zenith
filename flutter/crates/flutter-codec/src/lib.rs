use std::collections::BTreeMap;
use std::io;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

type ReadCursor<'a> = std::io::Cursor<&'a [u8]>;

#[repr(u8)]
enum EncodedType {
    Null = 0,
    True,
    False,
    Int32,
    Int64,
    LargeInt,
    Float64,
    String,
    UInt8List,
    Int32List,
    Int64List,
    Float64List,
    List,
    Map,
    Float32List,
}

impl From<u8> for EncodedType {
    fn from(value: u8) -> Self {
        use EncodedType::*;
        match value {
            0 => Null,
            1 => True,
            2 => False,
            3 => Int32,
            4 => Int64,
            5 => LargeInt,
            6 => Float64,
            7 => String,
            8 => UInt8List,
            9 => Int32List,
            10 => Int64List,
            11 => Float64List,
            12 => List,
            13 => Map,
            14 => Float32List,
            _ => panic!("invalid EncodedType: {value}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EncodableValue<'a> {
    Null,
    Bool(bool),
    String(&'a str),
    Map(BTreeMap<EncodableValue<'a>, EncodableValue<'a>>),
}

impl<'a> EncodableValue<'a> {
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&'a str> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<EncodableValue<'a>, EncodableValue<'a>>> {
        if let Self::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

fn read_size(cursor: &mut ReadCursor) -> io::Result<u32> {
    match cursor.read_u8()? {
        254 => cursor.read_u16::<NativeEndian>().map(u32::from),
        255 => cursor.read_u32::<NativeEndian>(),
        v => Ok(v.into()),
    }
}

fn read_string<'a>(cursor: &mut ReadCursor<'a>) -> io::Result<&'a str> {
    let size = read_size(cursor)?;
    let buf = &cursor.get_ref()[cursor.position() as usize..][..size as usize];
    cursor.set_position(cursor.position() + size as u64);
    std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn read_map<'a>(
    cursor: &mut ReadCursor<'a>,
) -> io::Result<BTreeMap<EncodableValue<'a>, EncodableValue<'a>>> {
    let size = read_size(cursor)?;
    let mut map = BTreeMap::new();
    for _ in 0..size {
        let key = read_value(cursor)?;
        let value = read_value(cursor)?;
        map.insert(key, value);
    }
    Ok(map)
}

pub fn read_value<'a>(cursor: &mut ReadCursor<'a>) -> io::Result<EncodableValue<'a>> {
    let encoded_type = EncodedType::from(cursor.read_u8()?);
    match encoded_type {
        EncodedType::Null => Ok(EncodableValue::Null),
        EncodedType::True => Ok(EncodableValue::Bool(true)),
        EncodedType::False => Ok(EncodableValue::Bool(false)),
        EncodedType::Int32 => todo!(),
        EncodedType::Int64 => todo!(),
        EncodedType::LargeInt => todo!(),
        EncodedType::Float64 => todo!(),
        EncodedType::String => Ok(EncodableValue::String(read_string(cursor)?)),
        EncodedType::UInt8List => todo!(),
        EncodedType::Int32List => todo!(),
        EncodedType::Int64List => todo!(),
        EncodedType::Float64List => todo!(),
        EncodedType::List => todo!(),
        EncodedType::Map => Ok(EncodableValue::Map(read_map(cursor)?)),
        EncodedType::Float32List => todo!(),
    }
}

pub fn write_value(mut w: impl io::Write, value: &EncodableValue) -> io::Result<()> {
    match value {
        EncodableValue::Null => {
            w.write_u8(EncodedType::Null as u8)?;
        }
        EncodableValue::Bool(_) => todo!(),
        EncodableValue::String(_) => todo!(),
        EncodableValue::Map(_) => todo!(),
    }

    Ok(())
}
