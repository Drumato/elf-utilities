use super::{Phdr32, Phdr64, Segment32, Segment64};

pub(crate) struct Segment {
    pub phdr: Phdr,
}

pub(crate) enum Phdr {
    Phdr32(Phdr32),
    Phdr64(Phdr64),
}

impl Segment {
    pub fn as_64bit(&self) -> Segment64 {
        Segment64 {
            header: self.phdr.as_64bit(),
        }
    }
    pub fn as_32bit(&self) -> Segment32 {
        Segment32 {
            header: self.phdr.as_32bit(),
        }
    }
}

impl Phdr {
    pub fn as_64bit(&self) -> Phdr64 {
        match self {
            Self::Phdr64(phdr) => *phdr,
            _ => unreachable!(),
        }
    }

    pub fn as_32bit(&self) -> Phdr32 {
        match self {
            Self::Phdr32(phdr) => *phdr,
            _ => unreachable!(),
        }
    }
}
