pub enum ELF64VERSION {
    // value must be 1
    VERSIONCURRENT,

    // for architecture-specific-value
    ANY(u8),
}

impl ELF64VERSION {
    pub fn to_identifier(&self) -> u128 {
        let byte = match self {
            Self::VERSIONCURRENT => 1,
            Self::ANY(c) => *c,
        };
        Self::shift_position(byte)
    }
    fn shift_position(byte: u8) -> u128 {
        (byte as u128) << 72
    }
}
