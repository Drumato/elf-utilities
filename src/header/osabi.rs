pub enum ELFOSABI {
    // UNIX System V ABI
    NONE,
    SYSV,
    // HP-UX
    HPUX,
    // NetBSD
    NETBSD,
    // Object uses GNU ELF extensions
    GNU,
    LINUX,

    //  Sun Solaris
    SOLARIS,
    //  IBM AIX
    AIX,
    //  SGI Irix
    IRIX,
    //  FreeBSD
    FREEBSD,
    //  Compaq TRU64 UNIX
    TRU64,
    //  Novell Modesto
    MODESTO,
    //  OpenBSD
    OPENBSD,
    //  ARM EABI
    ARMAEABI,
    //  ARM
    ARM,
    // Standalone (embedded) application
    STANDALONE,
    // for architecture-specific-value
    ANY(u8),
}

impl ELFOSABI {
    pub const INDEX: usize = 7;

    pub fn to_identifier(&self) -> u8 {
        match self {
            Self::NONE | Self::SYSV => 0,
            Self::HPUX => 1,
            Self::NETBSD => 2,
            Self::GNU | Self::LINUX => 3,

            Self::SOLARIS => 6,
            Self::AIX => 7,
            Self::IRIX => 8,
            Self::FREEBSD => 9,
            Self::TRU64 => 10,
            Self::MODESTO => 11,
            Self::OPENBSD => 12,
            Self::ARMAEABI => 64,
            Self::ARM => 97,
            Self::STANDALONE => 255,
            Self::ANY(c) => *c,
        }
    }
}
