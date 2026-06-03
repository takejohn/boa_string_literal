use crate::utils::*;
use boa_string::JsStr;

pub struct StrStats {
    pub is_latin1: bool,
    pub len: usize,
}

pub trait BufferTypeItem {
    type Item;
}

pub enum BufferType<const IS_LATIN1: bool> {}

impl BufferTypeItem for BufferType<true> {
    type Item = u8;
}

impl BufferType<true> {
    pub const fn fill<'a, const N: usize>(buf: &'a mut [u8; N], s: &str) {
        let mut bytes = s.as_bytes();
        let mut i: usize = 0;
        while let Some(ch) = pop_first_char(&mut bytes) {
            assert!(char_is_latin1(ch));
            buf[i] = ch as u8;
            i += 1;
        }
    }

    pub const fn cast<'a>(buf: &'a [u8]) -> JsStr<'a> {
        JsStr::latin1(buf)
    }
}

impl BufferTypeItem for BufferType<false> {
    type Item = u16;
}

impl BufferType<false> {
    pub const fn fill<'a, const N: usize>(buf: &'a mut [u16; N], s: &str) {
        let mut bytes = s.as_bytes();
        let mut i = 0;
        while let Some(ch) = pop_first_char(&mut bytes) {
            if (ch as u32) <= 0xFFFF {
                buf[i] = ch as u16;
                i += 1;
            } else {
                (buf[i], buf[i + 1]) = char_to_surrogate_pair(ch);
                i += 2;
            }
        }
    }

    pub const fn cast<'a>(buf: &'a [u16]) -> JsStr<'a> {
        JsStr::utf16(buf)
    }
}

pub const fn str_stats(s: &str) -> StrStats {
    let mut is_latin1 = true;
    let mut len: usize = 0;
    let mut bytes = s.as_bytes();
    while let Some(ch) = pop_first_char(&mut bytes) {
        if char_is_latin1(ch) {
            len += 1;
        } else {
            is_latin1 = false;
            len += char_code_unit_count(ch);
        }
    }
    StrStats { is_latin1, len }
}
