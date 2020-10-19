use crate::*;
use std::fs::File;
use std::io::Read;

use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum ReadELFError {
    #[error("input file `{file_path}` is not an ELF file")]
    NotELF { file_path: String },
    #[error("can't parse elf header => `{k}`")]
    CantParseELFHeader { k: Box<dyn std::error::Error> },
    #[error("can't parse section header => `{k}`")]
    CantParseSectionHeader { k: Box<dyn std::error::Error> },
    #[error("can't parse program header => `{k}`")]
    CantParseProgramHeader { k: Box<dyn std::error::Error> },
    #[error("can't parse symbol => `{k}`")]
    CantParseSymbol { k: Box<dyn std::error::Error> },
}

/// read 32bit ELF and construct `file::ELF32`
pub fn read_elf32(file_path: &str) -> Result<file::ELF32, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf);

    let _ = check_elf_magic(file_path, &buf[..4])?;

    let elf_header: header::Ehdr32 = parse_elf_header(&buf);
    let phdr_table_exists = elf_header.e_phnum != 0;

    let mut elf_file = file::ELF32::new(elf_header);

    let sections = read_elf_sections(
        elf_file.ehdr.e_shnum as usize,
        elf_file.ehdr.e_shoff as usize,
        &buf,
    )?;
    elf_file.sections = sections;

    if phdr_table_exists {
        let segments = read_elf_segments(
            elf_file.ehdr.e_phnum as usize,
            elf_file.ehdr.e_phoff as usize,
            &buf,
        )?;
        elf_file.segments = segments;
    }

    set_sections_name_shstrtab(
        elf_file.ehdr.e_shstrndx as usize,
        elf_file.ehdr.e_shnum as usize,
        &mut elf_file.sections,
    );

    Ok(elf_file)
}

/// read 64bit ELF and construct `file::ELF64`
pub fn read_elf64(file_path: &str) -> Result<file::ELF64, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf);

    let _ = check_elf_magic(file_path, &buf[..4])?;

    let elf_header: header::Ehdr64 = parse_elf_header(&buf);
    let phdr_table_exists = elf_header.e_phnum != 0;

    let mut elf_file = file::ELF64::new(elf_header);

    let sections: Vec<section::Section64> = read_elf_sections(
        elf_file.ehdr.e_shnum as usize,
        elf_file.ehdr.e_shoff as usize,
        &buf,
    )?;
    elf_file.sections = sections;

    if phdr_table_exists {
        let segments: Vec<segment::Segment64> = read_elf_segments(
            elf_file.ehdr.e_phnum as usize,
            elf_file.ehdr.e_phoff as usize,
            &buf,
        )?;
        elf_file.segments = segments;
    }
    set_sections_name_shstrtab(
        elf_file.ehdr.e_shstrndx as usize,
        elf_file.ehdr.e_shnum as usize,
        &mut elf_file.sections,
    );

    Ok(elf_file)
}

fn read_elf_sections<T: section::Section>(
    section_number: usize,
    sht_offset: usize,
    buf: &[u8],
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut sections: Vec<T> = Vec::new();

    for sct_idx in 0..section_number {
        let header_start = sht_offset + T::header_size() * sct_idx;
        let shdr = T::header_deserialize(buf, header_start)?;

        let mut sct = T::new(shdr);

        if sct.section_type() != section::Type::NoBits {
            let section_offset = sct.offset();

            sct.update_contents(
                buf[section_offset..section_offset + sct.section_size() as usize].to_vec(),
            );
        }

        sections.push(sct);
    }

    Ok(sections)
}

fn read_elf_segments<T: segment::Segment>(
    segment_number: usize,
    pht_offset: usize,
    buf: &[u8],
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut segments: Vec<T> = Vec::new();

    for seg_idx in 0..segment_number {
        let header_start = pht_offset as usize + T::header_size() * seg_idx;
        let phdr = T::header_deserialize(buf, header_start)?;

        let seg = T::new(phdr);
        segments.push(seg);
    }

    Ok(segments)
}

fn set_sections_name_shstrtab<T: section::Section>(
    shstrndx: usize,
    section_number: usize,
    sections: &mut Vec<T>,
) {
    for idx in 0..section_number {
        if idx == 0 || idx >= section_number {
            continue;
        }

        let name_idx = sections[idx].name_idx();

        let name_bytes = sections[shstrndx].clone_contents();
        let name_bytes: Vec<u8> = name_bytes[name_idx as usize..]
            .to_vec()
            .iter()
            .take_while(|byte| **byte != 0x00)
            .map(|byte| *byte)
            .collect();

        sections[idx].update_name(std::str::from_utf8(&name_bytes).unwrap().to_string());
    }
}

fn check_elf_magic(file_path: &str, buf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(buf.len(), 4);

    if buf[0] != 0x7f || buf[1] != 0x45 || buf[2] != 0x4c || buf[3] != 0x46 {
        return Err(Box::new(ReadELFError::NotELF {
            file_path: file_path.to_string(),
        }));
    }

    Ok(())
}

fn parse_elf_header<T: header::ELFHeader>(buf: &[u8]) -> T {
    T::deserialize(buf)
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use section::Section;

    #[test]
    fn check_elf_magic_test() {
        assert!(check_elf_magic("", &[0x7f, 0x45, 0x4c, 0x46]).is_ok());
        assert!(check_elf_magic("", &[0x7f, 0x45, 0x4b, 0x46]).is_err());
        assert!(check_elf_magic("", &[0x7f, 0x42, 0x43, 0x46]).is_err());
    }

    #[test]
    fn parse_elf64_header_test() {
        let header_bytes = vec![
            0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x3e, 0x00, 0x01, 0x00, 0x00, 0x00, 0x60, 0xe1, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x1d,
            0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x38, 0x00,
            0x0c, 0x00, 0x40, 0x00, 0x2c, 0x00, 0x2b, 0x00,
        ];
        let hdr_result: header::Ehdr64 = parse_elf_header(&header_bytes);

        assert_eq!(hdr_result.get_type(), header::Type::Dyn);
        assert_eq!(hdr_result.e_entry, 0xe160);
        assert_eq!(hdr_result.e_shnum, 44);
    }

    #[test]
    fn parse_elf32_header_test() {
        let header_bytes = vec![
            0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x90, 0x10, 0x00, 0x00,
            0x34, 0x00, 0x00, 0x00, 0xe4, 0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x00,
            0x20, 0x00, 0x0c, 0x00, 0x28, 0x00, 0x1f, 0x00, 0x1e, 0x00, 0x06, 0x00, 0x34, 0x00,
            0x00, 0x00, 0x40, 0x00, 0x2c, 0x00, 0x2b, 0x00,
        ];
        let hdr_result: header::Ehdr32 = parse_elf_header(&header_bytes);

        assert_eq!(hdr_result.get_type(), header::Type::Dyn);
        assert_eq!(hdr_result.e_entry, 0x1090);
        assert_eq!(hdr_result.e_shnum, 31);
    }

    #[test]
    fn read_elf64_test() {
        let f_result = read_elf64("examples/sample");
        assert!(f_result.is_ok());
        let f = f_result.unwrap();

        assert_eq!(f.ehdr.e_entry, 0x1040);
        assert_eq!(f.ehdr.e_shnum, 29);
        assert_eq!(f.ehdr.e_shstrndx, 28);

        assert_eq!(f.sections.len(), 29);
        assert_eq!(f.segments.len(), 13);

        assert_eq!(".interp", &f.sections[1].name);
        assert_eq!(f.sections[1].header.get_type(), section::Type::ProgBits);
        assert_eq!(f.sections[1].header.sh_addr, 0x318);
        assert_eq!(f.sections[1].header.sh_offset, 0x318);
        assert_eq!(f.sections[1].header.sh_addralign, 0x1);
        assert_eq!(f.sections[1].header.sh_flags, section::SHF_ALLOC);
        assert_eq!(f.sections[1].header.sh_size, 0x1c);
        assert!(!f.sections[1].bytes.is_empty());
        assert_eq!(
            f.sections[1].bytes.len(),
            f.sections[1].header.sh_size as usize
        );

        assert_eq!(f.sections[2].header.get_type(), section::Type::Note);
        assert_eq!(f.sections[2].header.sh_addr, 0x338);
        assert!(!f.sections[2].bytes.is_empty());
        assert_eq!(
            f.sections[2].bytes.len(),
            f.sections[2].header.sh_size as usize
        );

        let rela_symbols = f.sections[10].parse_bytes_as_relas();
        let symbols = f.sections[26].parse_bytes_as_symbols(&f.sections[27]);
        let dynamics = f.sections[21].parse_bytes_as_dynamics();

        assert_eq!(f.sections[10].header.get_type(), section::Type::Rela);
        assert_eq!(rela_symbols.len(), 8);
        assert_eq!(f.sections[26].header.get_type(), section::Type::SymTab);
        assert_eq!(symbols.len(), 62);
        assert!(symbols[26].symbol_name.is_some());
        assert_eq!(symbols[26].symbol_name.as_ref().unwrap(), "crtstuff.c");
        assert_eq!(
            symbols[45].symbol_name.as_ref().unwrap(),
            "_ITM_deregisterTMCloneTable"
        );

        assert_eq!(f.sections[21].header.get_type(), section::Type::Dynamic);
        assert_eq!(dynamics[1].get_type(), dynamic::EntryType::Init);
        assert_eq!(dynamics[2].get_type(), dynamic::EntryType::Fini);

        assert_eq!(f.segments[0].header.get_type(), segment::Type::Phdr);
        assert_eq!(f.segments[0].header.p_flags, segment::PF_R);
        assert_eq!(f.segments[0].header.p_align, 8);

        assert_eq!(f.segments[1].header.get_type(), segment::Type::Interp);
        assert_eq!(f.segments[1].header.p_flags, segment::PF_R);
        assert_eq!(f.segments[1].header.p_align, 1);
    }

    #[test]
    fn read_elf64_test_2() {
        let f_result = read_elf64("/bin/ls");
        assert!(f_result.is_ok());
    }
    #[test]
    fn read_elf32_test() {
        let f_result = read_elf32("examples/32bit");
        assert!(f_result.is_ok());

        let f: file::ELF32 = f_result.unwrap();

        assert_eq!(header::Type::Dyn, f.ehdr.get_type());
        assert_eq!(0x1090, f.ehdr.e_entry);
        assert_eq!(32, f.ehdr.e_phentsize);
        assert_eq!(40, f.ehdr.e_shentsize);
        assert_eq!(30, f.ehdr.e_shstrndx);

        assert_eq!(".interp", f.sections[1].name);
        assert_eq!(0x1b4, f.sections[1].header.sh_addr);
        assert_eq!(0x13, f.sections[1].header.sh_size);

        assert_eq!(".note.ABI-tag", f.sections[4].name);
        assert_eq!(0x208, f.sections[4].header.sh_addr);
    }
}
