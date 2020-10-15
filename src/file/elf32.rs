use crate::{header, section, segment};

#[repr(C)]
pub struct ELF32 {
    pub ehdr: header::Ehdr32,
    // pub sections: Vec<section::Section32>,
    // pub segments: Vec<segment::Segment32>,
}


impl ELF32 {
    pub fn new(elf_header: header::Ehdr32) -> Self {
        Self {
            ehdr: elf_header,
            // sections: Vec::new(),
            // segments: Vec::new(),
        }
    }
}