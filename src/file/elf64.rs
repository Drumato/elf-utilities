use crate::header;
use crate::section;

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

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut file_binary: Vec<u8> = Vec::new();

        let mut header_binary = self.ehdr.to_le_bytes();
        file_binary.append(&mut header_binary);

        for sct in self.sections.iter() {
            let mut section_binary = sct.bytes.clone();
            file_binary.append(&mut section_binary);
        }

        for sct in self.sections.iter() {
            let mut shdr_binary = sct.header.to_le_bytes();
            file_binary.append(&mut shdr_binary);
        }

        // TODO: Phdrs

        file_binary
    }

    pub fn section_number(&self) -> usize {
        self.sections.len()
    }

    pub fn add_section(&mut self, sct: section::Section64) {
        self.sections.push(sct);
    }
}
