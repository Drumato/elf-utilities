use section::Section64;
use segment::Segment64;

use crate::{
    header,
    section::{self, Contents64, StrTabEntry},
    segment,
};

const SHSTRTAB_INITIAL_SIZE: usize = 0xb;

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
            ehdr: {
                let mut hdr = header::Ehdr64::default();
                hdr.e_shnum = 2;
                hdr.e_shstrndx = 1;
                hdr.e_shoff += SHSTRTAB_INITIAL_SIZE as u64;

                hdr
            },
            sections: {
                let mut scts = Vec::with_capacity(50);
                scts.push(section::Section64::new_null_section());

                let shstrtab_contents = Contents64::new_string_table(vec![".shstrtab".to_string()]);
                scts.push(section::Section64 {
                    name: ".shstrtab".to_string(),
                    header: section::Shdr64 {
                        sh_name: 1,
                        sh_type: section::Type::StrTab.into(),
                        sh_flags: 0,
                        sh_addr: 0,
                        sh_offset: header::Ehdr64::SIZE as u64,
                        sh_size: shstrtab_contents.size() as u64,
                        sh_link: 0,
                        sh_info: 0,
                        sh_addralign: 1,
                        sh_entsize: 0,
                    },
                    contents: shstrtab_contents,
                });
                scts
            },
            segments: Vec::with_capacity(10),
        }
    }
}

impl ELF64 {
    /// add a section with creating new entry of section table and etc.
    pub fn add_section(&mut self, mut sct: Section64) {
        // 新しいセクションのsh_name等を計算する為に
        // 現在の末尾のセクションを取得する
        // 本当の末尾には.shstrtabが存在するので，その一つ前
        let last_sct_idx = self.sections.len() - 2;

        self.fill_elf_info(&mut sct, last_sct_idx);

        // セクションの追加 => SHTの開始オフセットが変更される
        self.ehdr.e_shoff += sct.header.sh_size;
        self.ehdr.e_shnum += 1;
        self.ehdr.e_shstrndx += 1;

        self.sections.insert(self.sections.len() - 1, sct);
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

        let mut sections = self.sections.clone();
        sections.sort_by_key(|sct| sct.header.sh_offset);

        for (idx, sct) in sections.iter().enumerate() {
            let mut section_binary = sct.to_le_bytes();
            if idx >= 1 && sct.header.sh_addralign > 1 {
                if sct.header.sh_addr == 0 {
                    if file_binary.len() < sct.header.sh_offset as usize {
                        file_binary.append(&mut vec![
                            0x00;
                            sct.header.sh_offset as usize
                                - file_binary.len()
                        ]);
                    }
                } else {
                    if file_binary.len() < sct.header.sh_addr as usize {
                        file_binary.append(&mut vec![
                            0x00;
                            sct.header.sh_addr as usize
                                - file_binary.len()
                        ]);
                    }
                };
            }

            file_binary.append(&mut section_binary);
        }

        if file_binary.len() < self.ehdr.e_shoff as usize {
            file_binary.append(&mut vec![
                0x00;
                self.ehdr.e_shoff as usize - file_binary.len()
            ]);
        }

        for sct in self.sections.iter() {
            let mut shdr_binary = sct.header.to_le_bytes();
            file_binary.append(&mut shdr_binary);
        }
        file_binary
    }

    /// sh_nameやsh_offset等の調整
    fn fill_elf_info(&mut self, new_sct: &mut Section64, prev_sct_idx: usize) {
        let shstrtab_len = self.sections[self.ehdr.e_shstrndx as usize].contents.size() as usize;
        let prev_offset = self.sections[prev_sct_idx].header.sh_offset;
        let prev_size = self.sections[prev_sct_idx].header.sh_size;

        // <prev_section_name> の後に0x00が入るので，+1
        new_sct.header.sh_name = shstrtab_len as u32 + 1;
        // .shstrtabの更新
        if let Contents64::StrTab(ref mut tab) =
            self.sections[self.ehdr.e_shstrndx as usize].contents
        {
            tab.push(StrTabEntry {
                v: new_sct.name.clone(),
                idx: shstrtab_len + 1,
            });
        }

        // NULLセクションのすぐ次に挿入する場合，
        // sh_offsetはEhdr64::SIZE + PHT's SIZEという感じになる．
        // .shstrtabが既に存在するがサイズは固定なので，その分足しておく
        if prev_sct_idx == 0 {
            new_sct.header.sh_offset = header::Ehdr64::SIZE as u64
                + segment::Phdr64::SIZE as u64 * self.segments.len() as u64
                + SHSTRTAB_INITIAL_SIZE as u64;
        } else {
            new_sct.header.sh_offset = prev_offset + prev_size;
        }

        new_sct.header.sh_size = new_sct.contents.size() as u64;
    }
}
