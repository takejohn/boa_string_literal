pub(crate) const fn char_is_latin1(ch: char) -> bool {
    (ch as u32) <= 0xFF
}

pub(crate) const fn char_code_unit_count(ch: char) -> usize {
    if (ch as u32) <= 0xFFFF { 1 } else { 2 }
}

pub(crate) const fn char_to_surrogate_pair(ch: char) -> (u16, u16) {
    assert!(char_code_unit_count(ch) == 2);
    let hi = (((ch as u32) - 0x10000) / 0x400 + 0xD800) as u16;
    let lo = (((ch as u32) - 0x10000) % 0x400 + 0xDC00) as u16;
    (hi, lo)
}

pub(crate) const fn pop_first_char(s: &mut &[u8]) -> Option<char> {
    let Some(first) = pop_first_byte(s) else {
        return None;
    };
    let (trailing_len, first_content) = split_utf8_first_byte(first);
    let mut ch = first_content as u32;
    let mut i = 0;
    while i < trailing_len {
        let byte = pop_first_byte(s).expect("Invalid byte pattern");
        let (head, tail) = apply_mask(byte, 0b1100_0000);
        assert!(head == 0b1000_0000, "Invalid byte pattern");
        ch = (ch << 6) | (tail as u32);
        i += 1;
    }
    Some(char::from_u32(ch).expect("Invalid byte pattern"))
}

const fn pop_first_byte(bytes: &mut &[u8]) -> Option<u8> {
    match bytes.split_first() {
        Some((first, rest)) => {
            *bytes = rest;
            Some(*first)
        }
        None => None,
    }
}

const fn split_utf8_first_byte(byte: u8) -> (usize, u8) {
    if let (0, ret) = apply_mask(byte, 0b1000_0000) {
        return (0, ret);
    }
    assert!((byte & 0b1100_0000) != 0b1000_0000, "Invalid byte pattern");
    if let (0b1100_0000, ret) = apply_mask(byte, 0b1110_0000) {
        return (1, ret);
    }
    if let (0b1110_0000, ret) = apply_mask(byte, 0b1111_0000) {
        return (2, ret);
    }
    if let (0b1111_0000, ret) = apply_mask(byte, 0b1111_1000) {
        return (3, ret);
    }
    panic!("Invalid byte pattern");
}

const fn apply_mask(byte: u8, mask: u8) -> (u8, u8) {
    ((byte & mask), (byte & !mask))
}
