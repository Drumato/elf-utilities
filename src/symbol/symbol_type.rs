#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TYPE {
    /// Unspecified
    NOTYPE,
/// Data object
OBJECT,
/// Code object
FUNC,
/// Section
SECTION,
    /// Symbol's name is file name
    FILE,
    /// Common data object
    COMMON,
    /// Thread-Local data object
    TLS,
    /// Number of defined types
    NUM,
    /// Start of OS-specific
    LOOS,
    /// Indirect code object
    GNUIFUNC,
    /// End of OS-specific
    HIOS,
    /// Start of processor-specific
    LOPROC,
    /// end of processor-specific
    HIPROC,
    /// User defined value
    ANY(u8),
}


impl TYPE {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::NOTYPE => 0,
            Self::OBJECT => 1,
            Self::FUNC => 2,
            Self::SECTION => 3,
            Self::FILE => 4,
            Self::COMMON => 5,
            Self::TLS => 6,
            Self::NUM => 7,
            Self::LOOS|Self::GNUIFUNC => 10,
            Self::HIOS => 12,
            Self::LOPROC => 13,
            Self::HIPROC => 15,
            Self::ANY(b) => *b,
        }
    }
}

impl From<u8> for TYPE {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::NOTYPE,
            1 => Self::OBJECT,
            2 => Self::FUNC,
            3 => Self::SECTION,
            4 => Self::FILE,
            5 => Self::COMMON,
            6 => Self::TLS,
            7 => Self::NUM,
            10 => Self::LOPROC,
            12 => Self::HIOS,
            13 => Self::LOPROC,
            15 => Self::HIPROC,
            _ => Self::ANY(byte),
        }
    }
}