//! Type definitions for segment types.

use crate::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// Program header table entry unused
    Null,
    /// Loadable program segment
    Load,
    /// dynamic linking information
    Dynamic,
    /// Program interpreter
    Interp,
    /// Auxiliary information
    Note,
    /// Reserved
    ShLib,
    /// Entry for header table itself
    Phdr,
    /// Thread-local storage segment
    TLS,
    /// Number of defined types
    Num,
    /// GCC .eh_frame_hdr segment
    GNUEHFrame,
    /// Indicates stack executability
    GNUStack,
    /// Read-only after relocation
    GNURelRO,
    /// User-defined values
    Any(Elf64Word),
}

impl Type {
    pub fn to_bytes(&self) -> Elf64Word {
        match self {
            Self::Null => 0,
            Self::Load => 1,
            Self::Dynamic => 2,
            Self::Interp => 3,
            Self::Note => 4,
            Self::ShLib => 5,
            Self::Phdr => 6,
            Self::TLS => 7,
            Self::Num => 8,
            Self::GNUEHFrame => 0x6474e550,
            Self::GNUStack => 0x6474e551,
            Self::GNURelRO => 0x6474e552,
            Self::Any(c) => *c,
        }
    }
}

impl From<Elf64Word> for Type {
    fn from(bytes: Elf64Word) -> Self {
        match bytes {
            0 => Self::Null,
            1 => Self::Load,
            2 => Self::Dynamic,
            3 => Self::Interp,
            4 => Self::Note,
            5 => Self::ShLib,
            6 => Self::Phdr,
            7 => Self::TLS,
            8 => Self::Num,
            0x6474e550 => Self::GNUEHFrame,
            0x6474e551 => Self::GNUStack,
            0x6474e552 => Self::GNURelRO,
            _ => Self::Any(bytes),
        }
    }
}
