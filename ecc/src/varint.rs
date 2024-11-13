/// Varint is shorthand for variable integer, which is a way to encode an
/// integer into bytes that range from 0 to 2^64 – 1.
use std::io::{Cursor, Read, Error};

/// Reads a varint from a cursor
pub fn read_varint(cursor: &mut Cursor<Vec<u8>>) -> Result<u64, Error> {
    let mut buffer = [0u8; 1];
    cursor.read_exact(&mut buffer)?;
    
    match buffer[0] {
        // 0xfd means the next two bytes are the number
        0xfd => {
            let mut buf = [0u8; 2];
            cursor.read_exact(&mut buf)?;
            Ok(u16::from_le_bytes(buf) as u64)
        }
        // 0xfe means the next four bytes are the number
        0xfe => {
            let mut buf = [0u8; 4];
            cursor.read_exact(&mut buf)?;
            Ok(u32::from_le_bytes(buf) as u64)
        }
        // 0xff means the next eight bytes are the number
        0xff => {
            let mut buf = [0u8; 8];
            cursor.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes(buf))
        }
        // anything else is just the integer
        i => Ok(i as u64)
    }
}

pub fn encode_varint(i: u64) -> Vec<u8> {
    if i < 0xfd {
        vec![i as u8]
    } else if i <= 0xffff {
        let mut result = vec![0xfd];
        result.extend_from_slice(&(i as u16).to_le_bytes());
        result
    } else if i <= 0xffffffff {
        let mut result = vec![0xfe];
        result.extend_from_slice(&(i as u32).to_le_bytes());
        result
    } else {
        let mut result = vec![0xff];
        result.extend_from_slice(&i.to_le_bytes());
        result
    }
}
