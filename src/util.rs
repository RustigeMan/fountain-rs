pub fn is_ascii_char(byte: u8) -> bool {
    byte & 0b10000000 == 0
}
