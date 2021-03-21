use crate::*;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
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

/// parse 64bit ELF
pub fn parse_elf64(file_path: &str) -> Result<file::ELF64, Box<dyn std::error::Error>> {
    Ok(parse_elf(file_path)?.as_64bit())
}
/// parse 32bit ELF
pub fn parse_elf32(file_path: &str) -> Result<file::ELF32, Box<dyn std::error::Error>> {
    Ok(parse_elf(file_path)?.as_32bit())
}

/// parse ELF and construct `file::ELF`
pub fn parse_elf(file_path: &str) -> Result<file::ELF, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf);

    let _ = check_elf_magic(file_path, &buf[..4])?;

    // 32bit/64bitでパース処理を共通化するため，classを取っておく
    let elf_class = header::Class::from(buf[header::Class::INDEX]);

    let elf_header = parse_elf_header(elf_class, &buf)?;
    let phdr_table_exists = elf_header.pht_exists();

    let mut sections = read_sht(elf_class, elf_header.shnum(), elf_header.sht_start(), &buf)?;
    let mut segments = Vec::new();

    if phdr_table_exists {
        segments = read_pht(elf_class, elf_header.phnum(), elf_header.pht_start(), &buf)?;
    }

    // セクション名の設定
    // .shstrtabセクションは大抵SHTの末尾にあるため，read_sht() 後に行う必要がある
    naming_sections_from_shstrtab(elf_header.shstrndx(), &mut sections);

    // シンボル名の設定
    // これもセクション名の設定と同様，SHTパース後に実行する必要があるため切り離している
    naming_symbols(&mut sections);

    match elf_class {
        header::Class::Bit64 => Ok(file::ELF::ELF64(file::ELF64 {
            ehdr: elf_header.as_64bit(),
            sections: sections.iter().map(|sct| sct.as_64bit()).collect(),
            segments: segments.iter().map(|sgt| sgt.as_64bit()).collect(),
        })),
        header::Class::Bit32 => Ok(file::ELF::ELF32(file::ELF32 {
            ehdr: elf_header.as_32bit(),
            sections: sections.iter().map(|sct| sct.as_32bit()).collect(),
            segments: segments.iter().map(|sgt| sgt.as_32bit()).collect(),
        })),
        _ => todo!(),
    }
}

/// セクションヘッダテーブルのパース
fn read_sht(
    class: header::Class,

    section_number: usize,
    sht_offset: usize,
    buf: &[u8],
) -> Result<Vec<section::Section>, Box<dyn std::error::Error>> {
    let mut sections = Vec::with_capacity(50);
    let shdr_size = match class {
        header::Class::Bit32 => section::Shdr32::SIZE,
        header::Class::Bit64 => section::Shdr64::SIZE,
        _ => todo!(),
    };

    for sct_idx in 0..section_number {
        let header_start = sht_offset + shdr_size * sct_idx;
        let shdr = match class {
            header::Class::Bit32 => {
                section::Shdr::Shdr32(bincode::deserialize(&buf[header_start..])?)
            }
            header::Class::Bit64 => {
                section::Shdr::Shdr64(bincode::deserialize(&buf[header_start..])?)
            }
            _ => todo!(),
        };

        let mut sct = section::Section::new(shdr);
        let section_type = sct.ty();

        if section_type != section::Type::NoBits {
            let section_offset = sct.offset();
            let section_raw_contents =
                buf[section_offset..section_offset + sct.size() as usize].to_vec();

            sct.contents = match section_type {
                section::Type::StrTab => parse_string_table(class, &section_raw_contents),
                section::Type::SymTab | section::Type::DynSym => {
                    parse_symbol_table(class, &sct, &section_raw_contents)
                }
                section::Type::Rela => parse_rela_symbol_table(class, &sct, &section_raw_contents),
                section::Type::Dynamic => {
                    parse_dynamic_information(class, &sct, &section_raw_contents)
                }
                _ => match class {
                    header::Class::Bit32 => section::Contents::Contents32(
                        section::Contents32::Raw(section_raw_contents),
                    ),
                    header::Class::Bit64 => section::Contents::Contents64(
                        section::Contents64::Raw(section_raw_contents),
                    ),
                    _ => todo!(),
                },
            }
        }

        sections.push(sct);
    }

    Ok(sections)
}

fn parse_string_table(class: header::Class, section_raw_contents: &Vec<u8>) -> section::Contents {
    let mut strs: Vec<section::StrTabEntry> = Default::default();
    let mut name_idx = 0;
    loop {
        if name_idx >= section_raw_contents.len() {
            break;
        }

        if section_raw_contents[name_idx] == 0x00 {
            name_idx += 1;
            continue;
        }

        let nul_range_end = section_raw_contents[name_idx..]
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(section_raw_contents.len());
        let s = std::str::from_utf8(&section_raw_contents[name_idx..name_idx + nul_range_end])
            .unwrap()
            .to_string();

        let idx = name_idx;
        name_idx += s.len();
        strs.push(section::StrTabEntry { v: s, idx });
    }

    match class {
        header::Class::Bit32 => section::Contents::Contents32(section::Contents32::StrTab(strs)),
        header::Class::Bit64 => section::Contents::Contents64(section::Contents64::StrTab(strs)),
        _ => todo!(),
    }
}
fn parse_rela_symbol_table(
    class: header::Class,
    sct: &section::Section,
    raw_symtab: &Vec<u8>,
) -> section::Contents {
    let entry_size = sct.entry_size();
    let entry_number = sct.size() / entry_size;
    match class {
        header::Class::Bit32 => section::Contents::Contents32(section::Contents32::RelaSymbols(
            parse_table(entry_size, entry_number, raw_symtab),
        )),
        header::Class::Bit64 => section::Contents::Contents64(section::Contents64::RelaSymbols(
            parse_table(entry_size, entry_number, raw_symtab),
        )),
        _ => todo!(),
    }
}

fn parse_dynamic_information(
    class: header::Class,
    sct: &section::Section,
    raw_symtab: &Vec<u8>,
) -> section::Contents {
    let entry_size = sct.entry_size();
    let entry_number = sct.size() / entry_size;
    match class {
        header::Class::Bit32 => section::Contents::Contents32(section::Contents32::Dynamics(
            parse_table(entry_size, entry_number, raw_symtab),
        )),
        header::Class::Bit64 => section::Contents::Contents64(section::Contents64::Dynamics(
            parse_table(entry_size, entry_number, raw_symtab),
        )),
        _ => todo!(),
    }
}

fn parse_symbol_table(
    class: header::Class,
    sct: &section::Section,
    raw_symtab: &Vec<u8>,
) -> section::Contents {
    let entry_size = sct.entry_size();
    let entry_number = sct.size() / entry_size;
    match class {
        header::Class::Bit32 => section::Contents::Contents32(section::Contents32::Symbols(
            parse_table(entry_size, entry_number, raw_symtab),
        )),
        header::Class::Bit64 => section::Contents::Contents64(section::Contents64::Symbols(
            parse_table(entry_size, entry_number, raw_symtab),
        )),
        _ => todo!(),
    }
}

fn parse_table<'a, T: Deserialize<'a>>(
    entry_size: usize,
    entry_number: usize,
    buf: &'a [u8],
) -> Vec<T> {
    let mut table = Vec::new();
    for idx in 0..entry_number {
        let start = idx * entry_size;
        let end = (idx + 1) * entry_size;
        let entry = bincode::deserialize(&buf[start..end]).unwrap();
        table.push(entry);
    }
    table
}

/// プログラムヘッダテーブルのパース
fn read_pht(
    class: header::Class,
    phnum: usize,
    pht_start: usize,
    buf: &[u8],
) -> Result<Vec<segment::Segment>, Box<dyn std::error::Error>> {
    let mut segments = Vec::with_capacity(10);
    let phdr_size = match class {
        header::Class::Bit32 => segment::Phdr32::SIZE,
        header::Class::Bit64 => segment::Phdr64::SIZE,
        _ => todo!(),
    };

    for seg_idx in 0..phnum {
        let header_start = pht_start as usize + phdr_size * seg_idx;
        let phdr = match class {
            header::Class::Bit32 => {
                segment::Phdr::Phdr32(segment::Phdr32::deserialize(buf, header_start)?)
            }
            header::Class::Bit64 => {
                segment::Phdr::Phdr64(segment::Phdr64::deserialize(buf, header_start)?)
            }
            _ => todo!(),
        };

        let seg = segment::Segment { phdr };
        segments.push(seg);
    }

    Ok(segments)
}

/// セクション名を.shstrtabから探して，Section構造体に書き込む
/// このようにしているのは，SHTのパースがすべて終わってからでないとshstrtabを使用できない為
fn naming_sections_from_shstrtab(shstrndx: usize, sections: &mut Vec<section::Section>) {
    let shstrtab = sections[shstrndx].contents.as_strtab();

    for sct in sections.iter_mut() {
        let name_idx = sct.name_idx();
        if name_idx == 0 {
            continue;
        }

        let s = shstrtab
            .iter()
            .find(|&s| s.idx <= name_idx && name_idx <= s.idx + s.v.len())
            .unwrap();

        let (_, name) = s.v.split_at(name_idx - s.idx);
        sct.name = name.to_string();
    }
}

/// シンボル名をsh_linkが指す文字列テーブルから探して割り当てる
/// このようにしているのは，SHTのパースがすべて終わってからでないとshstrtabを使用できない為
fn naming_symbols(sections: &mut Vec<section::Section>) {
    let section_number = sections.len();
    for sct_idx in 0..section_number {
        let sct = &sections[sct_idx];
        if sct.ty() != section::Type::SymTab && sct.ty() != section::Type::DynSym {
            continue;
        }

        let strtab = sections[sct.link()].contents.as_strtab();

        match &mut sections[sct_idx].contents {
            section::Contents::Contents32(c) => {
                if let section::Contents32::Symbols(ref mut symbols) = c {
                    for sym in symbols.iter_mut() {
                        let name_idx = sym.st_name as usize;
                        if name_idx == 0 {
                            continue;
                        }

                        let s = strtab
                            .iter()
                            .find(|s| s.idx <= name_idx && name_idx <= s.idx + s.v.len())
                            .unwrap();
                        let (_, name) = s.v.split_at(name_idx - s.idx);

                        sym.symbol_name = name.to_string();
                    }
                }
            }

            section::Contents::Contents64(c) => {
                if let section::Contents64::Symbols(ref mut symbols) = c {
                    for sym in symbols.iter_mut() {
                        let name_idx = sym.st_name as usize;
                        if name_idx == 0 {
                            continue;
                        }

                        let s = strtab
                            .iter()
                            .find(|s| s.idx <= name_idx && name_idx <= s.idx + s.v.len())
                            .unwrap();
                        let (_, name) = s.v.split_at(name_idx - s.idx);

                        sym.symbol_name = name.to_string();
                    }
                }
            }
        }
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

fn parse_elf_header(
    class: header::Class,
    buf: &[u8],
) -> Result<header::Ehdr, Box<dyn std::error::Error>> {
    match class {
        header::Class::Bit32 => Ok(header::Ehdr::Ehdr32(bincode::deserialize(buf)?)),
        header::Class::Bit64 => Ok(header::Ehdr::Ehdr64(bincode::deserialize(buf)?)),
        _ => todo!(),
    }
}

#[cfg(test)]
mod parse_tests {
    use crate::section::Contents64;

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
        let hdr_result = parse_elf_header(header::Class::Bit64, &header_bytes).unwrap();
        assert!(matches!(hdr_result, header::Ehdr::Ehdr64(_)));

        if let header::Ehdr::Ehdr64(ehdr) = hdr_result {
            assert_eq!(ehdr.get_type(), header::Type::Dyn);
            assert_eq!(ehdr.e_entry, 0xe160);
            assert_eq!(ehdr.e_shnum, 44);
        }
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
        let hdr_result = parse_elf_header(header::Class::Bit32, &header_bytes).unwrap();
        assert!(matches!(hdr_result, header::Ehdr::Ehdr32(_)));

        if let header::Ehdr::Ehdr32(ehdr) = hdr_result {
            assert_eq!(ehdr.get_type(), header::Type::Dyn);
            assert_eq!(ehdr.e_entry, 0x1090);
            assert_eq!(ehdr.e_shnum, 31);
        }
    }

    #[test]
    fn read_elf64_test() {
        let f_result = parse_elf("src/parser/testdata/sample");
        assert!(f_result.is_ok());
        let f = f_result.unwrap();
        assert!(matches!(f, file::ELF::ELF64(_)));
        if let file::ELF::ELF64(f) = f {
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
            assert!(f.sections[1]
                .header
                .get_flags()
                .contains(&section::Flag::Alloc));
            assert_eq!(f.sections[1].header.sh_size, 0x1c);
            assert!(
                matches!(&f.sections[1].contents, Contents64::Raw(x) if x.len() == f.sections[1].header.sh_size as usize )
            );

            assert_eq!(f.sections[2].header.get_type(), section::Type::Note);
            assert_eq!(f.sections[2].header.sh_addr, 0x338);
            assert!(
                matches!(&f.sections[2].contents, Contents64::Raw(x) if x.len() == f.sections[2].header.sh_size as usize )
            );

            assert_eq!(f.sections[10].header.get_type(), section::Type::Rela);
            assert!(matches!(
                f.sections[10].contents,
                Contents64::RelaSymbols(_)
            ));
            assert_eq!(f.sections[26].header.get_type(), section::Type::SymTab);
            assert!(matches!(
                &f.sections[26].contents,
                Contents64::Symbols(x) if x.len() == 62
            ));
            assert!(matches!(
                &f.sections[26].contents,
                Contents64::Symbols(x) if x[26].symbol_name == "crtstuff.c"
            ));
            assert!(matches!(
                &f.sections[26].contents,
                Contents64::Symbols(x) if x[45].symbol_name == "_ITM_deregisterTMCloneTable"
            ));

            assert_eq!(f.sections[21].header.get_type(), section::Type::Dynamic);
            assert!(matches!(
                &f.sections[21].contents,
                Contents64::Dynamics(x) if x[1].get_type() == dynamic::EntryType::Init
            ));
            assert!(matches!(
                &f.sections[21].contents,
                Contents64::Dynamics(x) if x[2].get_type() == dynamic::EntryType::Fini
            ));

            assert_eq!(f.segments[0].header.get_type(), segment::Type::Phdr);
            assert!(f.segments[0].header.get_flags().contains(&segment::Flag::R));
            assert_eq!(f.segments[0].header.p_align, 8);

            assert_eq!(f.segments[1].header.get_type(), segment::Type::Interp);
            assert!(f.segments[1].header.get_flags().contains(&segment::Flag::R));
            assert_eq!(f.segments[1].header.p_align, 1);
        }
    }

    #[test]
    fn read_elf32_test() {
        let f_result = parse_elf("src/parser/testdata/32bit");
        assert!(f_result.is_ok());

        let f = f_result.unwrap();
        assert!(matches!(f, file::ELF::ELF32(_)));

        if let file::ELF::ELF32(f) = f {
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
}
