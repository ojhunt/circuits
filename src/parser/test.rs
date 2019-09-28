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
        "Literal([false])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b1").unwrap()),
        "Literal([true])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b00").unwrap()),
        "Literal([false, false])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b10").unwrap()),
        "Literal([true, false])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b11").unwrap()),
        "Literal([true, true])"
    );
    assert_eq!(
        &format!("{:?}", parser.parse("0b01").unwrap()),
        "Literal([false, true])"
    );
}
