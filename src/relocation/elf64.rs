use crate::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize,
)]
#[repr(C)]
pub struct Rela64 {
    /// Location at which to apply the action
    r_offset: Elf64Addr,
    /// index and type of relocation
    r_info: Elf64Xword,
    /// Constant addend used to compute value
    r_addend: Elf64Sxword,
}

#[allow(dead_code)]
impl Rela64 {
    pub const SIZE: Elf64Xword = 24;
    pub fn get_sym(&self) -> Elf64Xword {
        self.r_info >> 32
    }
    pub fn get_type(&self) -> Elf64Xword {
        self.r_info & 0xffffffff
    }

    pub fn get_offset(&self) -> Elf64Addr {
        self.r_offset
    }
    pub fn get_info(&self) -> Elf64Xword {
        self.r_info
    }
    pub fn get_addend(&self) -> Elf64Sxword {
        self.r_addend
    }

    pub fn set_addend(&mut self, addend: Elf64Sxword) {
        self.r_addend = addend;
    }
    pub fn set_offset(&mut self, offset: Elf64Addr) {
        self.r_offset = offset;
    }
    pub fn set_info(&mut self, info: Elf64Xword) {
        self.r_info = info;
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::relocation::Rela64;
    /// let null_rel : Rela64 = Default::default();
    ///
    /// assert_eq!([0].repeat(Rela64::SIZE as usize), null_rel.to_le_bytes());
    /// ```
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
