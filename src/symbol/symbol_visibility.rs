/// Symbol Visibilities.
pub enum VISIBILITY {
    /// Default symbol visibility rules.
    DEFAULT,

    /// Processor specific hidden class.
    INTERNAL,

    /// Symbol is unavailable in other modules.
    HIDDEN,

    /// Not preemptive, not exported.
    PROTECTED,

    /// User-defined value.
    ANY(u8),
}

impl VISIBILITY {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::DEFAULT => 0,
            Self::INTERNAL => 1,
            Self::HIDDEN => 2,
            Self::PROTECTED => 3,
            Self::ANY(c) => *c,
        }
    }
}

impl From<u8> for VISIBILITY {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::DEFAULT,
            1 => Self::INTERNAL,
            2 => Self::HIDDEN,
            3 => Self::PROTECTED,
            _ => Self::ANY(byte),
        }
    }
}
