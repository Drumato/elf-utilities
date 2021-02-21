use super::{Ehdr32, Ehdr64};

pub(crate) enum Ehdr {
    Ehdr64(Ehdr64),
    Ehdr32(Ehdr32),
}

impl Ehdr {
    pub fn as_64bit(&self) -> Ehdr64 {
        match self {
            Ehdr::Ehdr64(ehdr) => *ehdr,
            _ => unreachable!(),
        }
    }
    pub fn as_32bit(&self) -> Ehdr32 {
        match self {
            Ehdr::Ehdr32(ehdr) => *ehdr,
            _ => unreachable!(),
        }
    }

    /// プログラムヘッダテーブルが存在するかチェック
    pub fn pht_exists(&self) -> bool {
        match self {
            Ehdr::Ehdr64(ehdr) => ehdr.e_phnum != 0,
            Ehdr::Ehdr32(ehdr) => ehdr.e_phnum != 0,
        }
    }

    pub fn shnum(&self) -> usize {
        match self {
            Ehdr::Ehdr64(ehdr) => ehdr.e_shnum as usize,
            Ehdr::Ehdr32(ehdr) => ehdr.e_shnum as usize,
        }
    }

    pub fn phnum(&self) -> usize {
        match self {
            Ehdr::Ehdr64(ehdr) => ehdr.e_phnum as usize,
            Ehdr::Ehdr32(ehdr) => ehdr.e_phnum as usize,
        }
    }

    pub fn sht_start(&self) -> usize {
        match self {
            Ehdr::Ehdr64(ehdr) => ehdr.e_shoff as usize,
            Ehdr::Ehdr32(ehdr) => ehdr.e_shoff as usize,
        }
    }
    pub fn pht_start(&self) -> usize {
        match self {
            Ehdr::Ehdr64(ehdr) => ehdr.e_phoff as usize,
            Ehdr::Ehdr32(ehdr) => ehdr.e_phoff as usize,
        }
    }
    pub fn shstrndx(&self) -> usize {
        match self {
            Ehdr::Ehdr64(ehdr) => ehdr.e_shstrndx as usize,
            Ehdr::Ehdr32(ehdr) => ehdr.e_shstrndx as usize,
        }
    }
}
