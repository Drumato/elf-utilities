pub enum ELFVERSION {
    // value must be 1
    VERSIONCURRENT,

    // for architecture-specific-value
    ANY(u8),
    ANY32(u32),
}

impl ELFVERSION {
    pub const INDEX: usize = 6;

    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::VERSIONCURRENT => 1,
            Self::ANY(c) => *c,
            _ => unreachable!(),
        }
    }
    pub fn to_object_version(&self) -> u32 {
        match self {
            Self::VERSIONCURRENT => 1,
            Self::ANY32(c) => *c,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for ELFVERSION {
    fn from(byte: u8) -> Self {
        match byte {
            1 => Self::VERSIONCURRENT,
            _ => Self::ANY(byte),
        }
    }
}
impl From<u32> for ELFVERSION {
    fn from(byte: u32) -> Self {
        match byte {
            1 => Self::VERSIONCURRENT,
            _ => Self::ANY32(byte),
        }
    }
}
