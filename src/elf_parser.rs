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

    todo!();
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
