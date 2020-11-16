use crate::*;
pub use elf64::*;

mod elf64;
pub use elf32::*;

mod elf32;

pub const R_X86_64_PC32: Elf64Xword = 2;
pub const R_X86_64_PLT32: Elf64Xword = 4;
pub const R_X86_64_32: Elf64Xword = 10;
