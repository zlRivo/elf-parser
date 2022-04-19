use elf_parser::elf_parser::*;
use elf_parser::elf::*;
use elf_parser::elf::{EI_NIDENT};
use std::fs;

fn main() {
    let content = fs::read("file").unwrap();
    let header = Elf32::parse_header(&content).unwrap();
    println!("{:?}", header);
}
