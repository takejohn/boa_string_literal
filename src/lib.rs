#[doc(hidden)]
pub mod internal;
mod utils;

/// Generates a const expression of [`boa_string::JsStr`] from a string literal
/// or a const expression of string.
///
/// If all characters in the string are within the Latin-1 range,
/// the `JsStr` will be Latin-1 encoding.
/// Otherwise, the `JsStr` will be UTF-16 encoding.
///
/// # Examples
/// ```
/// use boa_string::JsStr;
/// use boa_string_literal::js_str;
/// 
/// const s1: JsStr = js_str!("Hello, world!");
/// assert!(s1.is_latin1());
/// assert_eq!(s1, JsStr::latin1("Hello, world!".as_bytes()));
/// 
/// const s2: JsStr = js_str!("🦀");
/// assert!(!s2.is_latin1());
/// assert_eq!(s2, JsStr::utf16(&[0xD83E, 0xDD80]));
/// ```
#[macro_export]
macro_rules! js_str {
    ( $text:expr $(,)? ) => {
        const {
            const STR: &str = $text;
            const STATS: $crate::internal::StrStats = $crate::internal::str_stats($text);
            const IS_LATIN1: bool = STATS.is_latin1;
            const LEN: usize = STATS.len;
            type BufType = $crate::internal::BufferType<IS_LATIN1>;
            type Item = <BufType as $crate::internal::BufferTypeItem>::Item;
            const BUF: [Item; LEN] = const {
                let mut buf: [Item; LEN] = [0; LEN];
                BufType::fill(&mut buf, STR);
                buf
            };
            BufType::cast(&BUF)
        }
    };
}
