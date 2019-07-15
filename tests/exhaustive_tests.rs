extern crate unicode_xid;
use unicode_xid::UnicodeXID;
/// A `char` in Rust is a Unicode Scalar Value
///
/// See: http://www.unicode.org/glossary/#unicode_scalar_value
fn all_valid_chars() -> impl Iterator<Item = char> {
    (0u32..=0xD7FF).chain(0xE000u32..=0x10FFFF).map(|u|
        core::convert::TryFrom::try_from(u)
            .expect("The selected range should be infallible if the docs match impl")
    )

}

#[test]
fn all_valid_chars_do_not_panic_for_is_xid_start() {
    for c in all_valid_chars() {
        let _ = UnicodeXID::is_xid_start(c);
    }
}

#[test]
fn all_valid_chars_do_not_panic_for_is_xid_continue() {
    for c in all_valid_chars() {
        let _ = UnicodeXID::is_xid_continue(c);
    }
}

fn all_invalid_chars() -> impl Iterator<Item = char> {
    (0xD800u32..0xE000).chain(0x110000u32..=core::u32::MAX)
        .map(|u| unsafe { core::char::from_u32_unchecked(u)})
}

#[test]
fn all_invalid_chars_do_not_panic_for_is_xid_start() {
    for c in all_invalid_chars() {
        let _ = UnicodeXID::is_xid_start(c);
    }
}


#[test]
fn all_invalid_chars_do_not_panic_for_is_xid_continue() {
    for c in all_invalid_chars() {
        let _ = UnicodeXID::is_xid_continue(c);
    }
}
