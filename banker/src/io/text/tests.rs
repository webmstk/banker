use super::*;
use std::io::Cursor;

use rstest::rstest;
use std::mem::discriminant;

#[rstest]
#[case("content", ParseError::MustStartWithQuote)]
#[case("\"content", ParseError::MustEndWithQuote)]
#[case("", ParseError::UnexpectedEOF)]
#[case("\"ooo \"\"Romashka\"\"", ParseError::MustEndWithQuote)]
fn read_qouted_fn_returns_ok_for_valid_input(
    #[case] input: &'static str,
    #[case] expected: ParseError,
) {
    let cursor = Cursor::new(input);
    let result = read_quoted(cursor);

    assert!(
        result.is_err(),
        "expected: {:?}\ngot:      {:?}\ninput:    {}",
        expected,
        result,
        input
    );

    let actual = result.err().unwrap();

    assert!(
        discriminant(&actual) == discriminant(&expected),
        "expected: {:?}\nactual:   {:?}\ninput: {}",
        expected,
        actual,
        input,
    );
}

#[rstest]
#[case("\"boo\"", "boo")]
#[case("\"boo\",", "boo")]
#[case("\"boo\"\n", "boo")]
#[case("\"ooo \"\"Romashka\"\" company\"", "ooo \"Romashka\" company")]
#[case(
    "\"ooo \"\"OOO \"\"Romashka\"\"\"\" company\"",
    "ooo \"OOO \"Romashka\"\" company"
)]
#[case("\"start\"end\"", "start")]
fn read_qouted_fn_returns_err_for_invalid_input(
    #[case] input: &'static str,
    #[case] expected: &'static str,
) {
    let cursor = Cursor::new(input);
    let result = read_quoted(cursor);

    assert!(
        result.is_ok(),
        "expected: {:?}\ngot:      {:?}\ninput:    {}",
        expected,
        result,
        input
    );

    let actual = result.unwrap();

    assert_eq!(actual, expected, "input: {}", input,);
}

#[test]
fn read_qouted_fn_consumes_last_char() {
    let mut cursor = Cursor::new("\"aaa\",\"bbb\"");

    let _ = read_quoted(&mut cursor);
    let result = read_quoted(&mut cursor);

    assert!(result.is_ok());

    let actual = result.unwrap();

    assert_eq!(actual, "bbb");
}

#[test]
fn read_qouted_fn_consumes_newline() {
    let mut cursor = Cursor::new("\"aaa\"\n\"bbb\"");

    let _ = read_quoted(&mut cursor);
    let result = read_quoted(&mut cursor);

    assert!(result.is_ok());

    let actual = result.unwrap();

    assert_eq!(actual, "bbb");
}

#[test]
fn read_qouted_fn_returns_error_for_not_utf8_input() {
    let invalid_bytes = vec![b'"', 0, 159, b'"'];

    let cursor = Cursor::new(invalid_bytes);
    let result = read_quoted(cursor);
    assert!(matches!(
        result.err().unwrap(),
        ParseError::EncodingError(_)
    ));
}

#[rstest]
#[case(("123", b','), "123")]
#[case(("123,", b','), "123")]
#[case(("123,456", b','), "123")]
#[case(("123\n", b'\n'), "123")]
#[case(("123\n456", b','), "123")]
#[case(("123
            ", b'\n'), "123")]
#[case(("123\n", b','), "123")]
fn read_until_fn_returns_ok_for_valid_input(
    #[case] input: (&'static str, u8),
    #[case] expected: &'static str,
) {
    let cursor = Cursor::new(input.0);
    let result = read_until(cursor, input.1);

    assert!(
        result.is_ok(),
        "expected: {:?}\ngot:      {:?}\ninput:    ({},{})",
        expected,
        result,
        input.0,
        input.1,
    );

    let actual = result.unwrap();

    assert_eq!(actual, expected, "input: {}", input.0);
}

#[test]
fn read_until_fn_consumes_last_char() {
    let mut cursor = Cursor::new("aaa,bbb");

    let _ = read_until(&mut cursor, b',');
    let result = read_until(&mut cursor, b',');

    assert!(result.is_ok());

    let actual = result.unwrap();

    assert_eq!(actual, "bbb");
}

#[test]
fn read_until_fn_consumes_newline() {
    let mut cursor = Cursor::new("aaa\nbbb");

    let _ = read_until(&mut cursor, b',');
    let result = read_until(&mut cursor, b',');

    assert!(result.is_ok());

    let actual = result.unwrap();

    assert_eq!(actual, "bbb");
}

#[test]
fn read_until_fn_returns_error_for_empty_input() {
    let cursor = Cursor::new("");
    let result = read_until(cursor, b',');

    assert!(result.is_err());

    let actual = result.err().unwrap();

    matches!(actual, ParseError::UnexpectedEOF);
}

#[test]
fn read_until_fn_returns_error_for_not_utf8_str() {
    let invalid_bytes = vec![0, 159, b','];

    let cursor = Cursor::new(invalid_bytes);
    let result = read_until(cursor, b',');
    assert!(matches!(
        result.err().unwrap(),
        ParseError::EncodingError(_)
    ));
}

#[test]
fn write_quoted_fn_wrapes_str_with_quotes() {
    let mut writer = Vec::new();
    let value = String::from("text");
    let result = write_quoted(&mut writer, &value);

    assert!(result.is_ok());

    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(actual, "\"text\"");
}

#[test]
fn write_quoted_fn_escapes_inner_quotes() {
    let mut writer = Vec::new();
    let value = String::from("aaa \"bbb\" ccc");
    let result = write_quoted(&mut writer, &value);

    assert!(result.is_ok());

    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(actual, "\"aaa \"\"bbb\"\" ccc\"");
}
