//! Type definitions for section header flags.

use crate::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
/// Section flags
pub enum Flag {
    /// Writable
    Write,
    /// Occupies memory during execution
    Alloc,
    /// Executable
    ExecInstr,
    /// Might be merged
    Merge,
    /// Contains nul-terminated strings
    Strings,
    /// `sh_info' contains SHT index
    InfoLink,
    /// Preserve order after combining
    LinkOrder,
    /// Non-standard OS specific handling required
    OSNonConforming,
    /// Section is member of a group
    Group,
    /// Section hold thread-local data
    TLS,
    /// Section with compressed data
    COMPRESSED,
}

impl Into<Elf32Word> for Flag {
    fn into(self) -> Elf32Word {
        match self {
            Flag::Write => 1 << 0,
            Flag::Alloc => 1 << 1,
            Flag::ExecInstr => 1 << 2,
            Flag::Merge => 1 << 4,
            Flag::Strings => 1 << 5,
            Flag::InfoLink => 1 << 6,
            Flag::LinkOrder => 1 << 7,
            Flag::OSNonConforming => 1 << 8,
            Flag::Group => 1 << 9,
            Flag::TLS => 1 << 10,
            Flag::COMPRESSED => 1 << 11,
        }
    }
}

impl Into<Elf64Xword> for Flag {
    fn into(self) -> Elf64Xword {
        match self {
            Flag::Write => 1 << 0,
            Flag::Alloc => 1 << 1,
            Flag::ExecInstr => 1 << 2,
            Flag::Merge => 1 << 4,
            Flag::Strings => 1 << 5,
            Flag::InfoLink => 1 << 6,
            Flag::LinkOrder => 1 << 7,
            Flag::OSNonConforming => 1 << 8,
            Flag::Group => 1 << 9,
            Flag::TLS => 1 << 10,
            Flag::COMPRESSED => 1 << 11,
        }
    }
}

impl From<Elf32Word> for Flag {
    fn from(v: Elf32Word) -> Self {
        match v {
            0b1 => Flag::Write,
            0b10 => Flag::Alloc,
            0b100 => Flag::ExecInstr,
            0b1000 => Flag::Merge,
            0b10000 => Flag::Strings,
            0b100000 => Flag::InfoLink,
            0b1000000 => Flag::LinkOrder,
            0b10000000 => Flag::OSNonConforming,
            0b100000000 => Flag::Group,
            0b1000000000 => Flag::TLS,
            0b10000000000 => Flag::COMPRESSED,
            _ => unimplemented!(),
        }
    }
}

impl From<Elf64Xword> for Flag {
    fn from(v: Elf64Xword) -> Self {
        match v {
            0b1 => Flag::Write,
            0b10 => Flag::Alloc,
            0b100 => Flag::ExecInstr,
            0b1000 => Flag::Merge,
            0b10000 => Flag::Strings,
            0b100000 => Flag::InfoLink,
            0b1000000 => Flag::LinkOrder,
            0b10000000 => Flag::OSNonConforming,
            0b100000000 => Flag::Group,
            0b1000000000 => Flag::TLS,
            0b10000000000 => Flag::COMPRESSED,
            _ => unimplemented!(),
        }
    }
}
