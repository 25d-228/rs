use std::error::Error;

use rs::{Key, KeyError, MAX_KEY_BYTES};

fn assert_error_type<T: Error>() {}

#[test]
fn valid_key_preserves_owned_utf8_text() {
    let raw = String::from("users/東京:42");
    let key = Key::new(raw.clone()).expect("the fixture is a valid key");

    assert_eq!(key.as_str(), raw);
    assert_eq!(key.into_inner(), raw);
}

#[test]
fn rejects_empty_keys() {
    assert_eq!(Key::new(String::new()), Err(KeyError::Empty));
}

#[test]
fn accepts_exact_maximum_ascii_length() {
    let raw = "a".repeat(MAX_KEY_BYTES);
    let key = Key::new(raw.clone()).expect("the exact byte limit is valid");

    assert_eq!(key.as_str(), raw);
}

#[test]
fn rejects_one_ascii_byte_over_the_limit() {
    let raw = "a".repeat(MAX_KEY_BYTES + 1);

    assert_eq!(
        Key::new(raw),
        Err(KeyError::TooLong {
            max: MAX_KEY_BYTES,
            actual: MAX_KEY_BYTES + 1,
        })
    );
}

#[test]
fn measures_utf8_length_in_bytes() {
    let raw = "é".repeat(33);

    assert_eq!(
        Key::new(raw),
        Err(KeyError::TooLong {
            max: MAX_KEY_BYTES,
            actual: 66,
        })
    );
}

#[test]
fn reports_whitespace_as_a_utf8_byte_offset() {
    assert_eq!(
        Key::new("é key".to_owned()),
        Err(KeyError::ContainsWhitespace { byte_index: 2 })
    );
}

#[test]
fn whitespace_takes_precedence_when_a_character_is_also_control() {
    assert_eq!(
        Key::new("abc\n".to_owned()),
        Err(KeyError::ContainsWhitespace { byte_index: 3 })
    );
}

#[test]
fn reports_non_whitespace_control_characters() {
    assert_eq!(
        Key::new("ab\0cd".to_owned()),
        Err(KeyError::ContainsControl { byte_index: 2 })
    );
}

#[test]
fn key_error_implements_error_and_has_nonempty_display() {
    assert_error_type::<KeyError>();

    for error in [
        KeyError::Empty,
        KeyError::TooLong {
            max: MAX_KEY_BYTES,
            actual: MAX_KEY_BYTES + 1,
        },
        KeyError::ContainsWhitespace { byte_index: 3 },
        KeyError::ContainsControl { byte_index: 2 },
    ] {
        assert!(
            !error.to_string().trim().is_empty(),
            "Display must identify {error:?}"
        );
    }
}
