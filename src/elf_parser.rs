use crate::elf::*;
use crate::parse_error::ParseError;
use crate::endianness::{self, Endianness};

pub fn parse_ident(ident: &[u8]) -> Result<ElfIdent, ParseError> {
    if ident.len() < EI_NIDENT { return Err(ParseError::TooSmallIdent) } // Validate size
        
    let magic = &ident[0..4]; // Get magic number
    if magic != ELF_MAGIC_NUM { return Err(ParseError::NotELF) } // Ensure file is elf
    
    let bits = ident[4]; // 64 bits or 32
    let bits = match bits {
        1 => BitType::_32,
        2 => BitType::_64,
        _ => return Err(ParseError::UnsupportedMode)
    };

    let endianness = ident[5];
    let endianness = match endianness {
        1 => Endianness::LittleEndian,
        2 => Endianness::BigEndian,
        _ => return Err(ParseError::UnsupportedEndianness)
    };

    let header_format_version = ident[6];

    let abi = ident[7]; // Application binary interface
    let abi = match abi {
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

    let abi_ver = ident[8];

    Ok(ElfIdent {
        e_bits: bits,
        e_endianness: endianness,
        e_header_format_version: header_format_version,
        e_abi: abi,
        e_abi_version: abi_ver
    })
}

impl Elf32 {
    fn parse_header(h: &[u8]) -> Result<ElfHeader32, ParseError> {
        if h.len() < ELF32_HEADER_SIZE { return Err(ParseError::TooSmallHeader) } // Validate size
        let ident = &h[..EI_NIDENT]; // Ident part
        let ident = parse_ident(ident)?; // Parse ident
        let endian = ident.e_endianness;
        
        let filetype = endianness::read16(&[h[EI_NIDENT], h[EI_NIDENT + 1]], endian);
        let filetype = match filetype {
            0 => FileType::ET_NONE,
            1 => FileType::ET_NONE,
            2 => FileType::ET_EXEC,
            3 => FileType::ET_DYN,
            4 => FileType::ET_CORE,
            _ => return Err(ParseError::UnsupportedFileType)
        };

        let machine = endianness::read16(&[h[EI_NIDENT + 2], h[EI_NIDENT + 3]], endian);
        let machine = match machine {
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

        let version = endianness::read32(
            &[
                h[EI_NIDENT + 4],
                h[EI_NIDENT + 5],
                h[EI_NIDENT + 6],
                h[EI_NIDENT + 7]
            ],
            endian
        );
        let version = match version {
            0 => HeaderVersion::None,
            1 => HeaderVersion::Current,
            _ => return Err(ParseError::UnsupportedVersion)
        };

        let entry = endianness::read32(
            &[
                h[EI_NIDENT + 8],
                h[EI_NIDENT + 9],
                h[EI_NIDENT + 10],
                h[EI_NIDENT + 11]
            ],
            endian
        );

        let phoffset = endianness::read32(
            &[
                h[EI_NIDENT + 12],
                h[EI_NIDENT + 13],
                h[EI_NIDENT + 14],
                h[EI_NIDENT + 15]
            ],
            endian
        );

        let shoffset = endianness::read32(
            &[
                h[EI_NIDENT + 16],
                h[EI_NIDENT + 17],
                h[EI_NIDENT + 18],
                h[EI_NIDENT + 19]
            ],
            endian
        );
        
        let flags = endianness::read32(
            &[
                h[EI_NIDENT + 20],
                h[EI_NIDENT + 21],
                h[EI_NIDENT + 22],
                h[EI_NIDENT + 23]
            ],
            endian
        );

        let headersz = endianness::read16(&[h[EI_NIDENT + 24], h[EI_NIDENT + 25]], endian);
        let phentsize = endianness::read16(&[h[EI_NIDENT + 26], h[EI_NIDENT + 27]], endian);
        let phcount = endianness::read16(&[h[EI_NIDENT + 28], h[EI_NIDENT + 29]], endian);
        let shentsize = endianness::read16(&[h[EI_NIDENT + 30], h[EI_NIDENT + 31]], endian);
        let shcount = endianness::read16(&[h[EI_NIDENT + 32], h[EI_NIDENT + 33]], endian);
        let shstrndx = endianness::read16(&[h[EI_NIDENT + 34], h[EI_NIDENT + 35]], endian);

        Ok(ElfHeader32 {
            e_ident: ident,
            e_type: filetype,
            e_machine: machine,
            e_version: version,
            e_entry: entry,
            e_phoff: phoffset,
            e_shoff: shoffset,
            e_flags: flags,
            e_ehsize: headersz,
            e_phentsize: phentsize,
            e_phnum: phcount,
            e_shentsize: shentsize,
            e_shnum: shcount,
            e_shstrndx: shstrndx
        })
    }
    
    pub fn parse(f: &[u8]) -> Result<Self, ParseError> {

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
}
