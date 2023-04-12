use crate::file;

use super::ElfParserError;

pub struct ElfParser {
    /// determines whether the parse tries to parse ELF header or not.
    parse_header: bool,
}

impl ElfParser {
    pub fn parse_raw(&self, b: &[u8]) -> Result<file::RawElfFile, ElfParserError> {
        let (b, elf_identification) = self.peek_raw_elf_identification(b)?;

        Ok(file::RawElfFile::Elf64)
    }
}

impl Default for ElfParser {
    fn default() -> Self {
        Self {
            parse_header: false,
        }
    }
}

pub struct ElfParserConfig {
    /// determines whether the parse tries to parse ELF header or not.
    parse_header: bool,
}

impl ElfParserConfig {
    pub fn new() -> Self {
        Self {
            parse_header: false,
        }
    }

    pub fn build(self) -> ElfParser {
        ElfParser {
            parse_header: self.parse_header,
        }
    }

    /// determines whether the parse tries to parse ELF header or not.
    pub fn parse_header(mut self, parse_header: bool) -> Self {
        self.parse_header = parse_header;
        self
    }
}
