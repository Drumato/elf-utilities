#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum EntryType {
    /// Marks end of dynamic section
    Null,
    /// Name of needed library
    Needed,
    /// Size in bytes of PLT relocs
    PLTRelSz,
    /// Processor defined value
    PLTGOT,
    /// Address of symbol hash table
    Hash,
    /// Address of string table
    StrTab,
    /// Address of symbol table
    SymTab,
    /// Address of Rela relocs
    Rela,
    /// Total size of Rela relocs
    RelaSz,
    /// Size of one Rela reloc
    RelaEnt,
    /// Size of string table
    StrSz,
    /// Size of one symbol table entry
    SymEnt,
    /// Address of init function
    Init,
    /// Address of termination function
    Fini,
    /// Name of shared object
    SOName,
    /// Library search path (deprecated)
    RPath,
    /// Start Symbol search here
    Symbolic,
    /// Address of Rel relocs
    Rel,
    /// Total size of Rel relocs
    RelSz,
    /// Size of one Rel reloc
    RelEnt,
    /// Type of reloc in PLT
    PLTRel,
    /// For debugging
    Debug,
    /// Reloc might modify .text
    TextRel,
    /// Address of PLT relocs
    JmpRel,
    /// Process relocations of object
    BindNow,
    /// Array with addresses of init fct
    InitArray,
    /// Array with addresses of fini fct
    FiniArray,
    /// Size in bytes of `InitArray`
    InitArraySz,
    /// Size in bytes of `FiniArray`
    FiniArraySz,
    /// Library search path
    RunPath,
    /// Flags for the object being loaded
    Flags,
    /// Start of encode range
    Encoding,
    /// Array with addresses of preinit fct
    PreInitArray,
    /// Size in bytes of `PreInitArray`
    PreInitArraySz,
    /// Address of `SYMTAB_SHNDX` section
    SymTabShNdx,
    /// Number used
    Num,
    /// Start of OS specific
    LoOS,
    /// End of OS specific
    HiOS,
    /// Start of processor specific
    LoProc,
    /// End of processor specific
    HiProc,
    /// Address of table with needed versions
    VerNeed,
    /// Number of needed versions
    VerNeedNum,
    /// GNU-style hash table
    GNUHash,
    /// State Flags, See `Flags::*1`.
    Flags1,
    /// The versioning entry types.
    VerSym,
    RelCount,
    RelaCount,
    /// User defined value
    Any(i64),
}

impl From<i64> for EntryType {
    fn from(v: i64) -> Self {
        match v {
            0 => EntryType::Null,
            1 => EntryType::Needed,
            2 => EntryType::PLTRelSz,
            3 => EntryType::PLTGOT,
            4 => EntryType::Hash,
            5 => EntryType::StrTab,
            6 => EntryType::SymTab,
            7 => EntryType::Rela,
            8 => EntryType::RelaSz,
            9 => EntryType::RelaEnt,
            10 => EntryType::StrSz,
            11 => EntryType::SymEnt,
            12 => EntryType::Init,
            13 => EntryType::Fini,
            14 => EntryType::SOName,
            15 => EntryType::RPath,
            16 => EntryType::Symbolic,
            17 => EntryType::Rel,
            18 => EntryType::RelSz,
            19 => EntryType::RelEnt,
            20 => EntryType::PLTRel,
            21 => EntryType::Debug,
            22 => EntryType::TextRel,
            23 => EntryType::JmpRel,
            24 => EntryType::BindNow,
            25 => EntryType::InitArray,
            26 => EntryType::FiniArray,
            27 => EntryType::InitArraySz,
            28 => EntryType::FiniArraySz,
            29 => EntryType::RunPath,
            30 => EntryType::Flags,
            32 => EntryType::PreInitArray,
            33 => EntryType::PreInitArraySz,
            34 => EntryType::SymTabShNdx,
            35 => EntryType::Num,
            0x6000000d => EntryType::LoOS,
            0x6ffff000 => EntryType::HiOS,
            0x70000000 => EntryType::LoProc,
            0x7fffffff => EntryType::HiProc,
            0x6ffffef5 => EntryType::GNUHash,
            0x6ffffff0 => EntryType::VerSym,
            0x6ffffff9 => EntryType::RelaCount,
            0x6ffffffa => EntryType::RelCount,
            0x6ffffffb => EntryType::Flags1,
            0x6ffffffe => EntryType::VerNeed,
            0x6fffffff => EntryType::VerNeedNum,
            _ => EntryType::Any(v),
        }
    }
}
