#[cfg(test)]
mod test {

    use crate::parser::circuit_parser;

    pub fn variable_test() {
        let parser = circuit_parser::VariableNameParser::new();
        assert!(parser.parse("foo").is_ok());
        assert_eq!(
            &format!("{:?}", parser.parse(" foo").unwrap()),
            "Ident(\"foo\")"
        );
        assert!(!parser.parse("1foo").is_ok());
    }
    pub fn typename_test() {
        let parser = circuit_parser::TypeNameParser::new();
        assert!(parser.parse("Foo").is_ok());
        assert_eq!(
            &format!("{:?}", parser.parse(" foo").unwrap()),
            "Ident(\"foo\")"
        );
        assert!(!parser.parse("1foo").is_ok());
    }

    macro_rules! parser_test {
        ($testname: ident, $type: ty, $txt: expr, $expected: expr) => {
            #[test]
            pub fn $testname() {
                let parser = <$type>::new();
                let expected = $expected;
                assert_eq!(&format!("{:?}", parser.parse($txt).unwrap()), expected);
            }
        };
    }
    parser_test!(
        test_variable,
        circuit_parser::VariableNameParser,
        "a_circuit",
        "Ident(\"a_circuit\")"
    );

    parser_test!(
        test_typename,
        circuit_parser::TypeNameParser,
        "Circuit",
        "Ident(\"Circuit\")"
    );
    parser_test!(
        test_circuit,
        circuit_parser::CircuitParser,
        "circuit Foo{}",
        "Circuit { name: Ident(\"Foo\"), parameters: [], declarations: [] }"
    );
    parser_test!(
        test_circuit_params,
        circuit_parser::CircuitParser,
        "circuit Foo<A:Int>{}",
        "Circuit { name: Ident(\"Foo\"), parameters: [TypeParameter { name: Ident(\"A\"), constraints: Some(Resolve(Ident(\"Int\"))) }], declarations: [] }"
    );
    parser_test!(
        test_basic_alias,
        circuit_parser::CircuitParser,
        "circuit Foo<A:Int>{
            type F = Foo
        }",
        "Circuit { name: Ident(\"Foo\"), parameters: [TypeParameter { name: Ident(\"A\"), constraints: Some(Resolve(Ident(\"Int\"))) }], declarations: [TypeAlias(Ident(\"F\"), [], Resolve(Ident(\"Foo\")))] }"
    );
    parser_test!(
        test_basic_alias2,
        circuit_parser::CircuitParser,
        "circuit Foo<A:Int>{
            type F<i:Foo> = Foo<i, A>
        }",
        "Circuit { name: Ident(\"Foo\"), parameters: [TypeParameter { name: Ident(\"A\"), constraints: Some(Resolve(Ident(\"Int\"))) }], declarations: [TypeAlias(Ident(\"F\"), [TypeParameter { name: Ident(\"i\"), constraints: Some(Resolve(Ident(\"Foo\"))) }], Apply(Resolve(Ident(\"Foo\")), [Value(Binary(Resolve(Ident(\"i\")), [])), Type(Resolve(Ident(\"A\")))]))] }"
    );
    parser_test!(
        test_basic_expression,
        circuit_parser::ExprParser,
        "a < b",
        "Binary(Resolve(Ident(\"a\")), [BinaryTail { operation: Ident(\"<\"), expression: Resolve(Ident(\"b\")) }])"
    );
    parser_test!(
        test_basic_unary_expression,
        circuit_parser::ExprParser,
        "* b",
        "Binary(Unary(Ident(\"*\"), Resolve(Ident(\"b\"))), [])"
    );

    #[test]
    pub fn circuit_test() {
        let parser = circuit_parser::CircuitParser::new();
        assert!(!parser.parse("").is_ok());
        assert!(!parser.parse("circuit Foo").is_ok());
        assert!(!parser.parse("circuit Foo<>").is_ok());
        assert!(parser.parse("circuit Foo{circuit Bar<A>{}}").is_ok());
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
