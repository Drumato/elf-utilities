#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Bind {
    /// Local Symbol
    Local,
    /// Globbal Symbol
    Global,
    /// Weak Symbol
    Weak,
    /// Number of defined types
    Num,
    /// Start of OS-specific
    LoOS,
    /// Unique Symbol
    GNUUnique,
    /// End of OS-specific
    HiOS,
    /// Start of processor-specific
    LoProc,
    /// end of processor-specific
    HiProc,
    /// User defined value
    Any(u8),
}

impl Bind {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Local => 0,
            Self::Global => 1,
            Self::Weak => 2,
            Self::Num => 3,
            Self::LoOS | Self::GNUUnique => 10,
            Self::HiOS => 12,
            Self::LoProc => 13,
            Self::HiProc => 15,
            Self::Any(b) => *b,
        }
    }
}

impl From<u8> for Bind {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::Local,
            1 => Self::Global,
            2 => Self::Weak,
            3 => Self::Num,
            10 => Self::LoProc,
            12 => Self::HiOS,
            13 => Self::LoProc,
            15 => Self::HiProc,
            _ => Self::Any(byte),
        }
    }
}
