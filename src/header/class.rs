#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    // invalid class
    None,
    // 32bit objects
    Bit32,
    // 64bit objects
    Bit64,
    Num,

    // for architecture-specific-value
    Any(u8),
}

impl Class {
    pub const INDEX: usize = 4;
    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Bit32 => 1,
            Self::Bit64 => 2,
            Self::Num => 3,
            Self::Any(b) => *b,
        }
    }
}

impl From<u8> for Class {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::None,
            1 => Self::Bit32,
            2 => Self::Bit64,
            3 => Self::Num,
            _ => Self::Any(byte),
        }
    }
}
