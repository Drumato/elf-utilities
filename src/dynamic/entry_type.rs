#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum EntryType {
    /// Marks end of dynamic section
    Null,
    /// Name of needed library
    Needed,
    /// Size in bytes of PLT relocs
    PLTRelSZ,
    /// User defined value
    Any(i64),
}

impl From<i64> for EntryType {
    fn from(v: i64) -> Self {
        match v {
            0 => EntryType::Null,
            1 => EntryType::Needed,
            2 => EntryType::PLTRelSZ,
            _ => EntryType::Any(v),
        }
    }
}
