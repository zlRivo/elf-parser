const EI_NIDENT: usize = 16;

// Custom types
type HalfWord = u16;
type Word = u32;
type Address64 = u64;
type Address32 = u32;
type Offset64 = u64;
type Offset32 = u32;

pub struct ElfHeader64 {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: HalfWord,
    pub e_machine: HalfWord,
    pub e_version: Word,
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

pub struct ElfHeader32 {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: HalfWord,
    pub e_machine: HalfWord,
    pub e_version: Word,
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
