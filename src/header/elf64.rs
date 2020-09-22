use crate::header::{class, data, elf_type, machine, osabi, version};
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Ehdr64 {
    pub e_ident: [u8; 16],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

impl Default for Ehdr64 {
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

impl Ehdr64 {
    pub fn size() -> Elf64Half {
        0x40
    }

    pub fn get_class(&self) -> class::ELFCLASS {
        class::ELFCLASS::from(self.e_ident[class::ELFCLASS::INDEX])
    }
    pub fn get_data(&self) -> data::ELFDATA {
        data::ELFDATA::from(self.e_ident[data::ELFDATA::INDEX])
    }
    pub fn get_type(&self) -> elf_type::ELFTYPE {
        elf_type::ELFTYPE::from(self.e_type)
    }
    pub fn get_machine(&self) -> machine::ELFMACHINE {
        machine::ELFMACHINE::from(self.e_machine)
    }
    pub fn set_class(&mut self, c: class::ELFCLASS) {
        self.e_ident[class::ELFCLASS::INDEX] = c.to_identifier();
    }
    pub fn set_data(&mut self, d: data::ELFDATA) {
        self.e_ident[data::ELFDATA::INDEX] = d.to_identifier();
    }
    pub fn set_version(&mut self, v: version::ELFVERSION) {
        self.e_ident[version::ELFVERSION::INDEX] = v.to_identifier();
    }
    pub fn set_osabi(&mut self, o: osabi::ELFOSABI) {
        self.e_ident[osabi::ELFOSABI::INDEX] = o.to_identifier();
    }
    pub fn set_elf_type(&mut self, e_type: elf_type::ELFTYPE) {
        self.e_type = e_type.to_bytes();
    }
    pub fn set_machine(&mut self, e_machine: machine::ELFMACHINE) {
        self.e_machine = e_machine.to_bytes();
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::header::Ehdr64;
    /// let null_ehdr : Ehdr64 = Default::default();
    ///
    /// assert_eq!(
    ///     vec![
    ///         0x7f, 0x45, 0x4c, 0x46, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     ],
    ///     null_ehdr.to_le_bytes()
    /// );
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(buf: &[u8], start: usize) -> Result<Self, Box<dyn std::error::Error>>{
        // bincode::ErrorKindをトレイトオブジェクトとするため,この冗長な書き方が必要
        match bincode::deserialize(&buf[start..]){
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }
}
