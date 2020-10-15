pub trait ELFHeader {
    fn deserialize(buf: &[u8]) -> Self;
}