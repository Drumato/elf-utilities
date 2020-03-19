#[allow(unused)]
pub mod header;

/* Type for a 16-bit quantity.  */
pub type Elf32Half = u16;
pub type Elf64Half = u16;

/* Types for signed and unsigned 32-bit quantities.  */
pub type Elf32Word = u32;
pub type Elf64Word = u32;
pub type Elf32Sword = i32;
pub type Elf64Sword = i32;

/* Types for signed and unsigned 64-bit quantities.  */
pub type Elf32Xword = u64;
pub type Elf64Xword = u64;
pub type Elf32Sxword = i64;
pub type Elf64Sxword = i64;

/* Type of addresses.  */
pub type Elf32Addr = u32;
pub type Elf64Addr = u64;

/* Type of file offsets.  */
pub type Elf32Off = u32;
pub type Elf64Off = u64;

/* Type for section indices, which are 16-bit quantities.  */
pub type Elf32Section = u16;
pub type Elf64Section = u16;

/* Type for version symbol information.  */
pub type Elf32Versym = Elf32Half;
pub type Elf64Versym = Elf64Half;

#[cfg(test)]
mod header_tests {
    use super::*;

    #[test]
    fn construct_identification_test() {
        let mut ehdr = header::Ehdr64::new();
        ehdr.set_class(header::ELFCLASS::CLASS64);
        ehdr.set_data(header::ELFDATA::DATA2LSB);
        ehdr.set_version(header::ELFVERSION::VERSIONCURRENT);
        ehdr.set_osabi(header::ELFOSABI::SYSV);

        assert_eq!(
            ehdr.get_identification(),
            0x7f454c46020101000000000000000000
        );

        ehdr.set_elf_type(header::ELFTYPE::TYPEEXEC);
        assert_eq!(ehdr.get_elf_type(), header::ELFTYPE::TYPEEXEC);
    }

    #[test]
    fn construct_identification_with_buildter_test() {
        let ehdr = header::Ehdr64Builder::new()
            .class(header::ELFCLASS::CLASS64)
            .data(header::ELFDATA::DATA2LSB)
            .version(header::ELFVERSION::VERSIONCURRENT)
            .osabi(header::ELFOSABI::SYSV)
            .elf_type(header::ELFTYPE::TYPEEXEC)
            .finalize();

        assert_eq!(
            ehdr.get_identification(),
            0x7f454c46020101000000000000000000
        );
    }

    #[test]
    fn set_elf_machine_architecture_test() {
        let mut ehdr = header::Ehdr64::new();
        ehdr.set_machine(header::ELFMACHINE::EMX8664);

        assert_eq!(ehdr.get_machine(), header::ELFMACHINE::EMX8664);
    }
}
