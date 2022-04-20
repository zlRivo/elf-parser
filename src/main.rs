use elf_parser::elf_parser::*;
use elf_parser::elf::*;
use elf_parser::elf::{EI_NIDENT};
use std::fs;

fn main() {
    let content = fs::read("/bin/ls").unwrap();
    Elf64::parse(&content).unwrap();
}
