use boa_string::{JsStr, JsStrVariant};
use boa_string_literal::js_str;

#[derive(Debug, PartialEq, Eq)]
enum EqVariant<'a> {
    Latin1(&'a [u8]),
    Utf16(&'a [u16]),
}

impl<'a> From<JsStr<'a>> for EqVariant<'a> {
    fn from(value: JsStr<'a>) -> Self {
        match value.variant() {
            JsStrVariant::Latin1(items) => EqVariant::Latin1(items),
            JsStrVariant::Utf16(items) => EqVariant::Utf16(items),
        }
    }
}

#[test]
fn empty() {
    const S: JsStr = js_str!("");
    assert_eq!(EqVariant::Latin1(&[]), S.into());
}

#[test]
fn ascii_one_char() {
    const S: JsStr = js_str!("\u{0041}");
    assert_eq!(EqVariant::Latin1(&[0x0041]), S.into());
}

#[test]
fn latin1_one_char() {
    const S: JsStr = js_str!("\u{00A1}");
    assert_eq!(EqVariant::Latin1(&[0x00A1]), S.into());
}

#[test]
fn utf16_one_char() {
    const S: JsStr = js_str!("\u{8A9E}");
    assert_eq!(EqVariant::Utf16(&[0x8A9E]), S.into());
}

#[test]
fn utf16_surrogate_pair() {
    const S: JsStr = js_str!("\u{1F60A}");
    assert_eq!(EqVariant::Utf16(&[0xD83D, 0xDE0A]), S.into());
}

#[test]
fn latin1_some_chars() {
    const S: JsStr = js_str!("abc");
    assert_eq!(EqVariant::Latin1(&[0x61, 0x62, 0x63]), S.into());
}

#[test]
fn latin1_and_utf16_chars() {
    const S: JsStr = js_str!("abc\u{1F60A}");
    assert_eq!(
        EqVariant::Utf16(&[0x61, 0x62, 0x63, 0xD83D, 0xDE0A]),
        S.into()
    );
}

#[test]
fn utf16_and_latin1_chars() {
    const S: JsStr = js_str!("\u{1F60A}abc");
    assert_eq!(
        EqVariant::Utf16(&[0xD83D, 0xDE0A, 0x61, 0x62, 0x63]),
        S.into()
    );
}

#[test]
fn input_macro() {
    const S: JsStr = js_str!(concat!("a", "b"));
    assert_eq!(EqVariant::Latin1(&[0x61, 0x62]), S.into());
}

#[test]
fn input_expr() {
    const S: JsStr = js_str!(match str::from_utf8(&[0x61, 0x62]) {
        Ok(v) => v,
        Err(_) => panic!("Malformed UTF-8"),
    });
    assert_eq!(EqVariant::Latin1(&[0x61, 0x62]), S.into());
}

mod bounds {
    use super::*;

    #[test]
    fn utf8_1_byte_min() {
        const S: JsStr = js_str!("\u{0000}");
        assert_eq!(EqVariant::Latin1(&[0x00]), S.into());
    }

    #[test]
    fn utf8_1_byte_max() {
        const S: JsStr = js_str!("\u{007F}");
        assert_eq!(EqVariant::Latin1(&[0x7F]), S.into());
    }

    #[test]
    fn utf8_2_byte_min() {
        const S: JsStr = js_str!("\u{0080}");
        assert_eq!(EqVariant::Latin1(&[0x80]), S.into());
    }

    #[test]
    fn latin1_max() {
        const S: JsStr = js_str!("\u{00FF}");
        assert_eq!(EqVariant::Latin1(&[0xFF]), S.into());
    }

    #[test]
    fn utf16_min() {
        const S: JsStr = js_str!("\u{0100}");
        assert_eq!(EqVariant::Utf16(&[0x0100]), S.into());
    }

    #[test]
    fn utf8_2_byte_max() {
        const S: JsStr = js_str!("\u{07FF}");
        assert_eq!(EqVariant::Utf16(&[0x07FF]), S.into());
    }

    #[test]
    fn utf8_3_byte_min() {
        const S: JsStr = js_str!("\u{0800}");
        assert_eq!(EqVariant::Utf16(&[0x0800]), S.into());
    }

    #[test]
    fn utf8_3_byte_max() {
        const S: JsStr = js_str!("\u{FFFF}");
        assert_eq!(EqVariant::Utf16(&[0xFFFF]), S.into());
    }

    #[test]
    fn surrogate_min() {
        const S: JsStr = js_str!("\u{10000}");
        assert_eq!(EqVariant::Utf16(&[0xD800, 0xDC00]), S.into());
    }

    #[test]
    fn surrogate_max() {
        const S: JsStr = js_str!("\u{10FFFF}");
        assert_eq!(EqVariant::Utf16(&[0xDBFF, 0xDFFF]), S.into());
    }
}
