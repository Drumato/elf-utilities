use crate::*;

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
    pub fn set_addend(&mut self, addend: Elf64Sxword) {
        self.r_addend = addend;
    }
    pub fn set_offset(&mut self, offset: Elf64Addr) {
        self.r_offset = offset;
    }
    pub fn set_info(&mut self, info: Elf64Xword) {
        self.r_info = info;
    }
}