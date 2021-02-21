use super::{ELF32, ELF64};
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

pub enum ELF {
    ELF32(ELF32),
    ELF64(ELF64),
}

pub struct ELFDumper {
    pub file: ELF,
}

impl ELF {
    pub(crate) fn as_64bit(self) -> ELF64 {
        match self {
            ELF::ELF64(e) => e,
            _ => unreachable!(),
        }
    }
    pub(crate) fn as_32bit(self) -> ELF32 {
        match self {
            ELF::ELF32(e) => e,
            _ => unreachable!(),
        }
    }
    pub(crate) fn to_le_bytes(&self) -> Vec<u8> {
        match self {
            ELF::ELF64(e) => e.to_le_bytes(),
            ELF::ELF32(e) => e.to_le_bytes(),
        }
    }
}

impl ELFDumper {
    pub fn new(f: ELF) -> Self {
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
