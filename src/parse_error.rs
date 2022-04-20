#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    NotELF,
    TooSmallIdent,
    TooSmallHeader,
    UnsupportedMode,
    UnsupportedEndianness,
    UnsupportedABI,
    UnsupportedFileType,
    UnsupportedMachineType,
    UnsupportedVersion,
    UnsupportedProgramHeaderType
}
