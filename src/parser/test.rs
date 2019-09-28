#[cfg(test)]
mod test {

    use crate::parser::ast::Expression;

    use crate::parser::circuit_parser;

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

    macro_rules! binary_string_test {
        ($txt: expr, $expected: expr) => {
            let parser = circuit_parser::LiteralParser::new();
            assert_eq!(&format!("{:?}", parser.parse($txt).unwrap()), $expected);
        };
    }

    #[test]
    pub fn literal_test() {
        let parser = circuit_parser::LiteralParser::new();
        assert!(!parser.parse("").is_ok());
        assert!(!parser.parse("0b").is_ok());
        assert!(!parser.parse("1b0").is_ok());
        binary_string_test!("0b0", "Vector([false])");
        binary_string_test!("0b1", "Vector([true])");
        binary_string_test!("0b00", "Vector([false, false])");
        binary_string_test!("0b01", "Vector([false, true])");
        binary_string_test!("0b11", "Vector([true, true])");
        binary_string_test!("0b10", "Vector([true, false])");
        binary_string_test!("0x0", "Integer(0)");
        binary_string_test!("0x1", "Integer(1)");
        binary_string_test!("0x10", "Integer(16)");
        binary_string_test!("0x01", "Integer(1)");
        binary_string_test!("0x00", "Integer(0)");
    }
}
