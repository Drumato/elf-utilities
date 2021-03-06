use crate::header::{class, data, elf_type, machine, osabi, version};
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Ehdr32 {
    pub e_ident: [u8; 16],
    pub e_type: Elf32Half,
    pub e_machine: Elf32Half,
    pub e_version: Elf32Word,
    pub e_entry: Elf32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
    pub e_flags: Elf32Word,
    pub e_ehsize: Elf32Half,
    pub e_phentsize: Elf32Half,
    pub e_phnum: Elf32Half,
    pub e_shentsize: Elf32Half,
    pub e_shnum: Elf32Half,
    pub e_shstrndx: Elf32Half,
}

impl Default for Ehdr32 {
    fn default() -> Self {
        Self {
            e_ident: [
                0x7f, 0x45, 0x4c, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
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
}

impl Ehdr32 {
    pub const SIZE: Elf32Half = 52;

    pub fn get_class(&self) -> class::Class {
        class::Class::from(self.e_ident[class::Class::INDEX])
    }
    pub fn get_data(&self) -> data::Data {
        data::Data::from(self.e_ident[data::Data::INDEX])
    }
    pub fn get_file_version(&self) -> version::Version {
        version::Version::from(self.e_ident[version::Version::INDEX])
    }
    pub fn get_object_version(&self) -> version::Version {
        version::Version::from(self.e_version)
    }
    pub fn get_type(&self) -> elf_type::Type {
        elf_type::Type::from(self.e_type)
    }
    pub fn get_machine(&self) -> machine::Machine {
        machine::Machine::from(self.e_machine)
    }
    pub fn get_osabi(&self) -> osabi::OSABI {
        osabi::OSABI::from(self.e_ident[osabi::OSABI::INDEX])
    }
    pub fn set_class(&mut self, c: class::Class) {
        self.e_ident[class::Class::INDEX] = c.to_identifier();
    }
    pub fn set_data(&mut self, d: data::Data) {
        self.e_ident[data::Data::INDEX] = d.to_identifier();
    }
    pub fn set_file_version(&mut self, v: version::Version) {
        self.e_ident[version::Version::INDEX] = v.to_identifier();
    }
    pub fn set_object_version(&mut self, v: version::Version) {
        self.e_version = v.to_object_version();
    }
    pub fn set_osabi(&mut self, o: osabi::OSABI) {
        self.e_ident[osabi::OSABI::INDEX] = o.to_identifier();
    }
    pub fn set_elf_type(&mut self, e_type: elf_type::Type) {
        self.e_type = e_type.to_bytes();
    }
    pub fn set_machine(&mut self, e_machine: machine::Machine) {
        self.e_machine = e_machine.to_bytes();
    }

    /// Create Vec<u8> from this.
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(buf: &[u8], start: usize) -> Result<Self, Box<dyn std::error::Error>> {
        // bincode::ErrorKindをトレイトオブジェクトとするため,この冗長な書き方が必要
        match bincode::deserialize(&buf[start..]) {
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }
}
