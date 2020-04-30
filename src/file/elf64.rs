use crate::header;
use crate::section;

// use crate::section;

#[repr(C)]
pub struct ELF64 {
    ehdr: header::Ehdr64,
    sections: Vec<section::Section64>,
    // phdrs: Vec<program::Phdr64>,
}

impl ELF64 {
    pub fn new(elf_header: header::Ehdr64) -> Self {
        Self {
            ehdr: elf_header,
            sections: Vec::new(),
        }
    }

    pub fn section_number(&self) -> usize {
        self.sections.len()
    }
}
