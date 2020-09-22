pub enum ELFDATA {
    // invalid data encoding
    DATANONE,
    // 2's complement little endian
    DATA2LSB,
    // 2's complement big endian
    DATA2MSB,
    DATA2NUM,

    // for architecture-specific-value
    ANY(u8),
}

impl ELFDATA {
    pub const INDEX: usize = 5;
    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::DATANONE => 0,
            Self::DATA2LSB => 1,
            Self::DATA2MSB => 2,
            Self::DATA2NUM => 3,
            Self::ANY(c) => *c,
        }
    }
}

impl From<u8> for ELFDATA {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::DATANONE,
            1 => Self::DATA2LSB,
            2 => Self::DATA2MSB,
            3 => Self::DATA2NUM,
            _ => Self::ANY(byte),
        }
    }
}
