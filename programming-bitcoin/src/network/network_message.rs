use std::io::{Cursor, Error};

pub trait NetworkMessage where Self: Sized {
    fn command(&self) -> &str;
    fn serialize(&self) -> Vec<u8>;
    fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, Error>;
}
