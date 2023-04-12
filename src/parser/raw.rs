use super::ElfParser;
use crate::header;

impl ElfParser {
    pub(super) fn peek_raw_elf_identification<'a>(
        &'a self,
        b: &'a [u8],
    ) -> nom::IResult<&'a [u8], header::RawElfIdentification> {
        let mut p = nom::combinator::peek(self.parse_raw_elf_identification());
        let (b, e_ident) = p(b)?;

        Ok((b, e_ident))
    }

    fn parse_raw_elf_identification<'a>(
        &'a self,
    ) -> impl Fn(&'a [u8]) -> nom::IResult<&'a [u8], header::RawElfIdentification> {
        move |b: &[u8]| {
            let (b, e_magic) = nom::bytes::complete::tag(header::ELF_MAGICNUMBER_SIGNATURE)(b)?;

            let mut e_ident = header::RawElfIdentification::new();
            let magicnumber_indices = [
                header::ELF_IDENT_MAGICNUMBER0_INDEX,
                header::ELF_IDENT_MAGICNUMBER1_INDEX,
                header::ELF_IDENT_MAGICNUMBER2_INDEX,
                header::ELF_IDENT_MAGICNUMBER3_INDEX,
            ];

            for idx in magicnumber_indices.iter() {
                e_ident.e_magic[*idx] = e_magic[*idx];
            }

            let (b, e_class) = nom::number::complete::u8(b)?;
            e_ident.e_class = e_class;
            let (b, e_data) = nom::number::complete::u8(b)?;
            e_ident.e_data= e_data ;
            let (b, e_version) = nom::number::complete::u8(b)?;
            e_ident.e_version= e_version;
            let (b, e_osabi) = nom::number::complete::u8(b)?;
            e_ident.e_osabi= e_osabi;
            let (b, e_abiversion) = nom::number::complete::u8(b)?;
            e_ident.e_abiversion= e_abiversion;

            let (b, e_padding) = nom::multi::count(nom::number::complete::u8, header::ELF_IDENT_PADDING_FIELD_LENGTH)(b)?;
            for idx in 0..header::ELF_IDENT_PADDING_FIELD_LENGTH {
                e_ident.e_padding[idx] = e_padding[idx];
            }

            Ok((b, e_ident))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ElfParserConfig;

    use super::*;

    #[test]
    fn test_peek_raw_elf_identification() {
        let p = ElfParserConfig::new().build();
        let input = vec![
            0x7f, 0x45, 0x4c, 0x46, // elf magic number
            0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let result = p.peek_raw_elf_identification(&input);
        assert!(result.is_ok());

        let (b, _e_ident) = result.unwrap();
        assert_eq!(&input, b);
    }

    #[test]
    fn test_parse_raw_elf_identification() {
        let p = ElfParserConfig::new().build();
        let input = vec![
            0x7f, 0x45, 0x4c, 0x46, // elf magic number
            0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let result = p.parse_raw_elf_identification()(&input);
        assert!(result.is_ok());

        let (b, e_ident) = result.unwrap();
        assert_eq!(0, b.len());

        assert_eq!(header::ELF_MAGICNUMBER_SIGNATURE, e_ident.e_magic);
        assert_eq!(0x2, e_ident.e_class);
        assert_eq!(0x1, e_ident.e_data);
        assert_eq!(0x1, e_ident.e_version);
        assert_eq!(0x0, e_ident.e_osabi);
        assert_eq!(0x0, e_ident.e_abiversion);
        assert_eq!([0x00; header::ELF_IDENT_PADDING_FIELD_LENGTH], e_ident.e_padding);
    }
}
