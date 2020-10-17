use crate::section::Type;

pub trait Section {
    type Header;
    fn new(header: Self::Header) -> Self;

    fn header_size() -> usize;

    fn size_zero(&self) -> bool;

    fn offset(&self) -> usize;

    fn section_type(&self) -> Type;

    fn entry_size(&self) -> usize;
    fn section_size(&self) -> usize;
}