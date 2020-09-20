use crate::header::{class, data, elf_type, machine, osabi, version};
use crate::*;
use serde::{Deserialize, Serialize};
pub const ELF_MAGIC_NUMBER: u128 = 0x464c_457f;

#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Ehdr64 {
    e_ident: [u8; 16],
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

    // getter
    pub fn get_identification(&self) -> [u8; 16] {
        self.e_ident
    }
    pub fn get_class(&self) -> class::ELFCLASS {
        class::ELFCLASS::from(self.e_ident[data::ELFDATA::INDEX])
    }
    pub fn get_elf_type(&self) -> elf_type::ELFTYPE {
        elf_type::ELFTYPE::from(self.e_type)
    }
    pub fn get_machine(&self) -> machine::ELFMACHINE {
        machine::ELFMACHINE::from(self.e_machine)
    }
    pub fn get_shstrndx(&self) -> Elf64Half {
        self.e_shstrndx
    }
    pub fn get_shnum(&self) -> Elf64Half {
        self.e_shnum
    }
    pub fn get_shoff(&self) -> Elf64Off {
        self.e_shoff
    }
    pub fn get_entry(&self) -> Elf64Addr {
        self.e_entry
    }
    pub fn get_phnum(&self) -> Elf64Half {
        self.e_phnum
    }

    // setter
    pub fn set_phoff(&mut self, phoff: Elf64Off) {
        self.e_phoff = phoff;
    }
    pub fn set_shoff(&mut self, shoff: Elf64Off) {
        self.e_shoff = shoff;
    }
    pub fn set_phentsize(&mut self, phentsize: Elf64Half) {
        self.e_phentsize = phentsize;
    }
    pub fn set_shentsize(&mut self, shentsize: Elf64Half) {
        self.e_shentsize = shentsize;
    }
    pub fn set_phnum(&mut self, phnum: Elf64Half) {
        self.e_phnum = phnum;
    }
    pub fn set_shnum(&mut self, shnum: Elf64Half) {
        self.e_shnum = shnum;
    }
    pub fn set_ehsize(&mut self, ehsize: Elf64Half) {
        self.e_ehsize = ehsize;
    }
    pub fn set_shstrndx(&mut self, shstrndx: Elf64Half) {
        self.e_shstrndx = shstrndx;
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
    pub fn set_entry(&mut self, entry: Elf64Addr) {
        self.e_entry = entry;
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
}
