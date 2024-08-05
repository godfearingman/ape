use crate::ast::parser::Parser;
use crate::tokeniser::tokeniser::Tokeniser;
pub mod ast;
pub mod tokeniser;

fn main() {
    let tokens = Tokeniser::new("let x = 3\nlet y=2\nlet z = x\nz + y".to_string())
        .to_tokens()
        .unwrap();
    let mut parser = Parser::new(tokens);
    let expressions = parser.parse_lines().unwrap();
    let evaluated_result = parser.evaluate(&expressions);
    println!("{evaluated_result}");
}
