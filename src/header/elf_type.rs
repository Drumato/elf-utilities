#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// No file type
    None,
    /// Relocatable file
    Rel,
    /// Executable file
    Exec,
    /// Shared object file
    Dyn,
    /// Core file
    Core,
    /// Number of defined types
    Num,
    /// Start of OS specific
    LoOS,
    /// End of OS specific
    HiOS,
    /// Start of processor specific
    LoProc,
    /// End of processor specific
    HiProc,
    /// User defined value
    Any(u16),
}

impl Type {
    pub fn to_bytes(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::Rel => 1,
            Self::Exec => 2,
            Self::Dyn => 3,
            Self::Core => 4,
            Self::Num => 5,
            Self::LoOS => 0xfe00,
            Self::HiOS => 0xfeff,
            Self::LoProc => 0xff00,
            Self::HiProc => 0xffff,
            Self::Any(c) => *c,
        }
    }
}

impl From<u16> for Type {
    fn from(bytes: u16) -> Self {
        match bytes {
            0 => Self::None,
            1 => Self::Rel,
            2 => Self::Exec,
            3 => Self::Dyn,
            4 => Self::Core,
            5 => Self::Num,
            0xfe00 => Self::LoOS,
            0xfeff => Self::HiOS,
            0xff00 => Self::LoProc,
            0xffff => Self::HiProc,
            _ => Self::Any(bytes),
        }
    }
}
