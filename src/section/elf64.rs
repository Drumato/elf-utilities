//! Type definitions for 64-bit ELF binaries.

use crate::section;
use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Contents64 {
    Raw(Vec<u8>),
    Symbols(Vec<symbol::Symbol64>),
    RelaSymbols(Vec<relocation::Rela64>),
    Dynamics(Vec<dynamic::Dyn64>),
}

impl section::Contents for Contents64 {
    type Symbol = symbol::Symbol64;
    type Dyn = dynamic::Dyn64;
    type Rela = relocation::Rela64;
    fn clone_raw_binary(&self) -> Vec<u8> {
        match self {
            Contents64::Raw(bytes) => bytes.clone(),
            _ => panic!("cannot call 'clone_raw_binary' without Contents64::Raw"),
        }
    }
    fn clone_symbols(&self) -> Vec<Self::Symbol> {
        match self {
            Contents64::Symbols(syms) => syms.clone(),
            _ => panic!("cannot call 'clone_symbols' without Contents64::Symbols"),
        }
    }
    fn clone_dynamics(&self) -> Vec<Self::Dyn> {
        match self {
            Contents64::Dynamics(dynamics) => dynamics.clone(),
            _ => panic!("cannot call 'clone_dynamics' without Contents64::Dynamics"),
        }
    }
    fn clone_rela_symbols(&self) -> Vec<Self::Rela> {
        match self {
            Contents64::RelaSymbols(rela_syms) => rela_syms.clone(),
            _ => panic!("cannot call 'clone_rela_symbols' without Contents64::RelaSymbols"),
        }
    }
}

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Section64 {
    pub name: String,
    pub header: Shdr64,

    pub contents: Contents64,
}

impl section::Section for Section64 {
    type Header = section::Shdr64;
    type Contents = Contents64;

    fn new(header: section::Shdr64) -> Self {
        Self {
            header,
            contents: Contents64::Raw(Default::default()),
            name: String::new(),
        }
    }
    fn clone_contents(&self) -> Contents64{
        self.contents.clone()
    }
    fn update_contents_from_raw_bytes(&mut self, bytes: Vec<u8>) {
        match self.header.get_type(){
            section::Type::Dynamic => {
                self.contents = Contents64::Dynamics(self.parse_bytes_as_dynamics(bytes));
            },
            section::Type::SymTab | section::Type::DynSym => {
                self.contents = Contents64::Symbols(self.parse_bytes_as_symbols(bytes));
            },
            section::Type::Rela => {
                self.contents = Contents64::RelaSymbols(self.parse_bytes_as_rela_symbols(bytes));
            },
            _ => {
                self.contents = Contents64::Raw(bytes);
            },
        }
    }
    fn name_idx(&self) -> usize {
        self.header.sh_name as usize
    }
    fn update_name(&mut self, name: String) {
        self.name = name;
    }
    fn clone_raw_binary(&self) -> Vec<u8>{
        match &self.contents{
            Contents64::Raw(bytes) => bytes.clone(),
            _ => unreachable!()
        }
    }
    fn update_symbol_name(&mut self, sym_idx: usize, name_bytes: &[u8]){
        match self.contents{
            Contents64::Symbols(ref mut syms) => {
                let name_idx = syms[sym_idx].st_name as usize;

                let name_bytes: Vec<u8> = name_bytes[name_idx as usize..]
                .to_vec()
                .iter()
                .take_while(|byte| **byte != 0x00)
                .copied()
                .collect();

                syms[sym_idx].symbol_name = Some(std::str::from_utf8(&name_bytes).unwrap().to_string());
            }
            _ => unreachable!(),
        }
    }

    fn header_deserialize(
        buf: &[u8],
        header_start: usize,
    ) -> Result<section::Shdr64, Box<dyn std::error::Error>> {
        match bincode::deserialize(&buf[header_start..]) {
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }

    
    fn symbol_number(&self) -> usize{
        match &self.contents {
            Contents64::Symbols(syms) => syms.len(),
            _ => unreachable!(),
        }
    }
    
    fn section_link(&self) -> usize{
        self.header.sh_link as usize
    }
    
    fn header_size() -> usize {
        Shdr64::size() as usize
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
}

impl Section64 {
    pub fn new_null_section() -> Self {
        Self {
            contents: Contents64::Raw(Default::default()),
            header: Default::default(),
            name: Default::default(),
        }
    }
    /// create binary without header
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match &self.contents {
            Contents64::Raw(bytes) => bytes.clone(),
            Contents64::Symbols(syms) => {
                let mut bytes = Vec::new();
                for sym in syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            },
            Contents64::RelaSymbols(rela_syms) => {
                let mut bytes = Vec::new();
                for sym in rela_syms.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            },
            Contents64::Dynamics(dynamics) => {
                let mut bytes = Vec::new();
                for sym in dynamics.iter() {
                    bytes.append(&mut sym.to_le_bytes());
                }
                bytes
            },
        }
    }

    fn parse_bytes_as_rela_symbols(&self, bytes: Vec<u8>) -> Vec<relocation::Rela64> {
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
    fn parse_bytes_as_dynamics(&self, bytes: Vec<u8>) -> Vec<dynamic::Dyn64> {
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
    fn parse_bytes_as_symbols(&self, bytes: Vec<u8>) -> Vec<symbol::Symbol64> {
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
pub struct Shdr64 {
    pub sh_name: Elf64Word,
    pub sh_type: Elf64Word,
    pub sh_flags: Elf64Xword,
    pub sh_addr: Elf64Addr,
    pub sh_offset: Elf64Off,
    pub sh_size: Elf64Xword,
    pub sh_link: Elf64Word,
    pub sh_info: Elf64Word,
    pub sh_addralign: Elf64Xword,
    pub sh_entsize: Elf64Xword,
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
    /// use elf_utilities::section::Shdr64;
    /// let null_sct : Shdr64 = Default::default();
    ///
    /// assert_eq!([0].repeat(Shdr64::size() as usize), null_sct.to_le_bytes());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
