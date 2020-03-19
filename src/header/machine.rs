use crate::*;

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
    ANY(Elf64Half),
}
