//! Type definitions for segment flags.

use crate::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
/// Segment flags
pub enum Flag {
    /// Segment is executable
    X,
    /// segment is writable
    W,
    /// segment is readable
    R,
}

impl Into<Elf64Word> for Flag {
    fn into(self) -> Elf64Word {
        match self {
            Flag::X => 1 << 0,
            Flag::W => 1 << 1,
            Flag::R => 1 << 2,
        }
    }
}

impl From<Elf64Word> for Flag {
    fn from(v: Elf64Word) -> Self {
        match v {
            0b1 => Flag::X,
            0b10 => Flag::W,
            0b100 => Flag::R,
            _ => unimplemented!(),
        }
    }
}
