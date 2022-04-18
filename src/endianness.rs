#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}

pub fn read16(b: &[u8; 2], endian: Endianness) -> u16 {
    return match endian {
        LittleEndian => (b[1] as u16) << 8 | b[0] as u16,
        BigEndian => (b[0] as u16) << 8 | b[1] as u16
    }
}

pub fn read32(b: &[u8; 4], endian: Endianness) -> u32 {
    return match endian {
        LittleEndian => {
            (b[3] as u32) << 24 |
            (b[2] as u32) << 16 |
            (b[1] as u32) << 8 |
            b[0] as u32
        },
        BigEndian => {
            (b[0] as u32) << 24 |
            (b[1] as u32) << 16 |
            (b[2] as u32) << 8 |
            b[3] as u32
        }
    }
}

pub fn read64(b: &[u8; 8], endian: Endianness) -> u64 {
    return match endian {
        LittleEndian => {
            (b[7] as u64) << 56 |
            (b[6] as u64) << 48 |
            (b[5] as u64) << 40 |
            (b[4] as u64) << 32 |
            (b[3] as u64) << 24 |
            (b[2] as u64) << 16 |
            (b[1] as u64) << 8 |
            b[0] as u64
        },
        BigEndian => {
            (b[0] as u64) << 56 |
            (b[1] as u64) << 48 |
            (b[2] as u64) << 40 |
            (b[3] as u64) << 32 |
            (b[4] as u64) << 24 |
            (b[5] as u64) << 16 |
            (b[6] as u64) << 8 |
            b[7] as u64
        }
    }
}
