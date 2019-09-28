#[cfg(test)]
use crate::parser::circuit_parser;

#[test]
pub fn identifier_test() {
    let parser = circuit_parser::IdParser::new();
    assert!(parser.parse("foo").is_ok());
    assert_eq!(
        &format!("{:?}", parser.parse(" foo").unwrap()),
        "Ident(\"foo\")"
    );
    assert!(!parser.parse("1foo").is_ok());
}

#[test]
pub fn circuit_test() {
    let parser = circuit_parser::CircuitParser::new();
    assert!(!parser.parse("").is_ok());
    assert!(parser.parse("circuit Foo").is_ok());
    assert!(parser.parse("circuit Foo<>").is_ok());
}

#[test]
pub fn literal_test() {
    let parser = circuit_parser::LiteralParser::new();
    assert!(!parser.parse("").is_ok());
    assert!(!parser.parse("0b").is_ok());
    assert_eq!(
        &format!("{:?}", parser.parse("0b0").unwrap()),
        "Vector([false])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b1").unwrap()),
        "Vector([true])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b00").unwrap()),
        "Vector([false, false])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b10").unwrap()),
        "Vector([true, false])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b11").unwrap()),
        "Vector([true, true])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b01").unwrap()),
        "Vector([false, true])"
    );
}
