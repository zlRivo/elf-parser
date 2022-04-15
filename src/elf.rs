pub const ELF_MAGIC_NUM: &[u8] = &[0x7F, 0x45, 0x4C, 0x46];
pub const ELF32_HEADER_SIZE: usize = 52;
pub const ELF64_HEADER_SIZE: usize = 64;
pub const EI_NIDENT: usize = 16;

// Custom types
type HalfWord = u16;
type Word = u32;
type Address64 = u64;
type Address32 = u32;
type Offset64 = u64;
type Offset32 = u32;

#[derive(Debug, Eq, PartialEq)]
pub enum BitType {
    _32,
    _64
}

#[derive(Debug, Eq, PartialEq)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}

#[derive(Debug, Eq, PartialEq)]
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
pub struct ProgramHeader32 {
    pub r#type: Word,
    pub offset: Offset32,
    pub vaddr: Address32,
    pub paddr: Address32,
    pub filesz: Word,
    pub memsz: Word,
    pub flags: Word,
    pub align: Word
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
pub struct Elf32 {
    header: ElfHeader32,
    phtable: ProgramHeader32,
    sections: Vec<u8>,
    shtable: SectionHeader32
}

impl Elf32 {
    fn new(header: ElfHeader32,
           phtable: ProgramHeader32,
           sections: Vec<u8>,
           shtable: SectionHeader32) -> Self {
        Self {
            header,
            phtable,
            sections,
            shtable
        }
    }

    fn get_entry_point(&self) -> u32 {
        self.header.e_entry
    }

    // ...
}
