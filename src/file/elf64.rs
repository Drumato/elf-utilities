use crate::{header, section, segment};
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

#[repr(C)]
pub struct ELF64 {
    ehdr: header::Ehdr64,
    sections: Vec<section::Section64>,
    segments: Vec<segment::Segment64>,
}

impl ELF64 {
    pub fn new(elf_header: header::Ehdr64) -> Self {
        Self {
            ehdr: elf_header,
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }

    pub fn get_section_with_idx(&self, idx: usize) -> &section::Section64 {
        &self.sections[idx]
    }
    pub fn get_mut_section_with_idx(&mut self, idx: usize) -> &mut section::Section64 {
        &mut self.sections[idx]
    }

    /// get section index if predicate returns true.
    pub fn first_shidx_by<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(&section::Section64) -> bool,
    {
        for (i, sct) in self.sections.iter().enumerate() {
            if predicate(sct) {
                return Some(i);
            }
        }

        None
    }
    /// get a section if predicate returns true.
    pub fn first_section_by<P>(&self, predicate: P) -> Option<&section::Section64>
    where
        P: Fn(&section::Section64) -> bool,
    {
        match self.first_shidx_by(predicate) {
            Some(idx) => Some(&self.sections[idx]),
            None => None,
        }
    }
    /// get a mutable section if predicate returns true.
    pub fn first_mut_section_by<P>(&mut self, predicate: P) -> Option<&mut section::Section64>
    where
        P: Fn(&section::Section64) -> bool,
    {
        match self.first_shidx_by(predicate) {
            Some(idx) => Some(&mut self.sections[idx]),
            None => None,
        }
    }
    pub fn set_sections(&mut self, sections: Vec<section::Section64>) {
        self.sections = sections;
    }
    pub fn set_segments(&mut self, segments: Vec<segment::Segment64>) {
        self.segments = segments;
    }

    pub fn clone_sections(&self) -> Vec<section::Section64> {
        self.sections.clone()
    }

    pub fn iter_sections_as_mut(&mut self) -> std::slice::IterMut<section::Section64> {
        self.sections.iter_mut()
    }

    pub fn condition(&mut self) {
        self.ehdr.set_shentsize(section::Shdr64::size());
        self.ehdr.set_shnum(self.sections.len() as u16);
        self.ehdr.set_shstrndx(self.sections.len() as u16 - 1);

        self.ehdr.set_ehsize(header::Ehdr64::size());
        let shoff = header::Ehdr64::size() as u64 + self.all_section_size();
        self.ehdr.set_shoff(shoff);

        // セクションのオフセットを揃える
        let file_offset = header::Ehdr64::size() as u64;
        self.clean_sections_offset(file_offset);

        // セクション名を揃える
        let shstrndx = self.ehdr.get_shstrndx() as usize;
        let shnum = self.ehdr.get_shnum() as usize;

        let mut sh_name = 1;
        for (idx, bb) in self.sections[shstrndx]
            .bytes
            .as_ref()
            .unwrap()
            .to_vec()
            .splitn(shnum, |num| *num == 0x00)
            .enumerate()
        {
            if idx == 0 || idx >= shnum {
                continue;
            }
            let b: Vec<&u8> = bb
                .iter()
                .take_while(|num| *num != &0x00)
                .collect::<Vec<&u8>>();
            self.sections[idx].header.set_name(sh_name as u32);
            sh_name += b.len() as u32 + 1;
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut file_binary: Vec<u8> = Vec::new();

        let mut header_binary = self.ehdr.to_le_bytes();
        file_binary.append(&mut header_binary);

        for seg in self.segments.iter() {
            let mut phdr_binary = seg.header.to_le_bytes();
            file_binary.append(&mut phdr_binary);
        }

        for sct in self.sections.iter() {
            // セクションタイプによって処理を変える
            let mut section_binary = sct.to_le_bytes();
            file_binary.append(&mut section_binary);
        }

        for sct in self.sections.iter() {
            let mut shdr_binary = sct.header.to_le_bytes();
            file_binary.append(&mut shdr_binary);
        }
        file_binary
    }

    pub fn section_number(&self) -> usize {
        self.sections.len()
    }
    pub fn segment_number(&self) -> usize {
        self.segments.len()
    }
    pub fn add_section(&mut self, sct: section::Section64) {
        self.sections.push(sct);
    }
    pub fn get_ehdr(&self) -> &header::Ehdr64 {
        &self.ehdr
    }
    pub fn get_ehdr_as_mut(&mut self) -> &mut header::Ehdr64 {
        &mut self.ehdr
    }
    pub fn all_section_size(&self) -> u64 {
        self.sections.iter().map(|sct| sct.header.get_size()).sum()
    }
    pub fn add_segment(&mut self, seg: segment::Segment64) {
        self.segments.push(seg);
    }

    fn clean_sections_offset(&mut self, base: u64) {
        let mut total = base;
        for section in self.sections.iter_mut() {
            let sh_offset = section.header.get_offset();
            section.header.set_offset(sh_offset + total);

            let sh_size = section.header.get_size();
            total += sh_size;
        }
    }
}

pub struct ELF64Dumper {
    pub file: ELF64,
    permission: u32,
}

impl ELF64Dumper {
    pub fn new(f: ELF64, perm: u32) -> Self {
        Self {
            file: f,
            permission: perm,
        }
    }

    pub fn generate_elf_file(
        &self,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.file.to_le_bytes();

        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .mode(self.permission)
            .open(output_filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes)?;
        writer.flush()?;
        Ok(())
    }
}
