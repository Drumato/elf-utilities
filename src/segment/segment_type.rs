//! Type definitions for segment types.

use crate::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TYPE {
    /// Program header table entry unused
    NULL,
    /// Loadable program segment
    LOAD,
    /// dynamic linking information
    DYNAMIC,
    /// Program interpreter
    INTERP,
    /// Auxiliary information
    NOTE,
    /// Reserved
    SHLIB,
    /// Entry for header table itself
    PHDR,
    /// Thread-local storage segment
    TLS,
    /// Number of defined types
    NUM,
    /// GCC .eh_frame_hdr segment
    GNUEHFRAME,
    /// Indicates stack executability
    GNUSTACK,
    /// Read-only after relocation
    GNURELRO,
    /// User-defined values
    ANY(Elf64Word),
}

impl TYPE {
    pub fn to_bytes(&self) -> Elf64Word {
        match self {
            Self::NULL => 0,
            Self::LOAD => 1,
            Self::DYNAMIC => 2,
            Self::INTERP => 3,
            Self::NOTE => 4,
            Self::SHLIB => 5,
            Self::PHDR => 6,
            Self::TLS => 7,
            Self::NUM => 8,
            Self::GNUEHFRAME => 0x6474e550,
            Self::GNUSTACK => 0x6474e551,
            Self::GNURELRO => 0x6474e552,
            Self::ANY(c) => *c,
        }
    }
}

impl From<Elf64Word> for TYPE {
    fn from(bytes: Elf64Word) -> Self {
        match bytes {
            0 => Self::NULL,
            1 => Self::LOAD,
            2 => Self::DYNAMIC,
            3 => Self::INTERP,
            4 => Self::NOTE,
            5 => Self::SHLIB,
            6 => Self::PHDR,
            7 => Self::TLS,
            8 => Self::NUM,
            0x6474e550 => Self::GNUEHFRAME,
            0x6474e551 => Self::GNUSTACK,
            0x6474e552 => Self::GNURELRO,
            _ => Self::ANY(bytes),
        }
    }
}
