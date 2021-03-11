//! Type definitions for 32-bit ELF binaries.

use std::collections::HashSet;

use crate::*;

use serde::{Deserialize, Serialize};

use super::StrTabEntry;

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
/// section's contents
pub enum Contents32 {
    /// almost section's data
    Raw(Vec<u8>),
    /// String Table
    StrTab(Vec<StrTabEntry>),
    /// symbol table's representation
    Symbols(Vec<symbol::Symbol32>),
    /// relocation symbol table's representation
    RelaSymbols(Vec<relocation::Rela32>),
    /// dynamic information's representation
    Dynamics(Vec<dynamic::Dyn32>),
}

#[derive(Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Section32 {
    pub name: String,
    pub header: Shdr32,
    pub contents: Contents32,
}

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Shdr32 {
    /// Section name, index in string tbl
    pub sh_name: Elf32Word,
    /// Type of section
    pub sh_type: Elf32Word,
    /// Miscellaneous section attributes
    pub sh_flags: Elf32Word,
    ///  Section virtual addr at execution
    pub sh_addr: Elf32Addr,
    /// Section file offset
    pub sh_offset: Elf32Off,
    /// Size of section in bytes
    pub sh_size: Elf32Word,
    /// Index of another section
    pub sh_link: Elf32Word,
    /// Additional section information
    pub sh_info: Elf32Word,
    /// Section alignment
    pub sh_addralign: Elf32Word,
    /// Entry size if section holds table
    pub sh_entsize: Elf32Word,
}

/// A `Shdr32` builder
///
/// # Examples
///
/// ```
/// use elf_utilities::section;
/// let shdr: section::Shdr32 = section::ShdrPreparation32::default()
///            .ty(section::Type::ProgBits)
///            .flags(vec![section::Flag::Alloc, section::Flag::Write].iter())
///            .into();
///
/// assert_eq!(section::Type::ProgBits, shdr.get_type());
/// assert!(shdr.get_flags().contains(&section::Flag::Alloc));
/// assert!(shdr.get_flags().contains(&section::Flag::Write));
/// ```
#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(C)]
pub struct ShdrPreparation32 {
    /// Type of section
    pub sh_type: section::Type,
    /// Miscellaneous section attributes
    pub sh_flags: Elf32Word,
    /// Index of another section
    pub sh_link: Elf32Word,
    /// Additional section information
    pub sh_info: Elf32Word,
    /// Section alignment
    pub sh_addralign: Elf32Word,
}

impl Default for Contents32 {
    fn default() -> Self {
        Contents32::Raw(Default::default())
    }
}

impl Contents32 {
    pub fn size(&self) -> usize {
        match self {
            Contents32::Raw(bytes) => bytes.len(),
            Contents32::StrTab(strs) => {
                // ELFの文字列テーブルは null-byte + (name + null-byte) * n という形状に
                let total_len: usize = strs.iter().map(|s| s.v.len()).sum();
                total_len + strs.len() + 1
            }
            Contents32::Symbols(syms) => symbol::Symbol32::SIZE * syms.len(),
            Contents32::RelaSymbols(rela_syms) => {
                relocation::Rela32::SIZE as usize * rela_syms.len()
            }
            Contents32::Dynamics(dyn_info) => dynamic::Dyn32::SIZE * dyn_info.len(),
        }
    }
}

impl Default for Shdr32 {
    fn default() -> Self {
        Self {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }
}

impl Section32 {
    pub fn new(name: String, hdr: ShdrPreparation32, contents: Contents32) -> Self {
        Self {
            contents,
            name,
            header: hdr.into(),
        }
    }

    /// create binary without header
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match &self.contents {
            Contents32::Raw(bytes) => bytes.clone(),
            Contents32::StrTab(strs) => {
                // ELFの文字列テーブルは null-byte + (name + null-byte) * n という形状に
                // それに合うようにバイト列を構築.
                let mut string_table: Vec<u8> = vec![0x00];

                for st in strs {
                    for byte in st.v.as_bytes() {
                        string_table.push(*byte);
                    }
                    string_table.push(0x00);
                }

                string_table
            }
            Contents32::Symbols(syms) => {
                let mut bytes = Vec::new();
                for sym in syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
            Contents32::RelaSymbols(rela_syms) => {
                let mut bytes = Vec::new();
                for sym in rela_syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
            Contents32::Dynamics(dynamics) => {
                let mut bytes = Vec::new();
                for sym in dynamics.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
        }
    }

    pub fn new_null_section() -> Self {
        Default::default()
    }
}

#[allow(dead_code)]
impl Shdr32 {
    pub const SIZE: usize = 40;
    // getter
    pub fn get_type(&self) -> section::Type {
        section::Type::from(self.sh_type)
    }
    pub fn get_flags(&self) -> HashSet<section::Flag> {
        let mut mask: Elf32Word = 0b1;
        let mut flags = HashSet::new();
        loop {
            if mask == 0 {
                break;
            }
            if self.sh_flags & mask != 0 {
                flags.insert(section::Flag::from(mask));
            }
            mask <<= 1;
        }

        flags
    }
    // setter
    pub fn set_type(&mut self, ty: section::Type) {
        self.sh_type = ty.into();
    }
    pub fn set_flags<'a, I>(&mut self, flags: I)
    where
        I: Iterator<Item = &'a section::Flag>,
    {
        for flag in flags {
            self.sh_flags = self.sh_flags | Into::<Elf32Word>::into(*flag);
        }
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::section::Shdr32;
    /// let null_sct : Shdr32 = Default::default();
    ///
    /// assert_eq!([0].repeat(Shdr32::SIZE), null_sct.to_le_bytes());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

impl ShdrPreparation32 {
    pub fn ty(mut self, t: section::Type) -> Self {
        self.sh_type = t;
        self
    }

    pub fn flags<'a, I>(mut self, flags: I) -> Self
    where
        I: Iterator<Item = &'a section::Flag>,
    {
        for flag in flags {
            self.sh_flags |= Into::<Elf32Word>::into(*flag);
        }

        self
    }

    pub fn link(mut self, link: Elf32Word) -> Self {
        self.sh_link = link;
        self
    }
    pub fn info(mut self, info: Elf32Word) -> Self {
        self.sh_info = info;
        self
    }
}

impl Default for ShdrPreparation32 {
    fn default() -> Self {
        Self {
            sh_type: section::Type::Null,
            sh_flags: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
        }
    }
}
impl Into<Shdr32> for ShdrPreparation32 {
    fn into(self) -> Shdr32 {
        Shdr32 {
            sh_name: 0,
            sh_type: self.sh_type.into(),
            sh_flags: self.sh_flags,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: self.sh_link,
            sh_info: self.sh_info,
            sh_addralign: self.sh_addralign,
            sh_entsize: 0,
        }
    }
}

#[cfg(test)]
mod elf32_tests {
    use super::*;

    #[test]
    fn section32_test() {
        let sct = Section32::new_null_section();

        assert_eq!(vec![0x00; Shdr32::SIZE], sct.header.to_le_bytes(),);

        assert_eq!(Vec::new() as Vec<u8>, sct.to_le_bytes(),);
    }
}
