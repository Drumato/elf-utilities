use crate::*;

/* definitions for sh_flags */
pub const SHF_ALLOC: Elf64Xword = 1 << 1;
pub const SHF_EXECINSTR: Elf64Xword = 1 << 2;
pub const SHF_INFO_LINK: Elf64Xword = 1 << 6;
