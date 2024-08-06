#[cfg(test)]
mod tests {
    use crate::ast::parser::Parser;
    use crate::tokeniser::tokeniser::Tokeniser;

    fn parse_and_eval(input: &str) -> f64 {
        let mut tokeniser = Tokeniser::new(input.to_string());
        let tokens = tokeniser.to_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_lines().unwrap();
        parser.evaluate(&ast)
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(parse_and_eval("2 + 3"), 5.0);
        assert_eq!(parse_and_eval("10 - 4"), 6.0);
        assert_eq!(parse_and_eval("3 * 4"), 12.0);
        assert_eq!(parse_and_eval("20 / 5"), 4.0);
        assert_eq!(parse_and_eval("2 ^ 3"), 8.0);
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(parse_and_eval("2 + 3 * 4"), 14.0);
        assert_eq!(parse_and_eval("(2 + 3) * 4"), 20.0);
        assert_eq!(parse_and_eval("2 ^ 3 + 1"), 9.0);
    }

    #[test]
    fn test_variables() {
        assert_eq!(parse_and_eval("let x = 5\nx + 3"), 8.0);
        assert_eq!(parse_and_eval("let x = 5\nlet y = 3\nx * y"), 15.0);
    }

    #[test]
    fn test_scopes() {
        assert_eq!(parse_and_eval("{ 2 + 3 }"), 5.0);
        assert_eq!(parse_and_eval("let x = 1\n{ let x = 2\nx + 3 }\nx"), 1.0);
    }

    #[test]
    fn test_functions() {
        assert_eq!(parse_and_eval("sin(0)"), 0.0);
        assert_eq!(parse_and_eval("cos(0)"), 1.0);
        assert_eq!(parse_and_eval("log(100, 10)"), 2.0);
        assert_eq!(parse_and_eval("sqrt(16)"), 4.0);
    }

    #[test]
    fn test_complex_scopes() {
        let input = r#"
        let x = 1
        {
            let y = 2
            let z = {
                let x = 3
                x + y
            }
            z + x
        }
        "#;
        assert_eq!(parse_and_eval(input), 6.0);
    }
    #[test]
    #[should_panic(expected = "Missing ')'")]
    fn test_unmatched_parentheses() {
        parse_and_eval("(2 + 3");
    }

    #[test]
    #[should_panic(expected = "Undeclared Variable: x")]
    fn test_undeclared_variable() {
        parse_and_eval("x + 5");
    }

    #[test]
    #[should_panic(expected = "Invalid character")]
    fn test_invalid_character() {
        parse_and_eval("2 @ 3");
    }

    #[test]
    #[should_panic(expected = "Expected identifier after let")]
    fn test_invalid_let_statement() {
        parse_and_eval("let 5 = 10");
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        parse_and_eval("10 / 0");
    }

    #[test]
    #[should_panic]
    fn test_invalid_function_call() {
        parse_and_eval("notafunction(10)");
    }

    #[test]
    #[should_panic]
    fn test_mismatched_scope_braces() {
        parse_and_eval("{ let x = 5 } }");
    }
}
