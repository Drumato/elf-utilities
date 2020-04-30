use crate::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ELFMACHINE {
    // No machine
    EMNONE,
    // AT&T WE 32100
    EMM32,
    // SUN SPARC
    EMSPARC,
    // Intel 80386
    EM386,
    // Motorola m68k family
    EM68K,
    // Motorola m88k family
    EM88K,
    // Intel MCU
    EMIAMCU,
    // Intel 80860
    EM860,
    // MIPS R3000 big-endian
    EMMIPS,
    // IBM System/370
    EMS370,
    // MIPS R3000 little-endian
    EMMIPSRS3LE,

    // reserved 11-14

    // HPPA
    EMPARISC,
    // Fujitsu VPP500
    EMVPP500,
    // Sun's "v8plus"
    EMSPARC32PLUS,
    // Intel 80960
    EM960,
    // PowerPC
    EMPPC,
    // PowerPC 64-bit
    EMPPC64,
    // IBM S390
    EMS390,
    // IBM SPU/SPC
    EMSPU,

    // reserved 24-35

    // NEC V700 series
    EMV800,
    // Fujitsu FR20
    EMFR20,
    // TRW RH-32
    EMRH32,
    // Motorola RCE
    EMRCE,
    // ARM
    EMARM,
    // Digital Alpha
    EMFAKEALPHA,
    // Hitachi SH
    EMSH,
    // SPARC v9 64-bit
    EMSPARCV9,
    // Siemens Tricore
    EMTRICORE,
    // Argonaut RISC Core
    EMARC,
    // Hitachi H8/300
    EMH8300,
    // Hitachi H8/300H
    EMH8300H,
    // Hitachi H8S
    EMH8S,
    // Hitachi H8/500
    EMH8500,
    // Stanford MIPS-X
    EMMIPSX,
    // Motorola Coldfire
    EMCOLDFIRE,
    // Motorola M68HC12
    EM68HC12,
    // Fujitsu MMA Multimedia Accelerator
    EMMMA,
    // Siemens PCP
    EMPCP,
    // Sony nCPU embedded RISC
    EMNCPU,
    // Denso NDR1 microprocessor
    EMNDR1,
    // Motorola Start*Core processor
    EMSTARCORE,
    // Toyota ME16 processor
    EMME16,
    // STMicroelectronic ST100 processor
    EMST100,
    // Advanced Logic Corp. TinyJ embedded processor
    EMTINYJ,
    // Advanced Micro Devices X86-64 processor
    EMX8664,
    // Sony DSP Processor
    EMPDSP,
    // Digital Equipment Corp. PDP-10
    EMPDP10,
    // Digital Equipment Corp. PDP-11
    EMPDP11,
    // Siemens FX66 microcontroller
    EMFX66,
    // STMicroelectronics ST9+ 8/16 bit microcontroller
    EMST9PLUS,
    // STMicroelectronics ST7 8-bit microcontroller
    EMST7,
    // Motorola MC68HC16 Microcontroller
    EM68HC16,
    // Motorola MC68HC11 Microcontroller
    EM68HC11,
    // Motorola MC68HC08 Microcontroller
    EM68HC08,
    // Motorola MC68HC05 Microcontroller
    EM68HC05,
    // Silicon Graphics SVx
    EMSVX,
    // STMicroelectronics ST19 8-bit cpu
    EMST19,
    // Digital VAX
    EMVAX,
    // Axis Communications 32-bit embedded processor
    EMCRIS,
    // Infineon Technologies 32-bit embedded cpu
    EMJAVELIN,
    // Element 14 64-bit DSP processor
    EMFIREPATH,
    // LSI Logic's 16-bit DSP processor
    EMZSP,
    // Donald Knuth's educational 64-bit processor
    EMMMIX,
    // Harvard's machine-independent format
    EMHUANY,
    // SiTera Prism
    EMPRISM,
    // Atmel AVR 8-bit microcontroller
    EMAVR,
    // Fujitsu FR30
    EMFR30,
    // Mitsubishi D10V
    EMD10V,
    // Mitsubishi D30V
    EMD30V,
    // Renesas V850 (formerly NEC V850)
    EMV850,
    // Renesas M32R (formerly Mitsubishi M32R)
    EMM32R,
    // Matsushita MN10300
    EMMN10300,
    // Matsushita MN10200
    EMMN10200,
    // picoJava
    EMPJ,
    // OpenRISC 1000 32-bit embedded processor
    EMOR1K,
    // ARC International ARCompact processor
    EMARCCOMPACT,
    // Tensilica Xtensa Architecture
    EMXTENSA,
    // Old Sunplus S+core7 backend magic number.
    // Written in the absence of an ABI.
    EMSCOREOLD,
    // Alphamosaic VideoCore processor
    EMVIDEOCORE,
    // Thompson Multimedia General Purpose Processor
    EMTMMGPP,
    // National Semiconductor 32000 series
    EMNS32K,
    // Tenor Network TPC processor
    EMTPC,
    // Old value for picoJava. Deprecated.
    EMPJOLD,
    // Trebia SNP 1000 processor
    EMSNP1K,
    // STMicroelectronics ST200 microcontroller
    EMST200,
    ANY(Elf64Half),
}

impl ELFMACHINE {
    pub fn to_bytes(&self) -> Elf64Half {
        match self {
            Self::EMX8664 => 62,
            Self::ANY(c) => *c,
            _ => panic!("not implemented -> {:?}", self),
        }
    }
}

impl From<Elf64Half> for ELFMACHINE {
    fn from(bytes: Elf64Half) -> Self {
        match bytes {
            62 => Self::EMX8664,
            _ => Self::ANY(bytes),
        }
    }
}
