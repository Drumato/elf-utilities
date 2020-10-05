//! Type definitions for 64-bit ELF binaries.

use crate::*;
use serde::{Deserialize, Serialize};

/* definitions for st_info(bind) */
/// Local Symbol
pub const STB_LOCAL: u8 = 0;
/// Global Symbol
pub const STB_GLOBAL: u8 = 1;

/// Symbol64 is a entry of symbol table section.
///
/// Symbol64 はシンボルテーブルセクションのエントリである．
/// ELF64で用いることを想定している．
///
///
/// Defaultトレイトを実装しているので，
/// `Default::default()` を呼び出すことで簡単にNULLシンボルを作成できる．
///
/// # Examples
///
/// ```
/// use elf_utilities::symbol::Symbol64;
/// let null_sym : Symbol64 = Default::default();
///
/// // Symbol64::new_null_symbol() のエイリアスでも作成可能．
/// let null_sym2 : Symbol64 = Symbol64::new_null_symbol();
///
/// assert_eq!(null_sym, null_sym2);
///
/// ```
///
/// ELFファイルを生成する用途でこのライブラリを使用できるように，
/// バイト列への変換もサポートしている．
///
/// # Examples
///
/// ```
/// use elf_utilities::symbol::Symbol64;
/// let null_sym : Symbol64 = Default::default();
///
/// // to_le_bytes() を利用してバイト列に変換できる．
/// let sym_bytes = null_sym.to_le_bytes();
/// assert_eq!(Symbol64::size(),sym_bytes.len() as elf_utilities::Elf64Xword)
/// ```
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Symbol64 {
    /// Symbol name index.
    pub st_name: Elf64Word,

    /// Information that includes [symbol binds and symbol types](https://docs.rs/elf-utilities/latest/elf_utilities/symbol/elf64/index.html#constants).
    pub st_info: u8,

    /// Symbol's visibility.
    /// See [symbol::VISIBILITY](https://docs.rs/elf-utilities/latest/elf_utilities/symbol/symbol_visibility/enum.VISIBILITY.html).
    pub st_other: u8,

    /// A section table index that includes the symbol.
    pub st_shndx: Elf64Section,

    /// Symbol's value.
    pub st_value: Elf64Addr,

    /// Symbol's size.
    pub st_size: Elf64Xword,

    /// option member for utilities.
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub symbol_name: Option<String>,
}

impl Default for Symbol64 {
    fn default() -> Self {
        Self {
            st_name: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
            symbol_name: None,
        }
    }
}

#[allow(dead_code)]
impl Symbol64 {
    pub fn new_null_symbol() -> Self {
        Default::default()
    }
    /// size() provides Symbol64's size used by Shdr64.sh_entsize or else.
    pub fn size() -> Elf64Xword {
        24
    }

    /// for utilities
    pub fn compare_by<P>(&self, predicate: P) -> bool
    where
        P: Fn(&Self) -> bool,
    {
        predicate(self)
    }

    pub fn get_type(&self) -> symbol::Type {
        symbol::Type::from(self.st_info & 0x0f)
    }

    pub fn get_bind(&self) -> symbol::Bind {
        symbol::Bind::from(self.st_info >> 4)
    }
    pub fn get_visibility(&self) -> symbol::Visibility {
        symbol::Visibility::from(self.st_other & 0x03)
    }

    /// Set symbol's information to Symbol64
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol;
    /// let mut null_sym = symbol::Symbol64::new_null_symbol();
    ///
    /// null_sym.set_info(symbol::Type::Func, symbol::Bind::Global);
    ///
    /// assert_eq!((1 << 4) | 2, null_sym.st_info);
    /// ```
    pub fn set_info(&mut self, sym_type: symbol::Type, bind: symbol::Bind) {
        self.st_info = bind.to_byte() << 4 | sym_type.to_byte();
    }

    /// Create Vec<u8> from Symbol64's each fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol::Symbol64;
    /// let null_sym : Symbol64 = Default::default();
    ///
    /// assert_eq!([0].repeat(Symbol64::size() as usize), null_sym.to_le_bytes());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(buf: &[u8], start: usize) -> Result<Self, Box<dyn std::error::Error>> {
        // bincode::ErrorKindをトレイトオブジェクトとするため,この冗長な書き方が必要
        match bincode::deserialize(&buf[start..]) {
            Ok(header) => Ok(header),
            Err(e) => Err(e),
        }
    }
}
