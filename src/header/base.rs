pub(crate) trait ELFHeader {
    fn deserialize(buf: &[u8]) -> Self;

    fn program_header_table_exists(&self) -> bool;
    fn section_number(&self) -> usize;
    fn section_offset(&self) -> usize;
    fn segment_number(&self) -> usize;
    fn segment_offset(&self) -> usize;
    fn section_name_table_idx(&self) -> usize;
}
