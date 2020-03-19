use crate::header::{class, data, elf_type, machine, osabi, version};
use crate::*;

pub const ELF_MAGIC_NUMBER: u128 = (0x7f454c46) << (12 * 8);
#[repr(C)]
pub struct Ehdr64 {
    e_ident: u128,
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}

impl Ehdr64 {
    pub fn new() -> Self {
        Self {
            e_ident: ELF_MAGIC_NUMBER,
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
    // getter
    pub fn get_identification(&self) -> u128 {
        self.e_ident
    }
    pub fn get_elf_type(&self) -> elf_type::ELFTYPE {
        elf_type::ELFTYPE::from(self.e_type)
    }
    pub fn get_machine(&self) -> machine::ELFMACHINE {
        machine::ELFMACHINE::from(self.e_machine)
    }

    // setter
    pub fn set_class(&mut self, class: class::ELFCLASS) {
        self.e_ident |= class.to_identifier();
    }
    pub fn set_data(&mut self, data: data::ELFDATA) {
        self.e_ident |= data.to_identifier();
    }
    pub fn set_version(&mut self, version: version::ELFVERSION) {
        self.e_ident |= version.to_identifier();
    }
    pub fn set_osabi(&mut self, osabi: osabi::ELFOSABI) {
        self.e_ident |= osabi.to_identifier();
    }
    pub fn set_elf_type(&mut self, e_type: elf_type::ELFTYPE) {
        self.e_type = e_type.to_bytes();
    }
    pub fn set_machine(&mut self, e_machine: machine::ELFMACHINE) {
        self.e_machine = e_machine.to_bytes();
    }
}

#[repr(C)]
pub struct Ehdr64Builder {
    e_ident: u128,
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}
impl Ehdr64Builder {
    pub fn new() -> Self {
        Self {
            e_ident: ELF_MAGIC_NUMBER,
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
    pub fn class(&mut self, class: class::ELFCLASS) -> &mut Self {
        self.set_class(class);
        self
    }
    pub fn data(&mut self, data: data::ELFDATA) -> &mut Self {
        self.set_data(data);
        self
    }
    pub fn version(&mut self, version: version::ELFVERSION) -> &mut Self {
        self.set_version(version);
        self
    }
    pub fn osabi(&mut self, osabi: osabi::ELFOSABI) -> &mut Self {
        self.set_osabi(osabi);
        self
    }
    pub fn elf_type(&mut self, e_type: elf_type::ELFTYPE) -> &mut Self {
        self.set_elf_type(e_type);
        self
    }
    pub fn finalize(&self) -> Ehdr64 {
        Ehdr64 {
            e_ident: self.e_ident,
            e_type: self.e_type,
            e_machine: self.e_machine,
            e_version: self.e_version,
            e_entry: self.e_entry,
            e_phoff: self.e_phoff,
            e_shoff: self.e_shoff,
            e_flags: self.e_flags,
            e_ehsize: self.e_ehsize,
            e_phentsize: self.e_phentsize,
            e_phnum: self.e_phnum,
            e_shentsize: self.e_shentsize,
            e_shnum: self.e_shnum,
            e_shstrndx: self.e_shstrndx,
        }
    }

    fn set_class(&mut self, class: class::ELFCLASS) {
        self.e_ident |= class.to_identifier();
    }
    fn set_data(&mut self, data: data::ELFDATA) {
        self.e_ident |= data.to_identifier();
    }
    fn set_version(&mut self, version: version::ELFVERSION) {
        self.e_ident |= version.to_identifier();
    }
    fn set_osabi(&mut self, osabi: osabi::ELFOSABI) {
        self.e_ident |= osabi.to_identifier();
    }
    fn set_elf_type(&mut self, e_type: elf_type::ELFTYPE) {
        self.e_type = e_type.to_bytes();
    }
}
