//! Type definitions for 64-bit ELF binaries.

use crate::*;

use crate::segment::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Segment64 {
    pub header: Phdr64,
}

impl Segment64 {
    pub fn new(header: Phdr64) -> Self {
        Self { header }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phdr64 {
    /// Segment type
    pub p_type: Elf64Word,

    /// Segment flags
    pub p_flags: Elf64Word,

    /// Segment file offset
    pub p_offset: Elf64Off,

    /// Segment virtual address
    pub p_vaddr: Elf64Addr,

    /// Segment physical address
    pub p_paddr: Elf64Addr,

    /// Segment size in file
    pub p_filesz: Elf64Xword,

    /// Segment size in memory
    pub p_memsz: Elf64Xword,

    /// Segment alignment
    pub p_align: Elf64Xword,
}

impl Default for Phdr64 {
    fn default() -> Self {
        Self {
            p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }
}

impl Phdr64 {
    pub fn size() -> Elf64Half {
        0x38
    }

    // getter
    pub fn get_type(&self) -> segment_type::TYPE {
        segment_type::TYPE::from(self.p_type)
    }

    // setter
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment;
    ///
    /// let mut phdr : segment::Phdr64 = Default::default();
    /// phdr.set_type(segment::TYPE::LOAD);
    ///
    /// assert_eq!(phdr.get_type(), segment::TYPE::LOAD);
    /// ```
    pub fn set_type(&mut self, ptype: segment_type::TYPE) {
        self.p_type = ptype.to_bytes();
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment::Phdr64;
    /// let null_phdr : Phdr64 = Default::default();
    ///
    /// assert_eq!([0].repeat(Phdr64::size() as usize), null_phdr.to_le_bytes());
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
