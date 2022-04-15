pub enum ParseError {
    NotELF,
    TooSmallIdent,
    TooSmallHeader,
    UnsupportedMode,
    UnsupportedEndianness,
    UnsupportedABI
}
