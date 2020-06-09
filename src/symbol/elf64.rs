//! Type definitions for 64-bit ELF binaries.

use crate::*;

/* definitions for st_info(bind) */
/// Local Symbol
pub const STB_LOCAL: u8 = 0;
/// Global Symbol
pub const STB_GLOBAL: u8 = 1;

/* definitions for st_info(type) */
/// Code object
pub const STT_FUNC: u8 = 2;
/// Section
pub const STT_SECTION: u8 = 3;

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
/// assert_eq!(sym_bytes.len() as elf_utilities::Elf64Xword, Symbol64::size())
/// ```
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct Symbol64 {
    /// Symbol name index.
    st_name: Elf64Word,

    /// Information that includes [symbol binds and symbol types](https://docs.rs/elf-utilities/latest/elf_utilities/symbol/elf64/index.html#constants).
    st_info: u8,

    /// Symbol's visibility.
    /// See [symbol::VISIBILITY](https://docs.rs/elf-utilities/latest/elf_utilities/symbol/symbol_visibility/enum.VISIBILITY.html).
    st_other: u8,

    /// A section table index that includes the symbol.
    st_shndx: Elf64Section,

    /// Symbol's value.
    st_value: Elf64Addr,

    /// Symbol's size.
    st_size: Elf64Xword,

    /// option member for utilities.
    symbol_name: Option<String>,
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

    /// for comparison
    pub fn set_symbol_name(&mut self, name: String) {
        self.symbol_name = Some(name);
    }

    pub fn compare_symbol_name(&self, other: String)  -> bool {
        if self.symbol_name.is_none(){
            return false;
        }

        self.symbol_name.as_ref().unwrap() == &other
    }

    pub fn get_type(&self) -> u8 {
        self.st_info & 0xf
    }

    pub fn get_bind(&self) -> u8 {
        self.st_info >> 4
    }

    /// Set symbol name index to Symbol64
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol::Symbol64;
    /// let mut null_sym : Symbol64 = Default::default();
    /// null_sym.set_name(1);
    ///
    /// assert_eq!(null_sym.get_name(), 1);
    /// ```
    pub fn set_name(&mut self, name: Elf64Word) {
        self.st_name = name;
    }

    /// Set symbol's information to Symbol64
    /// See [symbol::symbol_info](https://docs.rs/elf-utilities/0.1.28/elf_utilities/symbol/util/fn.symbol_info.html) for constructing symbol's information.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol;
    /// let mut null_sym = symbol::Symbol64::new_null_symbol();
    ///
    /// let sym_info = symbol::symbol_info(symbol::STB_GLOBAL, symbol::STT_FUNC);
    /// null_sym.set_info(sym_info);
    ///
    /// assert_eq!((1 << 4) | 2, null_sym.get_info());
    /// ```
    pub fn set_info(&mut self, info: u8) {
        self.st_info = info;
    }

    /// Set symbol's other value to Symbol64
    /// See [symbol::VISIBILITY](https://docs.rs/elf-utilities/latest/elf_utilities/symbol/symbol_visibility/enum.VISIBILITY.html) and [symbol::symbol_visibility](https://docs.rs/elf-utilities/latest/elf_utilities/symbol/util/fn.symbol_visibility.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol;
    /// let mut null_sym = symbol::Symbol64::new_null_symbol();
    ///
    /// let sym_vis = symbol::symbol_visibility(symbol::VISIBILITY::PROTECTED);
    /// null_sym.set_other(sym_vis);
    /// assert_eq!(0x3, null_sym.get_other());
    /// ```
    pub fn set_other(&mut self, other: u8) {
        self.st_other = other;
    }

    /// Set a section table index includes this symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol::Symbol64;
    /// let mut null_sym : Symbol64 = Default::default();
    /// null_sym.set_shndx(1);
    ///
    /// assert_eq!(null_sym.get_shndx(), 1);
    /// ```
    pub fn set_shndx(&mut self, shndx: Elf64Section) {
        self.st_shndx = shndx;
    }

    /// Set symbol value to Symbol64
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol::Symbol64;
    /// let mut null_sym : Symbol64 = Default::default();
    /// null_sym.set_value(1);
    ///
    /// assert_eq!(null_sym.get_value(), 1);
    /// ```
    pub fn set_value(&mut self, value: Elf64Addr) {
        self.st_value = value;
    }

    /// Set symbol binary size to Symbol64
    ///
    /// # Examples
    ///
    /// ```
    /// use elf_utilities::symbol::Symbol64;
    /// let mut null_sym : Symbol64 = Default::default();
    /// null_sym.set_size(0x40);
    ///
    /// assert_eq!(null_sym.get_size(), 64);
    /// ```
    pub fn set_size(&mut self, size: Elf64Xword) {
        self.st_size = size;
    }

    /// Get the symbol name index.
    pub fn get_name(&self) -> Elf64Word {
        self.st_name
    }

    /// Get this symbol's information.
    pub fn get_info(&self) -> u8 {
        self.st_info
    }

    /// Get this symbol's visibility.
    pub fn get_other(&self) -> u8 {
        self.st_other
    }

    /// Get a section table index that related with this symbol.
    pub fn get_shndx(&self) -> Elf64Section {
        self.st_shndx
    }

    /// Get this symbol's value.
    pub fn get_value(&self) -> Elf64Addr {
        self.st_value
    }

    /// Get this symbol's size.
    pub fn get_size(&self) -> Elf64Xword {
        self.st_size
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
        let mut bytes: Vec<u8> = Vec::new();

        for byte in self.st_name.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_other.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_shndx.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_value.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        for byte in self.st_size.to_le_bytes().to_vec() {
            bytes.push(byte);
        }

        bytes
    }
}
