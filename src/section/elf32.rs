//! Type definitions for 32-bit ELF binaries.

use crate::section::{section_type, Section, Type};
use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Section32 {
    pub name: String,
    pub header: Shdr32,

    pub bytes: Vec<u8>,
}

impl Section for Section32 {
    type Header = Shdr32;
    type Symbol = symbol::Symbol32;
    type Dyn = dynamic::Dyn32;
    type Rela = relocation::Rela32;

    fn new(header: Shdr32) -> Self {
        Self {
            header,
            bytes: Vec::new(),
            name: String::new(),
        }
    }

    fn name_idx(&self) -> usize {
        self.header.sh_name as usize
    }
    fn clone_contents(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    fn update_name(&mut self, name: String) {
        self.name = name;
    }
    fn update_contents(&mut self, contents: Vec<u8>) {
        self.bytes = contents;
    }

    fn header_deserialize(
        buf: &[u8],
        header_start: usize,
    ) -> Result<Shdr32, Box<dyn std::error::Error>> {
        match bincode::deserialize(&buf[header_start..]) {
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }

    fn header_size() -> usize {
        Shdr32::size() as usize
    }

    fn size_zero(&self) -> bool {
        self.header.sh_size == 0
    }

    fn offset(&self) -> usize {
        self.header.sh_offset as usize
    }

    fn section_type(&self) -> Type {
        self.header.get_type()
    }

    fn entry_size(&self) -> usize {
        self.header.sh_entsize as usize
    }

    fn section_size(&self) -> usize {
        self.header.sh_size as usize
    }

    fn parse_bytes_as_symbols(&self, related_string_table: &Section32) -> Vec<Self::Symbol> {
        let mut symbols: Vec<Self::Symbol> = self.parse_bytes_as_table();

        let symbol_strtab = related_string_table.bytes.clone();

        let symbol_number = symbols.len();

        for sym_idx in 0..symbol_number {
            let name_idx = symbols[sym_idx].st_name;
            let name_bytes: Vec<u8> = symbol_strtab[name_idx as usize..]
                .to_vec()
                .iter()
                .take_while(|byte| **byte != 0x00)
                .map(|byte| *byte)
                .collect();

            symbols[sym_idx].symbol_name =
                Some(std::str::from_utf8(&name_bytes).unwrap().to_string());
        }
        symbols
    }
    fn parse_bytes_as_dynamics(&self) -> Vec<Self::Dyn> {
        self.parse_bytes_as_table()
    }
    fn parse_bytes_as_relas(&self) -> Vec<Self::Rela> {
        self.parse_bytes_as_table()
    }
}

impl Section32 {
    pub fn write_byte_to_index(&mut self, byte: u8, idx: usize) {
        self.bytes[idx] = byte;
    }

    /// create binary without header
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn new_null_section() -> Self {
        Self::new(Default::default())
    }

    fn parse_bytes_as_table<'a, T: Deserialize<'a>>(&'a self) -> Vec<T> {
        let entry_number = self.section_size() / self.entry_size();
        let mut table: Vec<T> = Vec::new();

        for idx in 0..entry_number {
            let start = idx * self.entry_size();
            let end = (idx + 1) * self.entry_size();

            let entry: T = bincode::deserialize(&self.bytes[start..end]).unwrap();
            table.push(entry);
        }

        table
    }
}

#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Shdr32 {
    pub sh_name: Elf32Word,
    pub sh_type: Elf32Word,
    pub sh_flags: Elf32Word,
    pub sh_addr: Elf32Addr,
    pub sh_offset: Elf32Off,
    pub sh_size: Elf32Word,
    pub sh_link: Elf32Word,
    pub sh_info: Elf32Word,
    pub sh_addralign: Elf32Word,
    pub sh_entsize: Elf32Word,
}

impl Default for Shdr32 {
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
impl Shdr32 {
    pub fn size() -> Elf32Half {
        40
    }

    // getter
    pub fn get_type(&self) -> section_type::Type {
        section_type::Type::from(self.sh_type)
    }
    // setter
    pub fn set_type(&mut self, ty: section_type::Type) {
        self.sh_type = ty.to_bytes();
    }

    /// Create Vec<u8> from this.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::section::Shdr32;
    /// let null_sct : Shdr32 = Default::default();
    ///
    /// assert_eq!([0].repeat(Shdr32::size() as usize), null_sct.to_le_bytes());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
