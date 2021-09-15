use crate::{
    header,
    section::{self, Section32},
    segment,
};

#[repr(C)]
#[derive(Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ELF32 {
    pub ehdr: header::Ehdr32,
    pub sections: Vec<section::Section32>,
    pub segments: Vec<segment::Segment32>,
}

impl ELF32 {
    /// add a section with creating new entry of section table and etc.
    pub fn add_section(&mut self, mut sct: section::Section32) {
        // ehdr.e_shstrndxの変更のために計算
        let is_section_name_table = sct.name == ".shstrtab";

        // 新しいセクションのsh_name等を計算する為に
        // 現在の末尾のセクションを取得する
        let last_sct_idx = self.sections.len() - 1;
        self.fill_elf_info(&mut sct, last_sct_idx, &self.sections[last_sct_idx]);

        // セクションの追加 => SHTの開始オフセットが変更される
        self.ehdr.e_shoff += sct.header.sh_size;
        self.ehdr.e_shnum += 1;

        self.sections.push(sct);

        if is_section_name_table {
            self.ehdr.e_shstrndx = self.sections.len() as u16 - 1;
        }
    }

    pub fn add_segment(&mut self, sgt: segment::Segment32) {
        // PHTに追加される => SHTのオフセットと各セクションのオフセットが変更される
        self.ehdr.e_shoff += segment::Phdr32::SIZE as u32;
        for sct in self.sections.iter_mut() {
            sct.header.sh_offset += segment::Phdr32::SIZE as u32;
        }
        self.ehdr.e_phnum += 1;

        self.segments.push(sgt);
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
    fn fill_elf_info(&self, new_sct: &mut Section32, prev_sct_idx: usize, prev_sct: &Section32) {
        let prev_name_idx = prev_sct.header.sh_name;
        let prev_name_len = prev_sct.name.as_bytes().len() as u32;
        let prev_offset = prev_sct.header.sh_offset;
        let prev_size = prev_sct.header.sh_size;

        // <prev_section_name> の後に0x00が入るので，+1
        new_sct.header.sh_name = prev_name_idx + prev_name_len + 1;

        // NULLセクションのすぐ次に挿入する場合，
        // sh_offsetはEhdr32::SIZE + PHT's SIZEという感じになる．
        if prev_sct_idx == 0 {
            new_sct.header.sh_offset = header::Ehdr32::SIZE as u32
                + segment::Phdr32::SIZE as u32 * self.segments.len() as u32;
        } else {
            new_sct.header.sh_offset = prev_offset + prev_size;
        }

        new_sct.header.sh_size = new_sct.contents.size() as u32;
    }
}
