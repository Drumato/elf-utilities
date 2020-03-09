pub enum ELF64DATA {
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

impl ELF64DATA {
    pub fn to_identifier(&self) -> u128 {
        let byte = match self {
            Self::DATANONE => 0,
            Self::DATA2LSB => 1,
            Self::DATA2MSB => 2,
            Self::DATA2NUM => 3,
            Self::ANY(c) => *c,
        };
        Self::shift_position(byte)
    }
    fn shift_position(byte: u8) -> u128 {
        (byte as u128) << 80
    }
}

