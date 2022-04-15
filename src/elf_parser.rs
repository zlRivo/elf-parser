use crate::elf::*;
use crate::parse_error::ParseError;

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

fn parse_header32(h: &[u8]) -> Result<ElfHeader32, ParseError> {
    let ident = &h[..EI_NIDENT]; // Ident part
    let ident = parse_ident(ident)?; // Parse ident
    
    let filetype = h[EI_NIDENT + 1] << 8 | h[EI_NIDENT];
    let filetype = match filetype {
        0 => FileType::ET_NONE,
        1 => FileType::ET_NONE,
        2 => FileType::ET_EXEC,
        3 => FileType::ET_DYN,
        4 => FileType::ET_CORE,
        _ => return Err(ParseError::UnsupportedFileType)
    };

    let machine = h[EI_NIDENT + 3] << 8 | h[EI_NIDENT + 2];
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

    let version: u32 = (h[EI_NIDENT + 7] as u32) << 24
        | (h[EI_NIDENT + 6] as u32) << 16
        | (h[EI_NIDENT + 5] as u32) << 8
        | (h[EI_NIDENT + 4] as u32);
    let version = match version {
        0 => HeaderVersion::None,
        1 => HeaderVersion::Current,
        _ => return Err(ParseError::UnsupportedVersion)
    };

    let entry: u32 = (h[EI_NIDENT + 11] as u32) << 24
        | (h[EI_NIDENT + 10] as u32) << 16
        | (h[EI_NIDENT + 9] as u32) << 8
        | (h[EI_NIDENT + 8] as u32);

    let phoffset: u32 = (h[EI_NIDENT + 15] as u32) << 24
        | (h[EI_NIDENT + 14] as u32) << 16
        | (h[EI_NIDENT + 13] as u32) << 8
        | (h[EI_NIDENT + 12] as u32);

    let shoffset: u32 = (h[EI_NIDENT + 19] as u32) << 24
        | (h[EI_NIDENT + 18] as u32) << 16
        | (h[EI_NIDENT + 17] as u32) << 8
        | (h[EI_NIDENT + 16] as u32);
    
    let flags: u32 = (h[EI_NIDENT + 23] as u32) << 24
        | (h[EI_NIDENT + 22] as u32) << 16
        | (h[EI_NIDENT + 21] as u32) << 8
        | (h[EI_NIDENT + 20] as u32);

    let headersz = (h[EI_NIDENT + 25] as u16) << 8 | h[EI_NIDENT + 24] as u16;
    let phentsize = (h[EI_NIDENT + 27] as u16) << 8 | h[EI_NIDENT + 26] as u16;
    let phcount = (h[EI_NIDENT + 29] as u16) << 8 | h[EI_NIDENT + 28] as u16;
    let shentsize = (h[EI_NIDENT + 31] as u16) << 8 | h[EI_NIDENT + 30] as u16;
    let shcount = (h[EI_NIDENT + 33] as u16) << 8 | h[EI_NIDENT + 32] as u16;
    let shstrndx = (h[EI_NIDENT + 35] as u16) << 8 | h[EI_NIDENT + 34] as u16;

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

pub fn parse32(f: &[u8]) -> Result<Elf32, ParseError> {

    todo!();
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
