//! Type definitions for 32-bit ELF binaries.

use crate::*;

use crate::segment::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct Segment32 {
    pub header: Phdr32,
}
impl Segment for Segment32 {
    type Header = Phdr32;

    fn new(header: Phdr32) -> Self {
        Self { header }
    }

    fn header_deserialize(
        buf: &[u8],
        header_start: usize,
    ) -> Result<Phdr32, Box<dyn std::error::Error>> {
        match bincode::deserialize(&buf[header_start..]) {
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }

    fn header_size() -> usize {
        Phdr32::size() as usize
    }
}

impl Segment32 {
    pub fn new(header: Phdr32) -> Self {
        Self { header }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phdr32 {
    /// Segment type
    pub p_type: Elf32Word,

    /// Segment file offset
    pub p_offset: Elf32Off,

    /// Segment virtual address
    pub p_vaddr: Elf32Addr,

    /// Segment physical address
    pub p_paddr: Elf32Addr,

    /// Segment size in file
    pub p_filesz: Elf32Word,

    /// Segment size in memory
    pub p_memsz: Elf32Word,

    /// Segment flags
    pub p_flags: Elf32Word,

    /// Segment alignment
    pub p_align: Elf32Word,
}

impl Default for Phdr32 {
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

impl Phdr32 {
    pub fn size() -> Elf32Half {
        0x20
    }

    // getter
    pub fn get_type(&self) -> segment_type::Type {
        segment_type::Type::from(self.p_type)
    }

    // setter
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment;
    ///
    /// let mut phdr : segment::Phdr32 = Default::default();
    /// phdr.set_type(segment::Type::Load);
    ///
    /// assert_eq!(phdr.get_type(), segment::Type::Load);
    /// ```
    pub fn set_type(&mut self, ptype: segment_type::Type) {
        self.p_type = ptype.to_bytes();
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment::Phdr32;
    /// let null_phdr : Phdr32 = Default::default();
    ///
    /// assert_eq!([0].repeat(Phdr32::size() as usize), null_phdr.to_le_bytes());
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
