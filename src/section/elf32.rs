//! Type definitions for 32-bit ELF binaries.

use crate::*;
use section::Section;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
/// section's contents
pub enum Contents32 {
    /// almost section's data
    Raw(Vec<u8>),
    /// symbol table's representation
    Symbols(Vec<symbol::Symbol32>),
    /// relocation symbol table's representation
    RelaSymbols(Vec<relocation::Rela32>),
    /// dynamic information's representation
    Dynamics(Vec<dynamic::Dyn32>),
}

impl section::Contents for Contents32 {
    type Symbol = symbol::Symbol32;
    type Dyn = dynamic::Dyn32;
    type Rela = relocation::Rela32;

    fn clone_raw_binary(&self) -> Vec<u8> {
        match self {
            Contents32::Raw(bytes) => bytes.clone(),
            _ => panic!("cannot call 'clone_raw_binary' without Contents32::Raw"),
        }
    }
}

impl Default for Contents32 {
    fn default() -> Self {
        Contents32::Raw(Default::default())
    }
}

#[derive(Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Section32 {
    pub name: String,
    pub header: Shdr32,
    pub contents: Contents32,
}

impl section::Section for Section32 {
    type Header = Shdr32;
    type Contents = Contents32;

    fn new(header: Shdr32) -> Self {
        Self {
            header,
            contents: Contents32::Raw(Default::default()),
            name: String::new(),
        }
    }
    fn clone_contents(&self) -> Contents32 {
        self.contents.clone()
    }
    fn clone_raw_binary(&self) -> Vec<u8> {
        match &self.contents {
            Contents32::Raw(bytes) => bytes.clone(),
            _ => unreachable!(),
        }
    }
    fn update_symbol_name(&mut self, sym_idx: usize, name_bytes: &[u8]) {
        match self.contents {
            Contents32::Symbols(ref mut syms) => {
                let name_idx = syms[sym_idx].st_name as usize;

                let name_bytes: Vec<u8> = name_bytes[name_idx as usize..]
                    .to_vec()
                    .iter()
                    .take_while(|byte| **byte != 0x00)
                    .copied()
                    .collect();

                syms[sym_idx].symbol_name =
                    Some(std::str::from_utf8(&name_bytes).unwrap().to_string());
            }
            _ => unreachable!(),
        }
    }
    fn name_idx(&self) -> usize {
        self.header.sh_name as usize
    }
    fn update_name(&mut self, name: String) {
        self.name = name;
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

    fn symbol_number(&self) -> usize {
        match &self.contents {
            Contents32::Symbols(syms) => syms.len(),
            _ => unreachable!(),
        }
    }
    fn section_link(&self) -> usize {
        self.header.sh_link as usize
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

    fn section_type(&self) -> section::Type {
        self.header.get_type()
    }

    fn entry_size(&self) -> usize {
        self.header.sh_entsize as usize
    }

    fn section_size(&self) -> usize {
        self.header.sh_size as usize
    }

    fn update_contents_from_raw_bytes(&mut self, bytes: Vec<u8>) {
        match self.header.get_type() {
            section::Type::Dynamic => {
                self.contents = Contents32::Dynamics(self.parse_bytes_as_dynamics(bytes));
            }
            section::Type::SymTab | section::Type::DynSym => {
                self.contents = Contents32::Symbols(self.parse_bytes_as_symbols(bytes));
            }
            section::Type::Rela => {
                self.contents = Contents32::RelaSymbols(self.parse_bytes_as_rela_symbols(bytes));
            }

            _ => {
                self.contents = Contents32::Raw(bytes);
            }
        }
    }
}

impl Section32 {
    /// create binary without header
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match &self.contents {
            Contents32::Raw(bytes) => bytes.clone(),
            Contents32::Symbols(syms) => {
                let mut bytes = Vec::new();
                for sym in syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
            Contents32::RelaSymbols(rela_syms) => {
                let mut bytes = Vec::new();
                for sym in rela_syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
            Contents32::Dynamics(dynamics) => {
                let mut bytes = Vec::new();
                for sym in dynamics.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            }
        }
    }

    pub fn new_null_section() -> Self {
        Self::new(Default::default())
    }

    fn parse_bytes_as_rela_symbols(&self, bytes: Vec<u8>) -> Vec<relocation::Rela32> {
        let entry_number = self.header.sh_size as usize / self.header.sh_entsize as usize;
        let mut table = Vec::new();

        for idx in 0..entry_number {
            let start = idx * self.header.sh_entsize as usize;
            let end = (idx + 1) * self.header.sh_entsize as usize;
            let entry = bincode::deserialize(&bytes[start..end]).unwrap();
            table.push(entry);
        }

        table
    }
    fn parse_bytes_as_dynamics(&self, bytes: Vec<u8>) -> Vec<dynamic::Dyn32> {
        let entry_number = self.header.sh_size as usize / self.header.sh_entsize as usize;
        let mut table = Vec::new();

        for idx in 0..entry_number {
            let start = idx * self.header.sh_entsize as usize;
            let end = (idx + 1) * self.header.sh_entsize as usize;
            eprintln!("entry len => {}", end - start);
            let entry = bincode::deserialize(&bytes[start..end]).unwrap();
            table.push(entry);
        }

        table
    }
    fn parse_bytes_as_symbols(&self, bytes: Vec<u8>) -> Vec<symbol::Symbol32> {
        let entry_number = self.header.sh_size as usize / self.header.sh_entsize as usize;
        let mut table = Vec::new();

        for idx in 0..entry_number {
            let start = idx * self.header.sh_entsize as usize;
            let end = (idx + 1) * self.header.sh_entsize as usize;
            let entry = bincode::deserialize(&bytes[start..end]).unwrap();
            table.push(entry);
        }

        table
    }
}

#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Shdr32 {
    /// Section name, index in string tbl
    pub sh_name: Elf32Word,
    /// Type of section
    pub sh_type: Elf32Word,
    /// Miscellaneous section attributes
    pub sh_flags: Elf32Word,
    ///  Section virtual addr at execution
    pub sh_addr: Elf32Addr,
    /// Section file offset
    pub sh_offset: Elf32Off,
    /// Size of section in bytes
    pub sh_size: Elf32Word,
    /// Index of another section
    pub sh_link: Elf32Word,
    /// Additional section information
    pub sh_info: Elf32Word,
    /// Section alignment
    pub sh_addralign: Elf32Word,
    /// Entry size if section holds table
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
    pub fn get_type(&self) -> section::Type {
        section::Type::from(self.sh_type)
    }
    // setter
    pub fn set_type(&mut self, ty: section::Type) {
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

#[cfg(test)]
mod elf32_tests {
    use super::*;

    #[test]
    fn section32_test() {
        let sct = Section32::new_null_section();

        assert_eq!(
            vec![0x00; Shdr32::size() as usize],
            sct.header.to_le_bytes(),
        );

        assert_eq!(Vec::new() as Vec<u8>, sct.to_le_bytes(),);
    }
}
