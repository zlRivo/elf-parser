use crate::elf::*;
use crate::parse_error::ParseError;
use crate::endianness::{self, Endianness};
use std::cell::Cell;

pub fn parse_ident(ident: &[u8]) -> Result<ElfIdent, ParseError> {
    if ident.len() < EI_NIDENT { return Err(ParseError::TooSmallIdent) } // Validate size

    let magic = &ident[0..4]; // Get magic number
    if magic != ELF_MAGIC_NUM { return Err(ParseError::NotELF) } // Ensure file is elf
    
    // Read a byte
    let idx = Cell::new(4);
    let read = || -> u8 { let temp = idx.get(); let v = ident[temp]; idx.set(temp + 1); v };
    
    // 64 bits or 32
    let bits = match read() {
        1 => BitType::_32,
        2 => BitType::_64,
        _ => return Err(ParseError::UnsupportedMode)
    };

    let endianness = match read() {
        1 => Endianness::LittleEndian,
        2 => Endianness::BigEndian,
        _ => return Err(ParseError::UnsupportedEndianness)
    };

    let header_format_version = read();

    // Application binary interface
    let abi = match read() {
        0 => ABI::UnixSystemV,
        1 => ABI::HP_UX,
        2 => ABI::NetBSD,
        3 => ABI::Linux,
        6 => ABI::SunSolaris,
        7 => ABI::IBM_AIX,
        8 => ABI::SGI_Irix,
        9 => ABI::FreeBSD,
        10 => ABI::CompaqTRU64,
        11 => ABI::NovellModesto,
        12 => ABI::OpenBSD,
        64 => ABI::ARM_EABI,
        97 => ABI::ARM,
        255 => ABI::Standalone,
        _ => return Err(ParseError::UnsupportedABI)
    };

    let abi_ver = read();

    Ok(ElfIdent {
        e_bits: bits,
        e_endianness: endianness,
        e_header_format_version: header_format_version,
        e_abi: abi,
        e_abi_version: abi_ver
    })
}

impl Elf64 {
    pub fn parse_header(h: &[u8]) -> Result<ElfHeader64, ParseError> {
        if h.len() < ELF64_HEADER_SIZE { return Err(ParseError::TooSmallHeader) } // Validate size
        let ident = &h[..EI_NIDENT]; // Ident part
        let ident = parse_ident(ident)?; // Parse ident
        let endian = ident.e_endianness; // Get endian type

        // Closures for reading values
        let idx = Cell::new(EI_NIDENT);
        let r16 = || -> u16 { let temp = idx.get(); let v = endianness::read16(&[h[temp], h[temp + 1]], endian); idx.set(temp + 2); v };
        let r32 = || -> u32 { let temp = idx.get(); let v = endianness::read32(&[h[temp], h[temp + 1], h[temp + 2], h[temp + 3]], endian); idx.set(temp + 4); v };
        let r64 = || -> u64 { let temp = idx.get(); let v = endianness::read64(&[h[temp],     h[temp + 1], h[temp + 2], h[temp + 3],
                                                           h[temp + 4], h[temp + 5], h[temp + 6], h[temp + 7]], endian); idx.set(temp + 8); v };
        let filetype = match r16() {
            0 => FileType::ET_NONE,
            1 => FileType::ET_NONE,
            2 => FileType::ET_EXEC,
            3 => FileType::ET_DYN,
            4 => FileType::ET_CORE,
            _ => return Err(ParseError::UnsupportedFileType)
        };

        let machine = match r16() {
            0 => MachineType::None,
            2 => MachineType::SPARC,
            3 => MachineType::Intel_80386,
            4 => MachineType::Motorola_68000,
            7 => MachineType::Intel_i860,
            8 => MachineType::MIPS_I,
            19 => MachineType::Intel_i960,
            20 => MachineType::PowerPC,
            40 => MachineType::ARM,
            50 => MachineType::Intel_IA64,
            62 => MachineType::x64,
            243 => MachineType::RISC_V,
            _ => return Err(ParseError::UnsupportedMachineType)
        };
        
        let version = match r32() {
            0 => HeaderVersion::None,
            1 => HeaderVersion::Current,
            _ => return Err(ParseError::UnsupportedVersion)
        };
        
        Ok(ElfHeader64 {
            e_ident: ident,
            e_type: filetype,
            e_machine: machine,
            e_version: version,
            e_entry: r64(),
            e_phoff: r64(),
            e_shoff: r64(),
            e_flags: r32(),
            e_ehsize: r16(),
            e_phentsize: r16(),
            e_phnum: r16(),
            e_shentsize: r16(),
            e_shnum: r16(),
            e_shstrndx: r16(),
        })
    }
    
    // TODO: Error handling
    pub fn parse(f: &[u8]) -> Result<Self, ParseError> {
        let header = Self::parse_header(f).unwrap();
        let endian = header.e_ident.e_endianness; // Get endianness
        let mut phtable: Vec<ProgramHeader64> = Vec::with_capacity(header.e_phnum as usize); // To store program headers
        
        // Closures for reading values
        let idx = Cell::new(header.e_phoff as usize);
        let r16 = || -> u16 { let temp = idx.get(); let v = endianness::read16(&[f[temp], f[temp + 1]], endian); idx.set(temp + 2); v };
        let r32 = || -> u32 { let temp = idx.get(); let v = endianness::read32(&[f[temp], f[temp + 1], f[temp + 2], f[temp + 3]], endian); idx.set(temp + 4); v };
        let r64 = || -> u64 { let temp = idx.get(); let v = endianness::read64(&[f[temp],     f[temp + 1], f[temp + 2], f[temp + 3],
                                                                                 f[temp + 4], f[temp + 5], f[temp + 6], f[temp + 7]], endian); idx.set(temp + 8); v };
        // Read each program header
        for _ in 0..header.e_phnum {
            let v = r32();
            println!("{:#02x}", v);
            let phtype = match v {
                0 => { ProgramHeaderType::PT_NULL },
                1 => { ProgramHeaderType::PT_LOAD },
                2 => { ProgramHeaderType::PT_DYNAMIC },
                3 => { ProgramHeaderType::PT_INTERP },
                4 => { ProgramHeaderType::PT_NOTE },
                5 => { ProgramHeaderType::PT_SHLIB },
                6 => { ProgramHeaderType::PT_PHDR },
                7 => { ProgramHeaderType::PT_TLS },
                0x6474E553 => { ProgramHeaderType::GNU_PROPERTY },
                0x6474E550 => { ProgramHeaderType::GNU_EH_FRAME },
                0x6474E551 => { ProgramHeaderType::GNU_STACK },
                0x6474E552 => { ProgramHeaderType::GNU_RELRO },
                _ => { return Err(ParseError::UnsupportedProgramHeaderType) }
            };

            phtable.push(ProgramHeader64 {
                r#type: phtype,
                flags: r32(),
                offset: r64(),
                vaddr: r64(),
                paddr: r64(),
                filesz: r64(),
                memsz: r64(),
                align: r64()
            });
        }
        println!("{:#?}", phtable);

        todo!();
    }
}

impl Elf32 {
    pub fn parse_header(h: &[u8]) -> Result<ElfHeader32, ParseError> {
        if h.len() < ELF32_HEADER_SIZE { return Err(ParseError::TooSmallHeader) } // Validate size
        let ident = &h[..EI_NIDENT]; // Ident part
        let ident = parse_ident(ident)?; // Parse ident
        let endian = ident.e_endianness; // Get endian type
        
        let idx = Cell::new(EI_NIDENT);
        let r16 = || -> u16 { let temp = idx.get(); let v = endianness::read16(&[h[temp], h[temp + 1]], endian); idx.set(temp + 2); v };
        let r32 = || -> u32 { let temp = idx.get(); let v = endianness::read32(&[h[temp], h[temp + 1], h[temp + 2], h[temp + 3]], endian); idx.set(temp + 4); v };
        let r64 = || -> u64 { let temp = idx.get(); let v = endianness::read64(&[h[temp],     h[temp + 1], h[temp + 2], h[temp + 3],
                                                           h[temp + 4], h[temp + 5], h[temp + 6], h[temp + 7]], endian); idx.set(temp + 8); v };
        
        let filetype = match r16() {
            0 => FileType::ET_NONE,
            1 => FileType::ET_NONE,
            2 => FileType::ET_EXEC,
            3 => FileType::ET_DYN,
            4 => FileType::ET_CORE,
            _ => return Err(ParseError::UnsupportedFileType)
        };

        let machine = match r16() {
            0 => MachineType::None,
            2 => MachineType::SPARC,
            3 => MachineType::Intel_80386,
            4 => MachineType::Motorola_68000,
            7 => MachineType::Intel_i860,
            8 => MachineType::MIPS_I,
            19 => MachineType::Intel_i960,
            20 => MachineType::PowerPC,
            40 => MachineType::ARM,
            50 => MachineType::Intel_IA64,
            62 => MachineType::x64,
            243 => MachineType::RISC_V,
            _ => return Err(ParseError::UnsupportedMachineType)
        };

        let version = match r32() {
            0 => HeaderVersion::None,
            1 => HeaderVersion::Current,
            _ => return Err(ParseError::UnsupportedVersion)
        };

        Ok(ElfHeader32 {
            e_ident: ident,
            e_type: filetype,
            e_machine: machine,
            e_version: version,
            e_entry: r32(),
            e_phoff: r32(),
            e_shoff: r32(),
            e_flags: r32(),
            e_ehsize: r16(),
            e_phentsize: r16(),
            e_phnum: r16(),
            e_shentsize: r16(),
            e_shnum: r16(),
            e_shstrndx: r16()
        })
    }
    
    pub fn parse(f: &[u8]) -> Result<Self, ParseError> {
        // Get all the program headers
        let header = Self::parse_header(f).unwrap();
        let endian = header.e_ident.e_endianness; // Get endianness
        
        {
            let start_i = header.e_phoff as usize;
            let phtsize = header.e_phentsize as usize * header.e_phnum as usize;
            let end_i = start_i + phtsize;
            // Loop through each program header
            for i in (start_i..end_i).skip(phtsize) {
                endianness::read16(&[f[i], f[i + 1]], endian);
            }
        }

        todo!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_ident() {
        use super::*;

        let ident = &[
            0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];
        
        assert_eq!(
            Ok(ElfIdent {
                e_bits: BitType::_64,
                e_endianness: Endianness::LittleEndian,
                e_header_format_version: 1,
                e_abi: ABI::UnixSystemV,
                e_abi_version: 0
            }),
            parse_ident(ident)
        );
    }

    #[test]
    // CURRENTLY DOESNT WORK DEPENDING ON PLATFORM
    fn parse_header32() {
        use super::*;
        use std::fs;
        use std::io::prelude::*;
        use std::process::Command;
        use std::path::Path;
        
        // Create dummy file and compile it
        // to compare header
        if !Path::new("temp").exists() { fs::create_dir("temp").unwrap(); }
        if let Err(_) = fs::File::open("temp/main32.c") {
            let mut f = fs::File::create("temp/main32.c").unwrap();
            f.write_all(b"int main(int argc, char** argv) { return 0; }").unwrap();
        }
        Command::new("gcc") // Compile file
            .arg("-m32")
            .arg("temp/main32.c")
            .arg("-o")
            .arg("temp/main32")
            .output().unwrap(); 
        // Read binary content of compiled file
        let data = fs::read("temp/main32").unwrap();
        // Compare headers
        assert_eq!(Elf32::parse_header(&data).unwrap(), ElfHeader32 {
            e_ident: ElfIdent {
                e_bits: BitType::_32,
                e_endianness: Endianness::LittleEndian,
                e_header_format_version: 1,
                e_abi: ABI::UnixSystemV,
                e_abi_version: 0
            },
            e_type: FileType::ET_DYN,
            e_machine: MachineType::Intel_80386,
            e_version: HeaderVersion::Current,
            e_entry: 0x1050,
            e_phoff: 52,
            e_shoff: 13792,
            e_flags: 0,
            e_ehsize: 52,
            e_phentsize: 32,
            e_phnum: 12,
            e_shentsize: 40,
            e_shnum: 30,
            e_shstrndx: 29
        });
    }
    
    #[test]
    // It compares header from the one of
    // "/bin/ls". It assumes the host is on x86_64
    // and it is also not stable on all systems
    fn parse_header64() {
        use super::*;
        use std::fs;

        // Read content of ls binary
        let ls_content = fs::read("/bin/ls").unwrap();
        let header = Elf64::parse_header(&ls_content).unwrap();
        
        assert_eq!(header, ElfHeader64 {
            e_ident: ElfIdent {
                e_bits: BitType::_64,
                e_endianness: Endianness::LittleEndian,
                e_header_format_version: 1,
                e_abi: ABI::UnixSystemV,
                e_abi_version: 0
            },
            e_type: FileType::ET_DYN,
            e_machine: MachineType::x64,
            e_version: HeaderVersion::Current,
            e_entry: 23488,
            e_phoff: 64,
            e_shoff: 144264,
            e_flags: 0,
            e_ehsize: 64,
            e_phentsize: 56,
            e_phnum: 13,
            e_shentsize: 64,
            e_shnum: 28,
            e_shstrndx: 27
        }); 
    }
}
