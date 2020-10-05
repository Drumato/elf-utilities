pub enum Version {
    // value must be 1
    Current,

    // for architecture-specific-value
    Any(u8),
    Any32(u32),
}

impl Version {
    pub const INDEX: usize = 6;

    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::Current => 1,
            Self::Any(c) => *c,
            _ => unreachable!(),
        }
    }
    pub fn to_object_version(&self) -> u32 {
        match self {
            Self::Current => 1,
            Self::Any32(c) => *c,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Version {
    fn from(byte: u8) -> Self {
        match byte {
            1 => Self::Current,
            _ => Self::Any(byte),
        }
    }
}
impl From<u32> for Version {
    fn from(byte: u32) -> Self {
        match byte {
            1 => Self::Current,
            _ => Self::Any32(byte),
        }
    }
}
