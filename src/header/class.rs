pub enum ELFCLASS {
    // invalid class
    CLASSNone,
    // 32bit objects
    CLASS32,
    // 64bit objects
    CLASS64,
    CLASSNUM,

    // for architecture-specific-value
    ANY(u8),
}

impl ELFCLASS {
    pub const INDEX: usize = 4;
    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::CLASSNone => 0,
            Self::CLASS32 => 1,
            Self::CLASS64 => 2,
            Self::CLASSNUM => 3,
            Self::ANY(b) => *b,
        }
    }
}
