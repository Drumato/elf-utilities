//! Type definitions for 64-bit ELF binaries.

use crate::*;

use crate::segment::*;

pub struct Segment64 {
    pub name: String,
    pub header: Phdr64,
}

impl Segment64 {
    pub fn new(name: String, header: Phdr64) -> Self {
        Self {
            name,
            header,
        }
    }
}

#[repr(C)]
pub struct Phdr64 {
    /// Segment type
    p_type: Elf64Word,

    /// Segment flags
    p_flags: Elf64Word,

    /// Segment file offset
    p_offset: Elf64Off,

    /// Segment virtual address
    p_vaddr: Elf64Addr,

    /// Segment physical address
    p_paddr: Elf64Addr,

    /// Segment size in file
    p_filesz: Elf64Xword,

    /// Segment size in memory
    p_memsz: Elf64Xword,

    /// Segment alignment
    p_align: Elf64Xword,
}

impl Default for Phdr64 {
    fn default() -> Self {
        Self {
            p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }
}

impl Phdr64 {
    pub fn size() -> Elf64Half {
        0x38
    }

    // getter
    pub fn get_flags(&self) -> Elf64Word {
        self.p_flags
    }
    pub fn get_type(&self) -> Elf64Word {
        self.p_type
    }

    // setter

    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment;
    ///
    /// let mut phdr : segment::Phdr64 = Default::default();
    /// phdr.set_type(segment::TYPE::LOAD);
    ///
    /// assert_eq!(phdr.get_type(), 0x01);
    /// ```
    pub fn set_type(&mut self, ptype: segment_type::TYPE) {
        self.p_type = ptype.to_bytes();
    }


    /// # Examples
    ///
    /// ```
    /// use elf_utilities::segment;
    ///
    ///
    /// let mut phdr : segment::Phdr64 = Default::default();
    /// phdr.set_flags(segment::PF_W | segment::PF_R);
    ///
    /// assert_eq!(phdr.get_flags(), 0b110);
    /// ```
    pub fn set_flags(&mut self, flags: Elf64Word) {
        self.p_flags = flags;
    }

    pub fn set_offset(&mut self, offset: Elf64Off) {
        self.p_offset = offset;
    }

    pub fn set_vaddr(&mut self, vaddr: Elf64Addr) {
        self.p_vaddr = vaddr;
    }

    pub fn set_paddr(&mut self, paddr: Elf64Addr) {
        self.p_paddr = paddr;
    }

    pub fn set_filesz(&mut self, filesz: Elf64Xword) {
        self.p_filesz = filesz;
    }
    pub fn set_memsz(&mut self, memsz: Elf64Xword) {
        self.p_memsz = memsz;
    }
    pub fn set_align(&mut self, align: Elf64Xword) {
        self.p_align = align;
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for byte in self.p_type.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_flags.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_offset.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_vaddr.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_paddr.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_filesz.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_memsz.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.p_align.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        bytes
    }
}