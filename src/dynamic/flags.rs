#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Flag {
    /// Object may use this
    Origin,
    /// Symbol resolutions starts here
    Symbolic,
    /// Object contains text relocations
    TextRel,
    /// No lazy binding for this object.
    BindNow,
    /// Module uses the static TLS model.
    StaticTLS,
    /// Set `RTLD_NOW` for this object
    Now1,
    /// Set `RTLD_GLOBAL` for this object
    Global1,
    /// Set `RTLD_GROUP` for this object
    Group1,
    /// Set `RTLD_NODELETE` for this object
    NoDelete1,
    /// Trigger filtee loading at runtime
    LoadFilter1,
    /// Set `RTLD_INITFIRST` for this object
    InitFirst1,
    /// Set `RTLD_NOOPEN` for this object
    NoOpen1,
    /// $ORIGIN must be handled
    Origin1,
    /// Direct binding enabled
    Direct1,
    Trans1,
    /// Object is used to interpose
    Interpose1,
    /// Ignore default lib search path
    NoDefLib1,
    /// Object can't be dldump'ed
    NoDump1,
    /// Configuration alternative created
    ConfAlt1,
    /// Filtee terminates filters search
    EndFiltee1,
    /// Disp reloc applied at build time
    DispRelDNE1,
    /// Disp reloc applied at run-time
    DispRelPND1,
    /// Object has no-direct binding
    NoDirect1,
    IGNMulDef1,
    NokSyms1,
    NoHdr1,
    /// Object is modified after built
    Edited1,
    NoReloc1,
    /// Object has individual interposers
    SymInterpose1,
    /// Global auditing required
    GlobalAudit1,
    /// Singleton symbols are used
    Singleton1,
    Stub1,
    PIE1,
    KMod1,
    WeakFilter1,
    NoCommon1,
    /// User defined value
    Any(u64),
}

impl Flag {
    /// For DT_FLAGS
    pub fn from_def(value: u64) -> Flag {
        match value {
            0x1 => Flag::Origin,
            0x2 => Flag::Symbolic,
            0x4 => Flag::TextRel,
            0x8 => Flag::BindNow,
            0x10 => Flag::StaticTLS,
            _ => Flag::Any(value),
        }
    }
    /// For DT_FLAGS_1
    pub fn from_1(value: u64) -> Flag {
        match value {
            0x1 => Flag::Now1,
            0x2 => Flag::Global1,
            0x4 => Flag::Group1,
            0x8 => Flag::NoDelete1,
            0x10 => Flag::LoadFilter1,
            0x20 => Flag::InitFirst1,
            0x40 => Flag::NoOpen1,
            0x80 => Flag::Origin1,
            0x100 => Flag::Direct1,
            0x200 => Flag::Trans1,
            0x400 => Flag::Interpose1,
            0x800 => Flag::NoDefLib1,
            0x1000 => Flag::NoDump1,
            0x2000 => Flag::ConfAlt1,
            0x4000 => Flag::EndFiltee1,
            0x8000 => Flag::DispRelDNE1,
            0x10000 => Flag::DispRelPND1,
            0x20000 => Flag::NoDirect1,
            0x40000 => Flag::IGNMulDef1,
            0x80000 => Flag::NokSyms1,
            0x100000 => Flag::NoHdr1,
            0x200000 => Flag::Edited1,
            0x400000 => Flag::NoReloc1,
            0x800000 => Flag::SymInterpose1,
            0x1000000 => Flag::GlobalAudit1,
            0x2000000 => Flag::Singleton1,
            0x4000000 => Flag::Stub1,
            0x8000000 => Flag::PIE1,
            0x10000000 => Flag::KMod1,
            0x20000000 => Flag::WeakFilter1,
            0x40000000 => Flag::NoCommon1,
            _ => Flag::Any(value),
        }
    }
}
