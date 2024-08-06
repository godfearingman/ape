use crate::tokeniser::token_enum::Operations;
use crate::tokeniser::token_enum::Token;
use crate::tokeniser::token_enum::TokenStream;
use crate::tokeniser::token_enum::ValueType;
use core::iter::Peekable;
use std::collections::HashMap;

pub struct Tokeniser {
    tape: String,
    line_number: i16,
}

impl Tokeniser {
    pub fn new(inp_tape: String) -> Self {
        Self {
            tape: inp_tape,
            line_number: 0i16,
        }
    }
    fn parse_alphanumeric<I>(&self, chars: &mut Peekable<I>) -> Option<Token>
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut parsed_string = String::new();
        let mut iter = chars.clone();

        let fn_map: HashMap<String, Operations> = [
            (String::from("sin"), Operations::FNSIN),
            (String::from("cos"), Operations::FNCOS),
            (String::from("tan"), Operations::FNTAN),
            (String::from("log"), Operations::FNLOG),
            (String::from("abs"), Operations::FNABS),
            (String::from("sqrt"), Operations::FNSQRT),
            (String::from("exp"), Operations::FNEXP),
            (String::from("asin"), Operations::FNASIN),
            (String::from("acos"), Operations::FNACOS),
            (String::from("atan"), Operations::FNATAN),
            (String::from("sinh"), Operations::FNSINH),
            (String::from("cosh"), Operations::FNCOSH),
            (String::from("tanh"), Operations::FNTANH),
            (String::from("floor"), Operations::FNFLOOR),
            (String::from("ceil"), Operations::FNCEIL),
            (String::from("round"), Operations::FNROUND),
            (String::from("let"), Operations::VARLET),
        ]
        .iter()
        .cloned()
        .collect();

        let constant_map: HashMap<String, f64> = [
            (String::from("e"), std::f64::consts::E),
            (String::from("pi"), std::f64::consts::PI),
        ]
        .iter()
        .cloned()
        .collect();

        while let Some(ch) = iter.next() {
            if ch.is_alphabetic() {
                parsed_string.push(ch);
            } else {
                break;
            }
        }

        if let Some(&op) = fn_map.get(&parsed_string) {
            for _ in 0..parsed_string.len() {
                chars.next();
            }

            return Some(Token {
                operation: Some(op),
                value: None,
                line_number: self.line_number,
            });
        }

        if let Some(&val) = constant_map.get(&parsed_string) {
            for _ in 0..parsed_string.len() {
                chars.next();
            }

            return Some(Token {
                operation: None,
                value: Some(ValueType::Number(val)),
                line_number: self.line_number,
            });
        }
        for _ in 0..parsed_string.len() {
            chars.next();
        }
        Some(Token {
            operation: None,
            value: Some(ValueType::Identifier(parsed_string)),
            line_number: self.line_number,
        })

        //None
    }
    fn parse_number<I>(chars: &mut Peekable<I>) -> Result<(f64, i8), String>
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut parsed_index = 0i8;
        let mut parsed_val_as_string = String::new();

        if chars
            .clone()
            .take_while(|&c| c != ' ')
            .filter(|&c| c == '.')
            .count()
            > 1
        {
            return Err("Failed to parse number, multiple decimal points".to_string());
        }

        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' => {
                    parsed_val_as_string.push(ch);
                    chars.next();
                    parsed_index += 1;
                }
                '.' => {
                    parsed_val_as_string.push(ch);
                    chars.next();
                    parsed_index += 1;
                }
                _ => break,
            }
        }
        parsed_val_as_string
            .parse::<f64>()
            .map(|val| (val, parsed_index))
            .map_err(|_| String::from("Failed to parse value"))
    }
    pub fn to_tokens(&mut self) -> Result<TokenStream, String> {
        // Setup return vector
        let mut return_stream = TokenStream::new();
        let mut inp_chars = self.tape.chars().peekable();

        while let Some(&ch) = inp_chars.peek() {
            match ch {
                'a'..='z' => {
                    if let Some(tok) = self.parse_alphanumeric(&mut inp_chars) {
                        return_stream.push(tok);
                    } else {
                        return Err(String::from("Unrecognised alphanumeric value"));
                    }
                }
                '0'..='9' => {
                    //Parse number
                    let (parsed_val, parsed_idx) = Self::parse_number(&mut inp_chars.clone())?;
                    return_stream.push(Token {
                        operation: None,
                        value: Some(ValueType::Number(parsed_val)),
                        line_number: self.line_number,
                    });
                    for _ in 0..parsed_idx {
                        inp_chars.next();
                    }
                }
                '{' => {
                    return_stream.push(Token {
                        operation: Some(Operations::LBRACE),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '}' => {
                    return_stream.push(Token {
                        operation: Some(Operations::RBRACE),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '=' => {
                    return_stream.push(Token {
                        operation: Some(Operations::VARASSIGN),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '~' => {
                    return_stream.push(Token {
                        operation: Some(Operations::NOT),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '+' => {
                    return_stream.push(Token {
                        operation: Some(Operations::ADD),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '-' => {
                    return_stream.push(Token {
                        operation: Some(Operations::MINUS),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '^' => {
                    return_stream.push(Token {
                        operation: Some(Operations::POWER),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '*' => {
                    return_stream.push(Token {
                        operation: Some(Operations::MULTIPLY),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '/' => {
                    return_stream.push(Token {
                        operation: Some(Operations::DIVIDE),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '!' => {
                    return_stream.push(Token {
                        operation: Some(Operations::FNFACT),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '(' => {
                    return_stream.push(Token {
                        operation: Some(Operations::LPAREN),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                ')' => {
                    return_stream.push(Token {
                        operation: Some(Operations::RPAREN),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                ',' => {
                    return_stream.push(Token {
                        operation: Some(Operations::COMMA),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '%' => {
                    return_stream.push(Token {
                        operation: Some(Operations::FNMOD),
                        value: None,
                        line_number: self.line_number,
                    });
                    inp_chars.next();
                }
                '\n' => {
                    self.line_number += 1;
                    inp_chars.next();
                }
                ' ' => {
                    inp_chars.next();
                }
                _ => return Err(String::from("Invalid character")),
            }
        }

        Ok(return_stream)
    }
}
