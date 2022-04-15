use crate::elf::*;

fn parse32(f: &[u8]) -> Elf32 {
    let header = &f[0..52]; // Read the header
}
