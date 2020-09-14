pub enum ELFVERSION {
    // value must be 1
    VERSIONCURRENT,

    // for architecture-specific-value
    ANY(u8),
}

impl ELFVERSION {
    pub const INDEX: usize = 6;

    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::VERSIONCURRENT => 1,
            Self::ANY(c) => *c,
        }
    }
}
