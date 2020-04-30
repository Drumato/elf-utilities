#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ELFTYPE {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    NUM,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC,
    ANY(u16),
}

impl ELFTYPE {
    pub fn to_bytes(&self) -> u16 {
        match self {
            Self::NONE => 0,
            Self::REL => 1,
            Self::EXEC => 2,
            Self::DYN => 3,
            Self::CORE => 4,
            Self::NUM => 5,
            Self::LOOS => 0xfe00,
            Self::HIOS => 0xfeff,
            Self::LOPROC => 0xff00,
            Self::HIPROC => 0xffff,
            Self::ANY(c) => *c,
        }
    }
}

impl From<u16> for ELFTYPE {
    fn from(bytes: u16) -> Self {
        match bytes {
            0 => Self::NONE,
            1 => Self::REL,
            2 => Self::EXEC,
            3 => Self::DYN,
            4 => Self::CORE,
            5 => Self::NUM,
            0xfe00 => Self::LOOS,
            0xfeff => Self::HIOS,
            0xff00 => Self::LOPROC,
            0xffff => Self::HIPROC,
            _ => Self::ANY(bytes),
        }
    }
}
