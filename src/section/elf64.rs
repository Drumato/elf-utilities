use crate::section::section_type;
use crate::*;

pub struct Section64 {
    pub name: String,
    pub header: Shdr64,
    pub bytes: Vec<u8>,
}

impl Section64 {
    pub fn new(section_name: String, shdr: Shdr64) -> Self {
        Self {
            name: section_name,
            header: shdr,
            bytes: Vec::new(),
        }
    }

    pub fn new_null_section() -> Self {
        Self::new(String::new(), Default::default())
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
    // getter
    pub fn get_type(&self) -> section_type::SHTYPE {
        section_type::SHTYPE::from(self.sh_type)
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

    // setter
    pub fn set_type(&mut self, ty: section_type::SHTYPE) {
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
}