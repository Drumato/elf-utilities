//! Type definitions for section header types.

use crate::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TYPE {
    NULL,
    PROGBITS,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DYNSYM,
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
            Self::NUM => 12,
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
            12 => Self::NUM,
            _ => Self::ANY(bytes),
        }
    }
}
