#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// Unspecified
    NoType,
    /// Data object
    Object,
    /// Code object
    Func,
    /// Section
    Section,
    /// Symbol's name is file name
    File,
    /// Common data object
    Common,
    /// Thread-Local data object
    TLS,
    /// Number of defined types
    Num,
    /// Start of OS-specific
    LoOS,
    /// Indirect code object
    GNUIFunc,
    /// End of OS-specific
    HiOS,
    /// Start of processor-specific
    LoProc,
    /// end of processor-specific
    HiProc,
    /// User defined value
    Any(u8),
}

impl Type {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::NoType => 0,
            Self::Object => 1,
            Self::Func => 2,
            Self::Section => 3,
            Self::File => 4,
            Self::Common => 5,
            Self::TLS => 6,
            Self::Num => 7,
            Self::LoOS | Self::GNUIFunc => 10,
            Self::HiOS => 12,
            Self::LoProc => 13,
            Self::HiProc => 15,
            Self::Any(b) => *b,
        }
    }
}

impl From<u8> for Type {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::NoType,
            1 => Self::Object,
            2 => Self::Func,
            3 => Self::Section,
            4 => Self::File,
            5 => Self::Common,
            6 => Self::TLS,
            7 => Self::Num,
            10 => Self::LoProc,
            12 => Self::HiOS,
            13 => Self::LoProc,
            15 => Self::HiProc,
            _ => Self::Any(byte),
        }
    }
}
