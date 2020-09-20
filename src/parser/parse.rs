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
}

/// read 64bit ELF and construct `file::ELF64`
pub fn read_elf64(file_path: &str) -> Result<file::ELF64, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf);

    let _ = check_elf_magic(file_path, &buf[..4])?;

    let elf_header = parse_elf64_header(&buf[..header::Ehdr64::size() as usize])?;
    let phdr_table_exists = elf_header.get_phnum() != 0;

    let mut elf_file = file::ELF64::new(elf_header);

    let sections = read_elf64_sections(elf_file.get_ehdr(), &buf)?;
    elf_file.set_sections(sections);

    if phdr_table_exists {
        // elf_file.set_segments();
    }

    Ok(elf_file)
}

fn read_elf64_sections(
    elf_header: &header::Ehdr64,
    buf: &[u8],
) -> Result<Vec<section::Section64>, Box<dyn std::error::Error>> {
    let mut sections: Vec<section::Section64> = Vec::new();

    for sct_idx in 0..elf_header.get_shnum() {
        let shdr_result =
            bincode::deserialize(&buf[elf_header.get_shoff() as usize + section::Shdr64::size() as usize * sct_idx as usize..]);
        if let Err(e) = shdr_result {
            return Err(Box::new(ReadELFError::CantParseSectionHeader { k: e }));
        }

        let shdr = shdr_result.unwrap();
        let sct = section::Section64::new(String::new(), shdr);
        // sct.shdrがシンボルテーブルなどの場合,追加の処理が必要となる
        sections.push(sct);
    }

    Ok(sections)
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
        assert_eq!(hdr_result.get_entry(), 0xe160);
        assert_eq!(hdr_result.get_shnum(), 44);
    }

    #[test]
    fn read_elf64_test() {
        let f_result = read_elf64("examples/sample");
        assert!(f_result.is_ok());
        let f = f_result.unwrap();

        assert_eq!(f.get_ehdr().get_entry(), 0x1040);
        assert_eq!(f.get_ehdr().get_shnum(), 29);
        assert_eq!(f.get_ehdr().get_shstrndx(), 28);

        assert_eq!(f.clone_sections().len(), 29);
        // assert_eq!(f.get_ehdr().get_phnum(), 13);
    }
}
