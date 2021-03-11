mod tests {
    use elf_utilities::{
        file, header,
        section::{self, Contents64},
        segment, Elf64Half, Elf64Off,
    };

    #[test]
    fn generate_elf64_test() {
        let mut f = file::ELF64::default();
        assert_eq!(2, f.sections.len());
        assert_eq!("", f.sections[0].name);
        assert_eq!(".shstrtab", f.sections[1].name);
        assert_eq!(
            header::Ehdr64 {
                e_ident: [
                    0x7f, 0x45, 0x4c, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ],
                e_type: 0,
                e_machine: 0,
                e_version: 0,
                e_entry: 0,
                e_phoff: header::Ehdr64::SIZE as Elf64Off,
                e_shoff: header::Ehdr64::SIZE as Elf64Off + 0xb,
                e_flags: 0,
                e_ehsize: header::Ehdr64::SIZE as Elf64Half,
                e_phentsize: segment::Phdr64::SIZE as Elf64Half,
                e_phnum: 0,
                e_shentsize: section::Shdr64::SIZE as Elf64Half,
                e_shnum: 2,
                e_shstrndx: 1,
            },
            f.ehdr
        );
        assert_eq!(
            section::Shdr64 {
                sh_name: 1,
                sh_type: section::Type::StrTab.into(),
                sh_flags: 0,
                sh_addr: 0,
                sh_offset: 0x40,
                sh_size: 0xb,
                sh_link: 0,
                sh_info: 0,
                sh_addralign: 1,
                sh_entsize: 0,
            },
            f.sections[1].header
        );

        f.add_section(section::Section64::new(
            ".test1".to_string(),
            section::ShdrPreparation64::default(),
            section::Contents64::Raw(vec![0x00; 1024]),
        ));

        assert_eq!(
            header::Ehdr64 {
                e_ident: [
                    0x7f, 0x45, 0x4c, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ],
                e_type: 0,
                e_machine: 0,
                e_version: 0,
                e_entry: 0,
                e_phoff: header::Ehdr64::SIZE as Elf64Off,
                e_shoff: header::Ehdr64::SIZE as Elf64Off + 0xb + 1024,
                e_flags: 0,
                e_ehsize: header::Ehdr64::SIZE as Elf64Half,
                e_phentsize: segment::Phdr64::SIZE as Elf64Half,
                e_phnum: 0,
                e_shentsize: section::Shdr64::SIZE as Elf64Half,
                e_shnum: 3,
                e_shstrndx: 2,
            },
            f.ehdr
        );
        // 追加された .test1セクションの情報が正当であるか
        assert_eq!(
            section::Shdr64 {
                sh_name: 12,
                sh_type: section::Type::Null.into(),
                sh_flags: 0,
                sh_addr: 0,
                sh_offset: 0x4b,
                sh_size: 1024,
                sh_link: 0,
                sh_info: 0,
                sh_addralign: 0,
                sh_entsize: 0,
            },
            f.sections[1].header
        );
        assert!(matches!(f.sections[2].contents, Contents64::StrTab(_)));
    }
}
