use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Dyn64 {
    /// dynamic entry type
    pub d_tag: Elf64Sxword,
    /// value
    pub d_un: Elf64Xword,
}

impl Dyn64 {
    pub fn get_type(&self) -> dynamic::EntryType {
        dynamic::EntryType::from(self.d_tag)
    }
}
