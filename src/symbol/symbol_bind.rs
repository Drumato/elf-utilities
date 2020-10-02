#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BIND {
    /// Local Symbol
    LOCAL,
    /// Globbal Symbol
    GLOBAL,
    /// Weak Symbol
    WEAK,
    /// Number of defined types
    NUM,
    /// Start of OS-specific
    LOOS,
    /// Unique Symbol
    GNUUNIQUE,
    /// End of OS-specific
    HIOS,
    /// Start of processor-specific
    LOPROC,
    /// end of processor-specific
    HIPROC,
    /// User defined value
    ANY(u8),
}


impl BIND {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::LOCAL => 0,
            Self::GLOBAL => 1,
            Self::WEAK => 2,
            Self::NUM => 3,
            Self::LOOS|Self::GNUUNIQUE => 10,
            Self::HIOS => 12,
            Self::LOPROC => 13,
            Self::HIPROC => 15,
            Self::ANY(b) => *b,
        }
    }
}

impl From<u8> for BIND {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::LOCAL,
            1 => Self::GLOBAL,
            2 => Self::WEAK,
            3 => Self::NUM,
            10 => Self::LOPROC,
            12 => Self::HIOS,
            13 => Self::LOPROC,
            15 => Self::HIPROC,
            _ => Self::ANY(byte),
        }
    }
}