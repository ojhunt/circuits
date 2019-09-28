mod ast;
mod circuit_parser;
mod lexer;
mod parser;

#[test]
pub fn identifier_test() {
    let parser = circuit_grammar::IdParser::new();
    assert!(parser.parse("foo").is_ok());
    assert_eq!(
        &format!("{:?}", parser.parse(" foo").unwrap()),
        "Ident(\"foo\")"
    );
    assert!(!parser.parse("1foo").is_ok());
}

#[test]
pub fn circuit_test() {
    let parser = circuit_grammar::CircuitParser::new();
    assert!(!parser.parse("").is_ok());
    assert!(parser.parse("circuit Foo").is_ok());
    assert!(parser.parse("circuit Foo<>").is_ok());
}
