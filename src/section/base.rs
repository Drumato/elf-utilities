use super::{Contents32, Contents64, Section32, Section64, Shdr32, Shdr64, Type};

#[derive(Clone)]
pub(crate) struct Section {
    pub name: String,
    pub header: Shdr,

    pub contents: Contents,
}

#[derive(Clone)]
pub(crate) enum Shdr {
    Shdr64(Shdr64),
    Shdr32(Shdr32),
}

#[derive(Clone)]
pub(crate) enum Contents {
    Contents64(Contents64),
    Contents32(Contents32),
}

impl Section {
    pub fn new(hdr: Shdr) -> Self {
        let is_64bit = matches!(hdr, Shdr::Shdr64(_));
        Self {
            name: Default::default(),
            contents: if is_64bit {
                Contents::Contents64(Contents64::Raw(Default::default()))
            } else {
                Contents::Contents32(Contents32::Raw(Default::default()))
            },
            header: hdr,
        }
    }

    pub fn as_64bit(&self) -> Section64 {
        Section64 {
            name: self.name.clone(),
            contents: self.contents.as_64bit(),
            header: self.header.as_64bit(),
        }
    }
    pub fn as_32bit(&self) -> Section32 {
        Section32 {
            name: self.name.clone(),
            contents: self.contents.as_32bit(),
            header: self.header.as_32bit(),
        }
    }

    pub fn ty(&self) -> Type {
        match self.header {
            Shdr::Shdr32(shdr) => shdr.get_type(),
            Shdr::Shdr64(shdr) => shdr.get_type(),
        }
    }
    pub fn name_idx(&self) -> usize {
        match self.header {
            Shdr::Shdr32(shdr) => shdr.sh_name as usize,
            Shdr::Shdr64(shdr) => shdr.sh_name as usize,
        }
    }
    pub fn offset(&self) -> usize {
        match self.header {
            Shdr::Shdr32(shdr) => shdr.sh_offset as usize,
            Shdr::Shdr64(shdr) => shdr.sh_offset as usize,
        }
    }
    pub fn size(&self) -> usize {
        match self.header {
            Shdr::Shdr32(shdr) => shdr.sh_size as usize,
            Shdr::Shdr64(shdr) => shdr.sh_size as usize,
        }
    }
    pub fn entry_size(&self) -> usize {
        match self.header {
            Shdr::Shdr32(shdr) => shdr.sh_entsize as usize,
            Shdr::Shdr64(shdr) => shdr.sh_entsize as usize,
        }
    }
    pub fn link(&self) -> usize {
        match self.header {
            Shdr::Shdr32(shdr) => shdr.sh_link as usize,
            Shdr::Shdr64(shdr) => shdr.sh_link as usize,
        }
    }
}

impl Contents {
    pub fn as_64bit(&self) -> Contents64 {
        match self {
            Contents::Contents64(contents) => contents.clone(),
            _ => unreachable!(),
        }
    }
    pub fn as_32bit(&self) -> Contents32 {
        match self {
            Contents::Contents32(contents) => contents.clone(),
            _ => unreachable!(),
        }
    }
    pub fn as_raw(&self) -> Vec<u8> {
        match self {
            Contents::Contents32(contents) => match contents {
                Contents32::Raw(v) => v.clone(),
                _ => unreachable!(),
            },
            Contents::Contents64(contents) => match contents {
                Contents64::Raw(v) => v.clone(),
                _ => unreachable!(),
            },
        }
    }
}

impl Shdr {
    pub fn as_64bit(&self) -> Shdr64 {
        match self {
            Self::Shdr64(shdr) => *shdr,
            _ => unreachable!(),
        }
    }
    pub fn as_32bit(&self) -> Shdr32 {
        match self {
            Self::Shdr32(shdr) => *shdr,
            _ => unreachable!(),
        }
    }
}
