//! ELF symbol visibility.

/// Symbol Visibilities.
pub enum Visibility {
    /// Default symbol visibility rules.
    Default,

    /// Processor specific hidden class.
    Internal,

    /// Symbol is unavailable in other modules.
    Hidden,

    /// Not preemptive, not exported.
    Protected,

    /// User-defined value.
    Any(u8),
}

impl Visibility {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Default => 0,
            Self::Internal => 1,
            Self::Hidden => 2,
            Self::Protected => 3,
            Self::Any(c) => *c,
        }
    }
}

impl From<u8> for Visibility {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::Default,
            1 => Self::Internal,
            2 => Self::Hidden,
            3 => Self::Protected,
            _ => Self::Any(byte),
        }
    }
}
