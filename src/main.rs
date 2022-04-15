use elf_parser::elf_parser::*;
use elf_parser::elf::{EI_NIDENT};
use std::fs;

fn main() {
    let content = fs::read("/bin/ls").unwrap();
    let ident = parse_ident(&content[..EI_NIDENT]);
    println!("{:?}", ident);
}
