use crate::ast::parser::Parser;
use crate::tokeniser::tokeniser::Tokeniser;
pub mod ast;
pub mod tokeniser;

#[cfg(test)]
pub mod tests;

fn main() {
    let input = r#"
        fn test(x, y) {
            x + y
        }
        test(5,2) + test(5,2)
        "#;
    let tokens = Tokeniser::new(input.to_string()).to_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let expressions = parser.parse_lines().unwrap();
    let evaluated_result = parser.evaluate(&expressions);
    println!("{evaluated_result}");
}
