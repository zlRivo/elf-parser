use crate::endianness::Endianness;

pub const ELF_MAGIC_NUM: &[u8] = &[0x7F, 0x45, 0x4C, 0x46];
pub const ELF32_HEADER_SIZE: usize = 52;
pub const ELF64_HEADER_SIZE: usize = 64;
pub const EI_NIDENT: usize = 16;

// Custom types
type HalfWord = u16;
type Word = u32;
type XWord = u64;
type Address64 = u64;
type Address32 = u32;
type Offset64 = u64;
type Offset32 = u32;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BitType {
    _32,
    _64
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ABI {
    UnixSystemV,
    HP_UX,
    NetBSD,
    Linux,
    SunSolaris,
    IBM_AIX,
    SGI_Irix,
    FreeBSD,
    CompaqTRU64,
    NovellModesto,
    OpenBSD,
    ARM_EABI,
    ARM,
    Standalone
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FileType {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MachineType {
    None,
    SPARC,
    Intel_80386,
    Motorola_68000,
    Intel_i860,
    MIPS_I,
    Intel_i960,
    PowerPC,
    ARM,
    Intel_IA64,
    x64,
    RISC_V
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum HeaderVersion {
    None,
    Current
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ProgramHeaderType {
    PT_NULL,
    PT_LOAD,
    PT_DYNAMIC,
    PT_INTERP,
    PT_NOTE,
    PT_SHLIB,
    PT_PHDR,
    PT_TLS,
    // Linux specific
    GNU_PROPERTY,
    GNU_EH_FRAME,
    GNU_STACK,
    GNU_RELRO
    
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ProgramHeaderFlag {
    PF_X,
    PF_W,
    PF_R
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ElfIdent {
    pub e_bits: BitType,
    pub e_endianness: Endianness,
    pub e_header_format_version: u8,
    pub e_abi: ABI,
    pub e_abi_version: u8
}

#[derive(Debug, Eq, PartialEq)]
pub struct ElfHeader64 {
    pub e_ident: ElfIdent,
    pub e_type: FileType,
    pub e_machine: MachineType,
    pub e_version: HeaderVersion,
    pub e_entry: Address64,
    pub e_phoff: Offset64,
    pub e_shoff: Offset64,
    pub e_flags: Word,
    pub e_ehsize: HalfWord,
    pub e_phentsize: HalfWord,
    pub e_phnum: HalfWord,
    pub e_shentsize: HalfWord,
    pub e_shnum: HalfWord,
    pub e_shstrndx: HalfWord
}

#[derive(Debug, Eq, PartialEq)]
pub struct ElfHeader32 {
    pub e_ident: ElfIdent,
    pub e_type: FileType,
    pub e_machine: MachineType,
    pub e_version: HeaderVersion,
    pub e_entry: Address32,
    pub e_phoff: Offset32,
    pub e_shoff: Offset32,
    pub e_flags: Word,
    pub e_ehsize: HalfWord,
    pub e_phentsize: HalfWord,
    pub e_phnum: HalfWord,
    pub e_shentsize: HalfWord,
    pub e_shnum: HalfWord,
    pub e_shstrndx: HalfWord
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramHeader64 {
    pub r#type: ProgramHeaderType,
    pub flags: Vec<ProgramHeaderFlag>,
    pub offset: Offset64,
    pub vaddr: Address64,
    pub paddr: Address64,
    pub filesz: XWord,
    pub memsz: XWord,
    pub align: XWord
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramHeader32 {
    pub r#type: ProgramHeaderType,
    pub offset: Offset32,
    pub vaddr: Address32,
    pub paddr: Address32,
    pub filesz: Word,
    pub memsz: Word,
    pub flags: Vec<ProgramHeaderFlag>,
    pub align: Word
}

#[derive(Debug, Eq, PartialEq)]
pub struct SectionHeader64 {
    pub sh_name: Word,
    pub sh_type: Word,
    pub sh_flags: Word,
    pub sh_addr: Address64,
    pub sh_offset: Offset64,
    pub sh_size: Word,
    pub sh_link: Word,
    pub sh_info: Word,
    pub sh_addralign: Word,
    pub sh_entsize: Word
}

#[derive(Debug, Eq, PartialEq)]
pub struct SectionHeader32 {
    pub sh_name: Word,
    pub sh_type: Word,
    pub sh_flags: Word,
    pub sh_addr: Address32,
    pub sh_offset: Offset32,
    pub sh_size: Word,
    pub sh_link: Word,
    pub sh_info: Word,
    pub sh_addralign: Word,
    pub sh_entsize: Word
}

#[derive(Debug, Eq, PartialEq)]
pub struct Elf64 {
    header: ElfHeader64,
    phtable: Vec<ProgramHeader64>,
    sections_data: Vec<u8>,
    shtable: Vec<SectionHeader64>
}

#[derive(Debug, Eq, PartialEq)]
pub struct Elf32 {
    header: ElfHeader32,
    phtable: Vec<ProgramHeader32>,
    sections_data: Vec<u8>,
    shtable: Vec<SectionHeader32>
}

impl Elf32 {
    fn new(header: ElfHeader32,
           phtable: Vec<ProgramHeader32>,
           sections_data: Vec<u8>,
           shtable: Vec<SectionHeader32>) -> Self {
        Self {
            header,
            phtable,
            sections_data,
            shtable
        }
    }

    fn get_entry_point(&self) -> u32 {
        self.header.e_entry
    }

    // ...
}

impl Elf64 {
    fn new(header: ElfHeader64,
           phtable: Vec<ProgramHeader64>,
           sections_data: Vec<u8>,
           shtable: Vec<SectionHeader64>) -> Self {
        Self {
            header,
            phtable,
            sections_data,
            shtable
        }
    }

    fn get_entry_point(&self) -> u64 {
        self.header.e_entry
    }

    // ...
}
