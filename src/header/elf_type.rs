#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ELF64TYPE {
    TYPENONE,
    TYPEREL,
    TYPEEXEC,
    TYPEDYN,
    TYPECORE,
    TYPENUM,
    TYPELOOS,
    TYPEHIOS,
    TYPELOPROC,
    TYPEHIPROC,
    ANY(u16),
}

impl ELF64TYPE {
    pub fn to_bytes(&self) -> u16 {
        match self {
            Self::TYPENONE => 0,
            Self::TYPEREL => 1,
            Self::TYPEEXEC => 2,
            Self::TYPEDYN => 3,
            Self::TYPECORE => 4,
            Self::TYPENUM => 5,
            Self::TYPELOOS => 0xfe00,
            Self::TYPEHIOS => 0xfeff,
            Self::TYPELOPROC => 0xff00,
            Self::TYPEHIPROC => 0xffff,
            Self::ANY(c) => *c,
        }
    }
}

impl From<u16> for ELF64TYPE {
    fn from(bytes: u16) -> Self {
        match bytes {
            0 => Self::TYPENONE,
            1 => Self::TYPEREL,
            2 => Self::TYPEEXEC,
            3 => Self::TYPEDYN,
            4 => Self::TYPECORE,
            5 => Self::TYPENUM,
            0xfe00 => Self::TYPELOOS,
            0xfeff => Self::TYPEHIOS,
            0xff00 => Self::TYPELOPROC,
            0xffff => Self::TYPEHIPROC,
            _ => Self::ANY(bytes),
        }
    }
}
