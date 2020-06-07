//! Type definitions for section header types.

use crate::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TYPE {
    /// Section header table entry unused
    NULL,
    /// Program data
    PROGBITS,
    /// Symbol table
    SYMTAB,
    /// String table
    STRTAB,
    /// Relocation entries with addends
    RELA,
    /// Symbol hash table
    HASH,
    /// Dynamic linking information
    DYNAMIC,
    /// Notes
    NOTE,
    /// Program space with no data(.bss)
    NOBITS,
    /// Relocation entries, no addends
    REL,
    /// Reserved
    SHLIB,
    /// Dynamic linker symbol table
    DYNSYM,
    /// Array of constructors
    INITARRAY,
    /// Array of destructors
    FINIARRAY,
    /// Array of preconstructors
    PREINITARRAY,
    /// Section group
    GROUP,
    /// Extended section indices
    SYMTABSHNDX,
    /// Number of defined types
    NUM,
    ANY(Elf64Word),
}

impl TYPE {
    pub fn to_bytes(&self) -> Elf64Word {
        match self {
            Self::NULL => 0,
            Self::PROGBITS => 1,
            Self::SYMTAB => 2,
            Self::STRTAB => 3,
            Self::RELA => 4,
            Self::HASH => 5,
            Self::DYNAMIC => 6,
            Self::NOTE => 7,
            Self::NOBITS => 8,
            Self::REL => 9,
            Self::SHLIB => 10,
            Self::DYNSYM => 11,
            Self::INITARRAY => 14,
            Self::FINIARRAY => 15,
            Self::PREINITARRAY => 16,
            Self::GROUP => 17,
            Self::SYMTABSHNDX => 18,
            Self::NUM => 19,
            Self::ANY(c) => *c,
        }
    }
}

impl From<Elf64Word> for TYPE {
    fn from(bytes: Elf64Word) -> Self {
        match bytes {
            0 => Self::NULL,
            1 => Self::PROGBITS,
            2 => Self::SYMTAB,
            3 => Self::STRTAB,
            4 => Self::RELA,
            5 => Self::HASH,
            6 => Self::DYNAMIC,
            7 => Self::NOTE,
            8 => Self::NOBITS,
            9 => Self::REL,
            10 => Self::SHLIB,
            11 => Self::DYNSYM,
            14 => Self::INITARRAY,
            15 => Self::FINIARRAY,
            16 => Self::PREINITARRAY,
            17 => Self::GROUP,
            18 => Self::SYMTABSHNDX,
            19 => Self::NUM,
            _ => Self::ANY(bytes),
        }
    }
}
