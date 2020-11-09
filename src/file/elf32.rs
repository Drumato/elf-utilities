use crate::{file, header, section, segment};

#[repr(C)]
#[derive(Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ELF32 {
    pub ehdr: header::Ehdr32,
    pub sections: Vec<section::Section32>,
    pub segments: Vec<segment::Segment32>,
}

impl file::ELF for ELF32 {
    type Header = header::Ehdr32;
    type Section = section::Section32;
    type Segment = segment::Segment32;

    fn new(elf_header: header::Ehdr32) -> Self {
        Self {
            ehdr: elf_header,
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }

    fn header(&self) -> Self::Header {
        self.ehdr
    }
    fn sections_as_mut(&mut self) -> &mut Vec<section::Section32> {
        &mut self.sections
    }
    fn update_sections(&mut self, sections: Vec<section::Section32>) {
        self.sections = sections;
    }
    fn update_segments(&mut self, segments: Vec<segment::Segment32>) {
        self.segments = segments;
    }
}

impl ELF32 {}
