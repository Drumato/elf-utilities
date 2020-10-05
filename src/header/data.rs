pub enum Data {
    // invalid data encoding
    None,
    // 2's complement little endian
    LSB2,
    // 2's complement big endian
    MSB2,
    Num,

    // for architecture-specific-value
    Any(u8),
}

impl Data {
    pub const INDEX: usize = 5;
    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::LSB2 => 1,
            Self::MSB2 => 2,
            Self::Num => 3,
            Self::Any(c) => *c,
        }
    }
}

impl From<u8> for Data {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::None,
            1 => Self::LSB2,
            2 => Self::MSB2,
            3 => Self::Num,
            _ => Self::Any(byte),
        }
    }
}
