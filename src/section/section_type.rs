//! Type definitions for section header types.

use crate::*;

#[derive(Debug, Clone, Hash, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// Section header table entry unused
    Null,
    /// Program data
    ProgBits,
    /// Symbol table
    SymTab,
    /// String table
    StrTab,
    /// Relocation entries with addends
    Rela,
    /// Symbol hash table
    Hash,
    /// Dynamic linking information
    Dynamic,
    /// Notes
    Note,
    /// Program space with no data(.bss)
    NoBits,
    /// Relocation entries, no addends
    Rel,
    /// Reserved
    ShLib,
    /// Dynamic linker symbol table
    DynSym,
    /// Array of constructors
    InitArray,
    /// Array of destructors
    FiniArray,
    /// Array of preconstructors
    PreInitArray,
    /// Section group
    Group,
    /// Extended section indices
    SymTabShNdx,
    /// Number of defined types
    Num,
    Any(Elf64Word),
}

impl Into<Elf64Word> for Type {
    fn into(self) -> Elf64Word {
        match self {
            Self::Null => 0,
            Self::ProgBits => 1,
            Self::SymTab => 2,
            Self::StrTab => 3,
            Self::Rela => 4,
            Self::Hash => 5,
            Self::Dynamic => 6,
            Self::Note => 7,
            Self::NoBits => 8,
            Self::Rel => 9,
            Self::ShLib => 10,
            Self::DynSym => 11,
            Self::InitArray => 14,
            Self::FiniArray => 15,
            Self::PreInitArray => 16,
            Self::Group => 17,
            Self::SymTabShNdx => 18,
            Self::Num => 19,
            Self::Any(c) => c,
        }
    }
}

impl From<Elf64Word> for Type {
    fn from(bytes: Elf64Word) -> Self {
        match bytes {
            0 => Self::Null,
            1 => Self::ProgBits,
            2 => Self::SymTab,
            3 => Self::StrTab,
            4 => Self::Rela,
            5 => Self::Hash,
            6 => Self::Dynamic,
            7 => Self::Note,
            8 => Self::NoBits,
            9 => Self::Rel,
            10 => Self::ShLib,
            11 => Self::DynSym,
            14 => Self::InitArray,
            15 => Self::FiniArray,
            16 => Self::PreInitArray,
            17 => Self::Group,
            18 => Self::SymTabShNdx,
            19 => Self::Num,
            _ => Self::Any(bytes),
        }
    }
}
