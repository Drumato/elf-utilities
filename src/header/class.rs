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
    pub fn to_identifier(&self) -> u128 {
        let byte = match self {
            Self::CLASSNone => 0,
            Self::CLASS32 => 1,
            Self::CLASS64 => 2,
            Self::CLASSNUM => 3,
            Self::ANY(b) => *b,
        };
        Self::shift_position(byte)
    }
    fn shift_position(byte: u8) -> u128 {
        (byte as u128) << 88
    }
}
