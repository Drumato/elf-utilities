use crate::{file, header, section, segment};
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(C)]
pub struct ELF64 {
    pub ehdr: header::Ehdr64,
    pub sections: Vec<section::Section64>,
    pub segments: Vec<segment::Segment64>,
}

impl file::ELF for ELF64 {
    type Header = header::Ehdr64;
    type Section = section::Section64;
    type Segment = segment::Segment64;

    fn new(elf_header: header::Ehdr64) -> Self {
        Self {
            ehdr: elf_header,
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }

    fn sections_as_mut(&mut self) -> &mut Vec<section::Section64> {
        &mut self.sections
    }
    fn update_sections(&mut self, sections: Vec<section::Section64>) {
        self.sections = sections;
    }
    fn update_segments(&mut self, segments: Vec<segment::Segment64>) {
        self.segments = segments;
    }

    fn header(&self) -> header::Ehdr64 {
        self.ehdr
    }
}

impl ELF64 {
    /// add a section with creating new entry of section table and etc.
    pub fn add_section(&mut self, sct: section::Section64) {
        // .shstrtab を追加する場合,先にヘッダを変更する必要がある.
        let is_section_name_table = sct.name == ".shstrtab";

        self.sections.push(sct);

        if is_section_name_table {
            self.ehdr.e_shstrndx = self.sections.len() as u16 - 1;
        }

        if self.sections.len() == 1 {
            return;
        }

        let prev_sct_name_len = self.sections[self.sections.len() - 2].name.as_bytes().len() as u32;
        let prev_sct_name_offset = self.sections[self.sections.len() - 2].header.sh_name;

        let appended_section = self.sections.len() - 1;
        self.sections[appended_section].header.sh_name =
            prev_sct_name_offset + prev_sct_name_len + 1;
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

    pub fn finalize(&mut self) {
        self.ehdr.e_shentsize = section::Shdr64::size();
        self.ehdr.e_shnum = self.sections.len() as u16;
        self.ehdr.e_shstrndx = self.sections.len() as u16 - 1;

        self.ehdr.e_ehsize = header::Ehdr64::size();
        let shoff = header::Ehdr64::size() as u64 + self.all_section_size();
        self.ehdr.e_shoff = shoff;

        // セクションのオフセットを揃える
        let file_offset = self.ehdr.e_ehsize as u64;
        self.clean_sections_offset(file_offset);
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

    pub fn all_section_size(&self) -> u64 {
        self.sections.iter().map(|sct| sct.header.sh_size).sum()
    }

    fn clean_sections_offset(&mut self, base: u64) {
        let mut total = base;
        for section in self.sections.iter_mut() {
            section.header.sh_offset += total;
            total += section.header.sh_size;
        }
    }
}

pub struct ELF64Dumper {
    pub file: ELF64,
}

impl ELF64Dumper {
    pub fn new(f: ELF64) -> Self {
        Self { file: f }
    }

    pub fn generate_elf_file(
        &self,
        output_filename: &str,
        permission: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.file.to_le_bytes();

        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .mode(permission)
            .open(output_filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes)?;
        writer.flush()?;
        Ok(())
    }
}
