use std::io::{Read, Error};

pub fn read_varint<R: Read>(stream: &mut R) -> Result<u64, Error> {
    let mut buffer = [0u8; 1];
    stream.read_exact(&mut buffer)?;
    
    match buffer[0] {
        0xfd => {
            let mut buf = [0u8; 2];
            stream.read_exact(&mut buf)?;
            Ok(u16::from_le_bytes(buf) as u64)
        }
        0xfe => {
            let mut buf = [0u8; 4];
            stream.read_exact(&mut buf)?;
            Ok(u32::from_le_bytes(buf) as u64)
        }
        0xff => {
            let mut buf = [0u8; 8];
            stream.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes(buf))
        }
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
