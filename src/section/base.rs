use crate::section::Type;

pub trait Section {
    type Header;
    type Symbol;
    type Dyn;
    type Rela;

    fn new(header: Self::Header) -> Self;

    fn header_size() -> usize;

    fn size_zero(&self) -> bool;

    fn offset(&self) -> usize;
    fn name_idx(&self) -> usize;

    fn section_type(&self) -> Type;

    fn entry_size(&self) -> usize;
    fn section_size(&self) -> usize;
    fn clone_contents(&self) -> Vec<u8>;

    fn parse_bytes_as_symbols(&self, related_string_table: &Self) -> Vec<Self::Symbol>;
    fn parse_bytes_as_dynamics(&self) -> Vec<Self::Dyn>;
    fn parse_bytes_as_relas(&self) -> Vec<Self::Rela>;

    fn header_deserialize(
        buf: &[u8],
        header_start: usize,
    ) -> Result<Self::Header, Box<dyn std::error::Error>>;

    fn update_contents(&mut self, contents: Vec<u8>);

    fn update_name(&mut self, name: String);
}
