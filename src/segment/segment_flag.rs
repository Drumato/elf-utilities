//! Type definitions for segment flags.

use crate::*;

/// segment is executable
pub const PF_X: Elf64Word = 1 << 0;
/// segment is writable
pub const PF_W: Elf64Word = 1 << 1;
/// segment is readable
pub const PF_R: Elf64Word = 1 << 2;