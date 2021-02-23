//! Type definitions for 64-bit ELF binaries.

use std::collections::HashSet;

use crate::section;
use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Contents64 {
    /// almost section's data
    Raw(Vec<u8>),
    /// symbol table
    Symbols(Vec<symbol::Symbol64>),
    /// relocation symbol table
    RelaSymbols(Vec<relocation::Rela64>),
    /// dynamic information
    Dynamics(Vec<dynamic::Dyn64>),
}

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Section64 {
    pub name: String,
    pub header: Shdr64,

    pub contents: Contents64,
}

#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Shdr64 {
    /// Section name, index in string tbl
    pub sh_name: Elf64Word,
    /// Type of section
    pub sh_type: Elf64Word,
    /// Miscellaneous section attributes
    pub sh_flags: Elf64Xword,
    ///  Section virtual addr at execution
    pub sh_addr: Elf64Addr,
    /// Section file offset
    pub sh_offset: Elf64Off,
    /// Size of section in bytes
    pub sh_size: Elf64Xword,
    /// Index of another section
    pub sh_link: Elf64Word,
    /// Additional section information
    pub sh_info: Elf64Word,
    /// Section alignment
    pub sh_addralign: Elf64Xword,
    /// Entry size if section holds table
    pub sh_entsize: Elf64Xword,
}

/// A `Shdr64` builder
///
/// # Examples
///
/// ```
/// use elf_utilities::section;
/// let shdr: section::Shdr64 = section::ShdrPreparation64::default()
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
pub struct ShdrPreparation64 {
    /// Type of section
    pub sh_type: section::Type,
    /// Miscellaneous section attributes
    pub sh_flags: Elf64Xword,
    /// Index of another section
    pub sh_link: Elf64Word,
    /// Additional section information
    pub sh_info: Elf64Word,
    /// Section alignment
    pub sh_addralign: Elf64Xword,
}

impl Default for Shdr64 {
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

#[allow(dead_code)]
impl Shdr64 {
    pub const SIZE: usize = 0x40;

    // getter
    pub fn get_type(&self) -> section::Type {
        section::Type::from(self.sh_type)
    }
    pub fn get_flags(&self) -> HashSet<section::Flag> {
        let mut mask: Elf64Xword = 0b1;
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
    pub fn set_flags<I>(&mut self, flags: I)
    where
        I: Iterator<Item = section::Flag>,
    {
        for flag in flags {
            self.sh_flags = self.sh_flags | Into::<Elf64Xword>::into(flag);
        }
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::section::Shdr64;
    /// let null_sct : Shdr64 = Default::default();
    ///
    /// assert_eq!([0].repeat(Shdr64::SIZE), null_sct.to_le_bytes());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

impl Section64 {
    pub fn new_null_section() -> Self {
        Self {
            contents: Contents64::Raw(Default::default()),
            header: Default::default(),
            name: Default::default(),
        }
    }

    pub fn new(name: String, hdr: ShdrPreparation64, contents: Contents64) -> Self {
        Self {
            contents,
            name,
            header: hdr.into(),
        }
    }

    /// create binary without header
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match &self.contents {
            Contents64::Raw(bytes) => bytes.clone(),
            Contents64::Symbols(syms) => {
                let mut bytes = Vec::new();
                for sym in syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
            Contents64::RelaSymbols(rela_syms) => {
                let mut bytes = Vec::new();
                for sym in rela_syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
            Contents64::Dynamics(dynamics) => {
                let mut bytes = Vec::new();
                for sym in dynamics.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
        }
    }
}

impl ShdrPreparation64 {
    pub fn ty(mut self, t: section::Type) -> Self {
        self.sh_type = t;
        self
    }

    pub fn flags<'a, I>(mut self, flags: I) -> Self
    where
        I: Iterator<Item = &'a section::Flag>,
    {
        for flag in flags {
            self.sh_flags |= Into::<Elf64Xword>::into(*flag);
        }

        self
    }

    pub fn link(mut self, link: Elf64Word) -> Self {
        self.sh_link = link;
        self
    }
    pub fn info(mut self, info: Elf64Word) -> Self {
        self.sh_info = info;
        self
    }
}

impl Default for ShdrPreparation64 {
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

impl Into<Shdr64> for ShdrPreparation64 {
    fn into(self) -> Shdr64 {
        Shdr64 {
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
