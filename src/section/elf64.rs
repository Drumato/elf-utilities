//! Type definitions for 64-bit ELF binaries.

use crate::section::section_type;
use crate::*;

pub struct Section64 {
    pub name: String,
    pub header: Shdr64,

    /// for normal section
    pub bytes: Option<Vec<u8>>,

    /// for symbol table
    pub symbols: Option<Vec<symbol::Symbol64>>,
}

impl Section64 {
    pub fn new(section_name: String, shdr: Shdr64) -> Self {
        Self {
            name: section_name,
            header: shdr,
            bytes: None,
            symbols: None,
        }
    }

    pub fn new_null_section() -> Self {
        let mut null_section = Self::new(String::new(), Default::default());
        null_section.bytes = Some(Vec::new());
        null_section
    }
}

#[repr(C)]
pub struct Shdr64 {
    sh_name: Elf64Word,
    sh_type: Elf64Word,
    sh_flags: Elf64Xword,
    sh_addr: Elf64Addr,
    sh_offset: Elf64Off,
    sh_size: Elf64Xword,
    sh_link: Elf64Word,
    sh_info: Elf64Word,
    sh_addralign: Elf64Xword,
    sh_entsize: Elf64Xword,
}

impl Default for Shdr64 {
    fn default() -> Self {
        Self {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }
}

#[allow(dead_code)]
impl Shdr64 {
    pub fn size() -> Elf64Half {
        0x40
    }

    // getter
    pub fn get_type(&self) -> section_type::TYPE {
        section_type::TYPE::from(self.sh_type)
    }
    pub fn get_link(&self) -> Elf64Word {
        self.sh_link
    }
    pub fn get_size(&self) -> Elf64Xword {
        self.sh_size
    }
    pub fn get_info(&self) -> Elf64Word {
        self.sh_info
    }
    pub fn get_entry_size(&self) -> Elf64Xword {
        self.sh_entsize
    }
    pub fn get_flags(&self) -> Elf64Xword {
        self.sh_flags
    }
    pub fn get_addralign(&self) -> Elf64Xword {
        self.sh_addralign
    }
    pub fn get_offset(&self) -> Elf64Off {
        self.sh_offset
    }

    // setter
    pub fn set_name(&mut self, name: Elf64Word) {
        self.sh_name = name;
    }
    pub fn set_offset(&mut self, offset: Elf64Off) {
        self.sh_offset = offset;
    }
    pub fn set_type(&mut self, ty: section_type::TYPE) {
        self.sh_type = ty.to_bytes();
    }
    pub fn set_size(&mut self, size: Elf64Xword) {
        self.sh_size = size;
    }
    pub fn set_link(&mut self, link: Elf64Word) {
        self.sh_link = link;
    }
    pub fn set_info(&mut self, info: Elf64Word) {
        self.sh_info = info;
    }
    pub fn set_entry_size(&mut self, entry_size: Elf64Xword) {
        self.sh_entsize = entry_size;
    }
    pub fn set_flags(&mut self, flags: Elf64Xword) {
        self.sh_flags = flags;
    }
    pub fn set_addralign(&mut self, addralign: Elf64Xword) {
        self.sh_addralign = addralign;
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for byte in self.sh_name.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_type.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_flags.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_addr.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_offset.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_size.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_link.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_addralign.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.sh_entsize.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        bytes
    }
}
