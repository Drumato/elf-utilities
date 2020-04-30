use crate::*;

/* definitions for st_info(bind) */
pub const STB_GLOBAL: u8 = 1; /* Global symbol */

/* definitions for st_info(type) */
pub const STT_FUNC: u8 = 2; /* Symbol is a code object */

#[repr(C)]
pub struct Symbol64 {
    st_name: Elf64Word,
    st_info: u8,
    st_other: u8,
    st_shndx: Elf64Section,
    st_value: Elf64Addr,
    st_size: Elf64Xword,
}

impl Default for Symbol64 {
    fn default() -> Self {
        Self {
            st_name: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
        }
    }
}

#[allow(dead_code)]
impl Symbol64 {
    pub fn new_null_symbol() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: Elf64Word) {
        self.st_name = name;
    }
    pub fn set_info(&mut self, info: u8) {
        self.st_info = info;
    }
    pub fn set_other(&mut self, other: u8) {
        self.st_other = other;
    }
    pub fn set_shndx(&mut self, shndx: Elf64Section) {
        self.st_shndx = shndx;
    }
    pub fn set_value(&mut self, value: Elf64Addr) {
        self.st_value = value;
    }
    pub fn set_size(&mut self, size: Elf64Xword) {
        self.st_size = size;
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for byte in self.st_name.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_other.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_shndx.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_value.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_size.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        bytes
    }
}
