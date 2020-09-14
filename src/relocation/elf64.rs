use crate::*;
use serde::{Deserialize, Serialize};

pub const R_X86_64_PC32: Elf64Xword = 2;
pub const R_X86_64_PLT32: Elf64Xword = 4;
pub const R_X86_64_32: Elf64Xword = 10;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Rela64 {
    r_offset: Elf64Addr,
    r_info: Elf64Xword,
    r_addend: Elf64Sxword,
}

impl Default for Rela64 {
    fn default() -> Self {
        Self {
            r_offset: 0,
            r_info: 0,
            r_addend: 0,
        }
    }
}

#[allow(dead_code)]
impl Rela64 {
    pub fn size() -> Elf64Xword {
        24
    }
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
    /// assert_eq!([0].repeat(Rela64::size() as usize), null_rel.to_le_bytes());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
