//! Type definitions for 64-bit ELF binaries.

use std::collections::HashSet;

use crate::*;

use crate::segment::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct Segment64 {
    pub header: Phdr64,
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
    pub const SIZE: usize = 0x38;

    // getter
    pub fn get_type(&self) -> segment_type::Type {
        segment_type::Type::from(self.p_type)
    }
    pub fn get_flags(&self) -> HashSet<segment::Flag> {
        let mut mask: Elf64Word = 0b1;
        let mut flags = HashSet::new();
        loop {
            if mask == 0 {
                break;
            }
            if self.p_flags & mask != 0 {
                flags.insert(segment::Flag::from(mask));
            }
            mask <<= 1;
        }

        flags
    }

    // setter
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment;
    ///
    /// let mut phdr : segment::Phdr64 = Default::default();
    /// phdr.set_type(segment::Type::Load);
    ///
    /// assert_eq!(phdr.get_type(), segment::Type::Load);
    /// ```
    pub fn set_type(&mut self, ptype: segment_type::Type) {
        self.p_type = ptype.to_bytes();
    }

    pub fn set_flags<'a, I>(&mut self, flags: I)
    where
        I: Iterator<Item = &'a segment::Flag>,
    {
        for flag in flags {
            self.p_flags = self.p_flags | Into::<Elf64Word>::into(*flag);
        }
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment::Phdr64;
    /// let null_phdr : Phdr64 = Default::default();
    ///
    /// assert_eq!([0].repeat(Phdr64::SIZE), null_phdr.to_le_bytes());
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
