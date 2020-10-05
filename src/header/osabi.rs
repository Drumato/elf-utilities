pub enum OSABI {
    // UNIX System V ABI
    None,
    SysV,
    // HP-UX
    HPUX,
    // NetBSD
    NetBSD,
    // Object uses GNU ELF extensions
    GNU,
    Linux,

    //  Sun Solaris
    Solaris,
    //  IBM AIX
    AIX,
    //  SGI Irix
    Irix,
    //  FreeBSD
    FreeBSD,
    //  Compaq TRU64 UNIX
    TRU64,
    //  Novell Modesto
    Modesto,
    //  OpenBSD
    OPENBSD,
    //  ARM EABI
    ArmAEABI,
    //  ARM
    Arm,
    // Standalone (embedded) application
    Standalone,
    // for architecture-specific-value
    Any(u8),
}

impl OSABI {
    pub const INDEX: usize = 7;

    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::None | Self::SysV => 0,
            Self::HPUX => 1,
            Self::NetBSD => 2,
            Self::GNU | Self::Linux => 3,

            Self::Solaris => 6,
            Self::AIX => 7,
            Self::Irix => 8,
            Self::FreeBSD => 9,
            Self::TRU64 => 10,
            Self::Modesto => 11,
            Self::OPENBSD => 12,
            Self::ArmAEABI => 64,
            Self::Arm => 97,
            Self::Standalone => 255,
            Self::Any(c) => *c,
        }
    }
}

impl From<u8> for OSABI {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::SysV,
            1 => Self::HPUX,
            2 => Self::NetBSD,
            3 => Self::GNU,
            6 => Self::Solaris,
            7 => Self::AIX,
            8 => Self::Irix,
            9 => Self::FreeBSD,
            10 => Self::TRU64,
            11 => Self::Modesto,
            12 => Self::OPENBSD,
            64 => Self::ArmAEABI,
            97 => Self::Arm,
            255 => Self::Standalone,
            _ => Self::Any(byte),
        }
    }
}
