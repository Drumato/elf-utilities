use crate::{header, section, segment};
pub(crate) trait ELF {
    type Header: header::ELFHeader;
    type Section: section::Section;
    type Segment: segment::Segment;

    fn new(header: Self::Header) -> Self;
    fn header(&self) -> Self::Header;

    fn sections_as_mut(&mut self) -> &mut Vec<Self::Section>;
    fn update_sections(&mut self, sections: Vec<Self::Section>);
    fn update_segments(&mut self, segments: Vec<Self::Segment>);
}
