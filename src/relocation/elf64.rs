use crate::*;

pub const R_X86_64_PC32: Elf64Xword = 2;
pub const R_X86_64_PLT32: Elf64Xword = 4;

#[derive(Clone)]
#[repr(C)]
pub struct Rela64 {
    r_offset: Elf64Addr,
    r_info: Elf64Xword,
    r_addend: Elf64Sxword,
}

impl Default for Rela64 {
    fn default() -> Self {
        Self {
            r_offset: 0,
            r_info: 0,
            r_addend: 0,
        }
    }
}

#[allow(dead_code)]
impl Rela64 {
    pub fn size() -> Elf64Xword {
        24
    }
    pub fn bind(&self) -> Elf64Xword {
        self.r_info >> 32
    }

    pub fn get_offset(&self) -> Elf64Addr {
        self.r_offset
    }
    pub fn get_info(&self) -> Elf64Xword {
        self.r_info
    }
    pub fn get_addend(&self) -> Elf64Sxword {
        self.r_addend
    }

    pub fn set_addend(&mut self, addend: Elf64Sxword) {
        self.r_addend = addend;
    }
    pub fn set_offset(&mut self, offset: Elf64Addr) {
        self.r_offset = offset;
    }
    pub fn set_info(&mut self, info: Elf64Xword) {
        self.r_info = info;
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for byte in self.r_offset.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.r_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.r_addend.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        bytes
    }
}
