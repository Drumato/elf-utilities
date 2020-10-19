//! ELF section and section header utilities.

pub use base::*;
pub use elf32::*;
pub use elf64::*;
pub use section_flag::*;
pub use section_type::*;
pub use util::*;

mod base;
mod elf32;
mod elf64;
mod section_flag;
mod section_type;
mod util;

/// Undefined section
pub const SHN_UNDEF: u16 = 0;
/// Start of processor-specific
pub const SHN_LOPROC: u16 = 0xff00;
/// End of processor-specific
pub const SHN_HIPROC: u16 = 0xff1f;
/// Start of OS-specific
pub const SHN_LOOS: u16 = 0xff20;
/// End of OS-specific
pub const SHN_HIOS: u16 = 0xff3f;
/// Associated symbol is absolute
pub const SHN_ABS: u16 = 0xfff1;
/// Associated symbol is common
pub const SHN_COMMON: u16 = 0xfff2;
/// Index is in extra table
pub const SHN_XINDEX: u16 = 0xffff;
