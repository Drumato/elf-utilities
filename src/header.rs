pub struct ElfIdentification {}

pub struct RawElfIdentification {
    pub e_magic: [u8; ELF_MAGICNUMBER_FIELD_LENGTH],
    pub e_class: u8,
    pub e_data: u8,
    pub e_version: u8,
    pub e_osabi: u8,
    pub e_abiversion: u8,
    pub e_padding: [u8; ELF_IDENT_PADDING_FIELD_LENGTH],
}

impl RawElfIdentification {
    pub fn new() -> Self {
        Self {
            e_magic: [0x00, 0x00, 0x00, 0x00],
            e_padding: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            e_osabi: 0,
            e_abiversion: 0,
            e_data: 0,
            e_class: 0,
            e_version: 0,
        }
    }
}

mod magicnumber {
    pub const ELF_MAGICNUMBER_SIGNATURE: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    pub const ELF_MAGICNUMBER_FIELD_LENGTH: usize = 4;

    pub const ELF_IDENT_MAGICNUMBER0_INDEX: usize = 0;
    pub const ELF_IDENT_MAGICNUMBER1_INDEX: usize = 1;
    pub const ELF_IDENT_MAGICNUMBER2_INDEX: usize = 2;
    pub const ELF_IDENT_MAGICNUMBER3_INDEX: usize = 3;
}
pub use magicnumber::*;

mod class {
    pub const ELF_IDENT_CLASS_INDEX: usize = 4;
}
pub use class::*;

mod data {
    pub const ELF_IDENT_DATA_INDEX: usize = 5;
}
pub use data::*;

pub const ELF_IDENT_VERSION_INDEX: usize = 6;
pub const ELF_IDENT_OSABI_INDEX: usize = 7;
pub const ELF_IDENT_ABIVERSION_INDEX: usize = 8;
pub const ELF_IDENT_PADDING_INDEX: usize = 9;
pub const ELF_IDENT_PADDING_FIELD_LENGTH: usize = 7;
