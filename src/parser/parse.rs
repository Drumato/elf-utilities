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

/// read 64bit ELF and construct `file::ELF64`
pub fn read_elf64(file_path: &str) -> Result<file::ELF64, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf);

    let _ = check_elf_magic(file_path, &buf[..4])?;

    let elf_header = parse_elf64_header(&buf[..header::Ehdr64::size() as usize])?;
    let phdr_table_exists = elf_header.e_phnum != 0;

    let mut elf_file = file::ELF64::new(elf_header);

    let sections = read_elf64_sections(&elf_file.ehdr, &buf)?;
    elf_file.sections = sections;

    if phdr_table_exists {
        let segments = read_elf64_segments(&elf_file.ehdr, &buf)?;
        elf_file.segments = segments;
    }
    set_section_names_from_shstrtab(&mut elf_file);

    Ok(elf_file)
}

fn read_elf64_sections(
    elf_header: &header::Ehdr64,
    buf: &[u8],
) -> Result<Vec<section::Section64>, Box<dyn std::error::Error>> {
    let mut sections: Vec<section::Section64> = Vec::new();

    for sct_idx in 0..elf_header.e_shnum {
        let shdr_result = bincode::deserialize(
            &buf[elf_header.e_shoff as usize
                + section::Shdr64::size() as usize * sct_idx as usize..],
        );
        if let Err(e) = shdr_result {
            return Err(Box::new(ReadELFError::CantParseSectionHeader { k: e }));
        }

        let shdr: section::Shdr64 = shdr_result.unwrap();
        let mut sct = section::Section64::new(String::new(), shdr);

        if sct.header.sh_size != 0 {
            let sct_type = sct.header.get_type();
            let start = sct.header.sh_offset as usize;

            match sct_type {
                section::TYPE::SYMTAB => {
                    let ent_size = sct.header.sh_entsize as usize;
                    let symbol_number = sct.header.sh_size as usize / ent_size;
                    let mut symbols = Vec::new();

                    for sym_idx in 0..symbol_number {
                        let sym_start = start + sym_idx * ent_size;
                        let sym_end = start + (sym_idx + 1) * ent_size;

                        let sym: symbol::Symbol64 =
                            match bincode::deserialize(&buf[sym_start..sym_end]) {
                                Ok(s) => s,
                                Err(e) => {
                                    return Err(Box::new(ReadELFError::CantParseSymbol { k: e }));
                                }
                            };
                        symbols.push(sym);
                    }

                    sct.symbols = Some(symbols);
                }

                section::TYPE::RELA => {
                    let ent_size = sct.header.sh_entsize as usize;
                    let symbol_number = sct.header.sh_size as usize / ent_size;
                    let mut symbols = Vec::new();

                    for sym_idx in 0..symbol_number {
                        let sym_start = start + sym_idx * ent_size;
                        let sym_end = start + (sym_idx + 1) * ent_size;

                        let sym: relocation::Rela64 =
                            match bincode::deserialize(&buf[sym_start..sym_end]) {
                                Ok(s) => s,
                                Err(e) => {
                                    return Err(Box::new(ReadELFError::CantParseSymbol { k: e }));
                                }
                            };
                        symbols.push(sym);
                    }

                    sct.rela_symbols = Some(symbols);
                }

                // 通常通りバイト列として処理
                _ => {
                    sct.bytes = Some(buf[start..start + sct.header.sh_size as usize].to_vec());
                }
            }
        }

        // sct.shdrがシンボルテーブルなどの場合,追加の処理が必要となる
        sections.push(sct);
    }

    Ok(sections)
}

fn read_elf64_segments(
    elf_header: &header::Ehdr64,
    buf: &[u8],
) -> Result<Vec<segment::Segment64>, Box<dyn std::error::Error>> {
    let mut segments: Vec<segment::Segment64> = Vec::new();

    for seg_idx in 0..elf_header.e_phnum {
        let phdr_result = bincode::deserialize(
            &buf[elf_header.e_phoff as usize
                + segment::Phdr64::size() as usize * seg_idx as usize..],
        );
        if let Err(e) = phdr_result {
            return Err(Box::new(ReadELFError::CantParseProgramHeader { k: e }));
        }

        let phdr = phdr_result.unwrap();
        let seg = segment::Segment64::new(phdr);
        segments.push(seg);
    }

    Ok(segments)
}

fn set_section_names_from_shstrtab(elf_file: &mut file::ELF64) {
    let shstrndx = elf_file.ehdr.e_shstrndx as usize;
    let shnum = elf_file.ehdr.e_shnum as usize;

    let mut sh_name: u32 = 1;
    for idx in 0..shnum {
        if idx == 0 || idx >= shnum {
            continue;
        }

        let name_idx = elf_file.sections[idx].header.sh_name;

        let name_bytes: Vec<u8> = elf_file.sections[shstrndx].bytes.as_ref().unwrap()
            [name_idx as usize..]
            .to_vec()
            .iter()
            .take_while(|byte| **byte != 0x00)
            .map(|byte| *byte)
            .collect();

        elf_file.sections[idx].name = std::str::from_utf8(&name_bytes).unwrap().to_string();
        elf_file.sections[idx].header.sh_name = sh_name as u32;
        sh_name += name_bytes.len() as u32 + 1;
    }
}

fn check_elf_magic(file_path: &str, header: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(header.len(), 4);

    if header[0] != 0x7f || header[1] != 0x45 || header[2] != 0x4c || header[3] != 0x46 {
        return Err(Box::new(ReadELFError::NotELF {
            file_path: file_path.to_string(),
        }));
    }

    Ok(())
}

fn parse_elf64_header(header: &[u8]) -> Result<header::Ehdr64, Box<dyn std::error::Error>> {
    assert_eq!(header.len(), header::Ehdr64::size() as usize);

    match bincode::deserialize(header) {
        Ok(ehdr) => Ok(ehdr),
        Err(e) => Err(Box::new(ReadELFError::CantParseELFHeader { k: e })),
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

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
        let hdr_result = parse_elf64_header(&header_bytes);
        assert!(hdr_result.is_ok());
        let hdr_result = hdr_result.unwrap();

        assert_eq!(hdr_result.get_elf_type(), header::ELFTYPE::DYN);
        assert_eq!(hdr_result.e_entry, 0xe160);
        assert_eq!(hdr_result.e_shnum, 44);
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
        assert_eq!(f.sections[1].header.get_type(), section::TYPE::PROGBITS);
        assert_eq!(f.sections[1].header.sh_addr, 0x318);
        assert_eq!(f.sections[1].header.sh_offset, 0x318);
        assert_eq!(f.sections[1].header.sh_addralign, 0x1);
        assert_eq!(
            f.sections[1].header.sh_flags,
            section::section_flag::SHF_ALLOC
        );
        assert_eq!(f.sections[1].header.sh_size, 0x1c);
        assert!(f.sections[1].bytes.is_some());
        assert_eq!(
            f.sections[1].bytes.as_ref().unwrap().len(),
            f.sections[1].header.sh_size as usize
        );

        assert_eq!(f.sections[2].header.get_type(), section::TYPE::NOTE);
        assert_eq!(f.sections[2].header.sh_addr, 0x338);
        assert!(f.sections[2].bytes.is_some());
        assert_eq!(
            f.sections[2].bytes.as_ref().unwrap().len(),
            f.sections[2].header.sh_size as usize
        );

        assert_eq!(f.sections[10].header.get_type(), section::TYPE::RELA);
        assert_eq!(f.sections[10].rela_symbols.as_ref().unwrap().len(), 8);
        assert_eq!(f.sections[26].header.get_type(), section::TYPE::SYMTAB);
        assert_eq!(f.sections[26].symbols.as_ref().unwrap().len(), 62);

        assert_eq!(f.segments[0].header.get_type(), segment::TYPE::PHDR);
        assert_eq!(f.segments[0].header.p_flags, segment::segment_flag::PF_R);
        assert_eq!(f.segments[0].header.p_align, 8);

        assert_eq!(f.segments[1].header.get_type(), segment::TYPE::INTERP);
        assert_eq!(f.segments[1].header.p_flags, segment::segment_flag::PF_R);
        assert_eq!(f.segments[1].header.p_align, 1);
    }
}
