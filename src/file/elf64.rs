use section::Section64;
use segment::Segment64;

use crate::{header, section, segment};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(C)]
pub struct ELF64 {
    pub ehdr: header::Ehdr64,
    pub sections: Vec<section::Section64>,
    pub segments: Vec<segment::Segment64>,
}

impl Default for ELF64 {
    fn default() -> Self {
        Self {
            ehdr: Default::default(),
            sections: {
                let mut scts = Vec::with_capacity(50);
                scts.push(section::Section64::new_null_section());
                scts
            },
            segments: Vec::with_capacity(10),
        }
    }
}

impl ELF64 {
    /// add a section with creating new entry of section table and etc.
    pub fn add_section(&mut self, mut sct: Section64) {
        // ehdr.e_shstrndxの変更のために計算
        let is_section_name_table = sct.name == ".shstrtab";

        // 新しいセクションのsh_name等を計算する為に
        // 現在の末尾のセクションを取得する
        let last_sct_idx = self.sections.len() - 1;
        fill_elf_info(&mut sct, &self.sections[last_sct_idx]);

        // セクションの追加 => SHTの開始オフセットが変更される
        self.ehdr.e_shoff += sct.header.sh_size;
        self.ehdr.e_shnum += 1;

        self.sections.push(sct);

        if is_section_name_table {
            self.ehdr.e_shstrndx = self.sections.len() as u16 - 1;
        }
    }

    pub fn add_segment(&mut self, sgt: Segment64) {
        // PHTに追加される => SHTのオフセットと各セクションのオフセットが変更される
        self.ehdr.e_shoff += segment::Phdr64::SIZE as u64;
        for sct in self.sections.iter_mut() {
            sct.header.sh_offset += segment::Phdr64::SIZE as u64;
        }
        self.ehdr.e_phnum += 1;

        self.segments.push(sgt);
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
}

/// sh_nameやsh_offset等の調整
fn fill_elf_info(new_sct: &mut Section64, prev_sct: &Section64) {
    let prev_name_idx = prev_sct.header.sh_name;
    let prev_name_len = prev_sct.name.as_bytes().len() as u32;
    let prev_offset = prev_sct.header.sh_offset;
    let prev_size = prev_sct.header.sh_size;

    // <prev_section_name> の後に0x00が入るので，+1
    new_sct.header.sh_name = prev_name_idx + prev_name_len + 1;
    new_sct.header.sh_offset = prev_offset + prev_size;
}
