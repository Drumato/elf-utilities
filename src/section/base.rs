use crate::section;

pub(crate) trait Section {
    type Header;
    type Contents: Contents;

    fn new(header: Self::Header) -> Self;
    fn clone_contents(&self) -> Self::Contents;
    fn clone_raw_binary(&self) -> Vec<u8>;
    fn update_contents_from_raw_bytes(&mut self, bytes: Vec<u8>);
    fn symbol_number(&self) -> usize;
    fn update_symbol_name(&mut self, sym_idx: usize, name_bytes: &[u8]);

    fn header_size() -> usize;

    fn size_zero(&self) -> bool;

    fn offset(&self) -> usize;
    fn name_idx(&self) -> usize;
    fn section_link(&self) -> usize;

    fn section_type(&self) -> section::Type;

    fn entry_size(&self) -> usize;
    fn section_size(&self) -> usize;
    fn header_deserialize(
        buf: &[u8],
        header_start: usize,
    ) -> Result<Self::Header, Box<dyn std::error::Error>>;

    fn update_name(&mut self, name: String);
}

pub(crate) trait Contents {
    type Symbol;
    type Dyn;
    type Rela;

    fn clone_raw_binary(&self) -> Vec<u8>;
}
