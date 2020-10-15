pub trait Segment {
    type Header;

    fn new(header: Self::Header) -> Self;

    fn header_size() -> usize;

    fn header_deserialize(
        buf: &[u8],
        header_start: usize,
    ) -> Result<Self::Header, Box<dyn std::error::Error>>;
}
