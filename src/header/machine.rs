use crate::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Machine {
    // No machine
    None,
    // AT&T WE 32100
    M32,
    // SUN SPARC
    SPARC,
    // Intel 80386
    Intel386,
    // Motorola m68k family
    M68K,
    // Motorola m88k family
    M88K,
    // Intel MCU
    IntelMCU,
    // Intel 80860
    Intel80860,
    // MIPS R3000 big-endian
    MIPS,
    // IBM System/370
    S370,
    // MIPS R3000 little-endian
    MIPSRS3LE,

    // reserved 11-14

    // HPPA
    Parisc,
    // Fujitsu VPP500
    VPP500,
    // Sun's "v8plus"
    SPARC32Plus,
    // Intel 80960
    Intel80960,
    // PowerPC
    PowerPC,
    // PowerPC 64-bit
    PowerPC65,
    // IBM S390
    S390,
    // IBM SPU/SPC
    SPU,

    // reserved 24-35

    // NEC V700 series
    V800,
    // Fujitsu FR20
    FR20,
    // TRW RH-32
    RH32,
    // Motorola RCE
    RCE,
    // ARM
    Arm,
    // Digital Alpha
    FakeAlpha,
    // Hitachi SH
    EMSH,
    // SPARC v9 64-bit
    EMSPARCV9,
    // Siemens Tricore
    Tricore,
    // Argonaut RISC Core
    ARC,
    // Hitachi H8/300
    H8300,
    // Hitachi H8/300H
    H8300H,
    // Hitachi H8S
    H8S,
    // Hitachi H8/500
    H8500,
    // Stanford MIPS-X
    MIPSX,
    // Motorola Coldfire
    Coldfire,
    // Motorola M68HC12
    M68HC12,
    // Fujitsu MMA Multimedia Accelerator
    MMA,
    // Siemens PCP
    PCP,
    // Sony nCPU embedded RISC
    NCPU,
    // Denso NDR1 microprocessor
    NDR1,
    // Motorola Start*Core processor
    StarCore,
    // Toyota ME16 processor
    ME16,
    // STMicroelectronic ST100 processor
    ST100,
    // Advanced Logic Corp. TinyJ embedded processor
    TinyJ,
    // Advanced Micro Devices X86-64 processor
    X8664,
    // Sony DSP Processor
    PSDP,
    // Digital Equipment Corp. PDP-10
    PDP10,
    // Digital Equipment Corp. PDP-11
    PDP11,
    // Siemens FX66 microcontroller
    FX66,
    // STMicroelectronics ST9+ 8/16 bit microcontroller
    ST9Plus,
    // STMicroelectronics ST7 8-bit microcontroller
    ST7,
    // Motorola MC68HC16 Microcontroller
    MC68HC16,
    // Motorola MC68HC11 Microcontroller
    MC68HC11,
    // Motorola MC68HC08 Microcontroller
    MC68HC08,
    // Motorola MC68HC05 Microcontroller
    MC68HC05,
    // Silicon Graphics SVx
    SVx,
    // STMicroelectronics ST19 8-bit cpu
    ST19,
    // Digital VAX
    VAX,
    // Axis Communications 32-bit embedded processor
    CRIS,
    // Infineon Technologies 32-bit embedded cpu
    Javelin,
    // Element 14 64-bit DSP processor
    Firepath,
    // LSI Logic's 16-bit DSP processor
    ZSP,
    // Donald Knuth's educational 64-bit processor
    MMIX,
    // Harvard's machine-independent format
    HUANY,
    // SiTera Prism
    Prism,
    // Atmel AVR 8-bit microcontroller
    AVR,
    // Fujitsu FR30
    FR30,
    // Mitsubishi D10V
    D10V,
    // Mitsubishi D30V
    D30V,
    // Renesas V850 (formerly NEC V850)
    V850,
    // Renesas M32R (formerly Mitsubishi M32R)
    M32R,
    // Matsushita MN10300
    MN10300,
    // Matsushita MN10200
    MN10200,
    // picoJava
    PicoJava,
    // OpenRISC 1000 32-bit embedded processor
    OR1K,
    // ARC International ARCompact processor
    ARCompact,
    // Tensilica Xtensa Architecture
    Xtensa,
    // Old Sunplus S+core7 backend magic number.
    // Written in the absence of an ABI.
    SCoreOld,
    // Alphamosaic VideoCore processor
    VideoCore,
    // Thompson Multimedia General Purpose Processor
    TMMGPP,
    // National Semiconductor 32000 series
    NS32K,
    // Tenor Network TPC processor
    TPC,
    // Old value for picoJava. Deprecated.
    PicoJavaOld,
    // Trebia SNP 1000 processor
    SNP1K,
    // STMicroelectronics ST200 microcontroller
    ST200,
    Any(Elf64Half),
}

impl Machine {
    pub fn to_bytes(&self) -> Elf64Half {
        match self {
            Self::X8664 => 62,
            Self::Any(c) => *c,
            _ => panic!("not implemented -> {:?}", self),
        }
    }
}

impl From<Elf64Half> for Machine {
    fn from(bytes: Elf64Half) -> Self {
        match bytes {
            62 => Self::X8664,
            _ => Self::Any(bytes),
        }
    }
}
