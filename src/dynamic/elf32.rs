use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Dyn32 {
    /// dynamic entry type
    pub d_tag: Elf32Sword,
    /// value
    pub d_un: Elf32Word,
}

impl Dyn32 {
    pub const SIZE: usize = 0x8;
    pub fn get_type(&self) -> dynamic::EntryType {
        dynamic::EntryType::from(self.d_tag as i64)
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(buf: &[u8], start: usize) -> Result<Self, Box<dyn std::error::Error>> {
        // bincode::ErrorKindをトレイトオブジェクトとするため,この冗長な書き方が必要
        match bincode::deserialize(&buf[start..]) {
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }
}
