use crate::ast::ast::factorial;
use crate::ast::ast::Expr;
use crate::tokeniser::token_enum::Operations;
use crate::tokeniser::token_enum::Token;
use crate::tokeniser::token_enum::TokenStream;
use crate::tokeniser::token_enum::ValueType;
use std::collections::HashMap;

pub type ExprStream = Vec<Expr>;

pub struct Parser {
    tokens: TokenStream,
    cursor: usize,
    line: i16,
    scopes: Vec<HashMap<String, f64>>,
    fmap: HashMap<String, (Vec<String>, Box<Expr>)>,
}

impl Parser {
    /// Construct parser data from token stream (Tokenised input)
    pub fn new(inp_tokens: TokenStream) -> Self {
        Self {
            tokens: inp_tokens,
            cursor: 0usize,
            line: 1i16,
            scopes: vec![HashMap::new()],
            fmap: HashMap::new(),
        }
    }
    /// Evaluate arg 'expr' which is going to be the AST tree representation
    pub fn evaluate(&mut self, exprs: &[Expr]) -> f64 {
        let mut last_result = 0f64;

        for expr in exprs {
            match self.eval(expr) {
                val if val != 0.0 => last_result = val,
                _ => {}
            }
        }

        last_result
    }
    pub fn eval(&mut self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(arb_val) => *arb_val,
            Expr::Function(fn_name, params, exprs) => {
                self.fmap
                    .insert(fn_name.clone(), (params.clone(), exprs.clone()));
                0.0
            }
            Expr::FunctionCall(name, params) => {
                if let Some((param_names, body)) = self.fmap.get(name).cloned() {
                    if param_names.len() != params.len() {
                        panic!("Function {0} called with wrong # of params", name)
                    }
                    self.enter_scope();
                    for (param_name, param_exp) in param_names.iter().zip(params.iter()) {
                        let exp = self.eval(param_exp);
                        self.set_variable(param_name.to_string(), exp);
                    }
                    let res = self.eval(&body);
                    self.exit_scope();
                    res
                } else {
                    panic!("Undefined Function: {0}", name)
                }
            }
            Expr::Variable(id) => self
                .get_variable(id.to_string())
                .ok_or_else(|| panic!("Undeclared Variable: {0}", id))
                .unwrap(),
            Expr::ScopeExp(exprs) => {
                self.enter_scope();
                let mut last_result = 0f64;
                for expr in exprs {
                    last_result = self.eval(expr);
                }
                self.exit_scope();
                last_result
            }
            Expr::Assignment(expr, id) => {
                let val = self.eval(expr);
                self.set_variable(id.to_string(), val);
                0.0
            }
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.eval(left);
                let right_val = self.eval(right);
                match op {
                    Operations::ADD => left_val + right_val,
                    Operations::MINUS => left_val - right_val,
                    Operations::POWER => f64::powf(left_val, right_val),
                    Operations::DIVIDE => {
                        if right_val == 0f64 {
                            panic!("Division by zero")
                        }
                        left_val / right_val
                    }
                    Operations::MULTIPLY => left_val * right_val,
                    Operations::FNLOG => left_val.ln() / right_val.ln(),
                    Operations::FNMOD => left_val % right_val,
                    _ => 0.0,
                }
            }
            Expr::UnaryOp(left, op) => {
                let val = self.eval(left);
                match op {
                    Operations::FNCOS => val.cos(),
                    Operations::FNSIN => val.sin(),
                    Operations::FNTAN => val.tan(),
                    Operations::FNFACT => factorial(val),
                    Operations::MINUS => -val,
                    Operations::NOT => -(val + 1.0),
                    Operations::FNABS => val.abs(),
                    Operations::FNSQRT => val.sqrt(),
                    Operations::FNEXP => f64::powf(std::f64::consts::E, val),
                    Operations::FNASIN => val.asin(),
                    Operations::FNACOS => val.acos(),
                    Operations::FNATAN => val.atan(),
                    Operations::FNSINH => val.sinh(),
                    Operations::FNCOSH => val.cosh(),
                    Operations::FNTANH => val.tanh(),
                    Operations::FNFLOOR => val.floor(),
                    Operations::FNCEIL => val.ceil(),
                    Operations::FNROUND => val.round(),
                    _ => 0.0,
                }
            }
        }
    }
    /// Advance cursor after consuming element from token stream
    fn advance(&mut self) -> Result<Token, String> {
        let token = match self.tokens.get(self.cursor) {
            None => return Err(format!("Empty input @ {0}", self.cursor)),
            Some(val) => val,
        };
        self.cursor += 1;
        self.line = token.line_number;

        Ok(token.clone())
    }
    /// Expect or error on given operation
    fn expect(&mut self, op: Operations) -> Result<Token, String> {
        let tok = self.advance()?;

        if tok.operation != Some(op) {
            return Err(format!(
                "Expected {0:#?} got {1:#?} at {2}",
                op,
                tok.operation.unwrap(),
                self.cursor
            ));
        }

        Ok(tok)
    }
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    fn get_variable(&self, id: String) -> Option<f64> {
        for scope in self.scopes.iter().rev() {
            if let Some(&val) = scope.get(&id) {
                return Some(val);
            }
        }
        None
    }
    fn set_variable(&mut self, id: String, value: f64) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(id, value);
        }
    }
    /// Peek into current cursor element without consuming data
    fn peek(&mut self) -> Option<Token> {
        self.tokens.get(self.cursor).cloned()
    }
    /// Parse tokens into AST nodes
    pub fn parse_lines(&mut self) -> Result<Vec<Expr>, String> {
        let mut ret = Vec::new();
        while self.cursor < self.tokens.len() {
            ret.push(self.parse_tokens()?);
        }
        Ok(ret)
    }
    pub fn parse_tokens(&mut self) -> Result<Expr, String> {
        let start_line = self.line;

        let expr = self.parse_addition_and_subtraction()?;

        if let Some(tok) = self.peek() {
            if tok.line_number > start_line {
                return Ok(expr);
            }
        }
        Ok(expr)
    }
    /// Handle addition & subtraction
    fn parse_addition_and_subtraction(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication_and_division()?;

        while let Some(parsed_tok) = self.peek() {
            match parsed_tok.operation {
                Some(Operations::ADD) | Some(Operations::MINUS) => {
                    self.advance()?;
                    let right = self.parse_multiplication_and_division()?;
                    left = Expr::BinaryOp(
                        Box::new(left),
                        parsed_tok.operation.unwrap(),
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// Handle multiplication and division
    fn parse_multiplication_and_division(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;

        while let Some(parsed_tok) = self.peek() {
            match parsed_tok.operation {
                Some(Operations::MULTIPLY) | Some(Operations::DIVIDE) => {
                    self.advance()?;
                    let right = self.parse_power()?;
                    left = Expr::BinaryOp(
                        Box::new(left),
                        parsed_tok.operation.unwrap(),
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// Handle powers
    fn parse_power(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_fn()?;

        while let Some(parsed_tok) = self.peek() {
            match parsed_tok.operation {
                Some(Operations::POWER) => {
                    self.advance()?;
                    let right = self.parse_fn()?;
                    left = Expr::BinaryOp(Box::new(left), Operations::POWER, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// Handle all functions like sin/cos/tan and factorial
    fn parse_fn(&mut self) -> Result<Expr, String> {
        if let Some(parsed_tok) = self.peek() {
            match parsed_tok.operation {
                Some(Operations::FNCOS)
                | Some(Operations::FNSIN)
                | Some(Operations::FNTAN)
                | Some(Operations::FNABS)
                | Some(Operations::FNSQRT)
                | Some(Operations::FNEXP)
                | Some(Operations::FNASIN)
                | Some(Operations::FNACOS)
                | Some(Operations::FNATAN)
                | Some(Operations::FNSINH)
                | Some(Operations::FNCOSH)
                | Some(Operations::FNTANH)
                | Some(Operations::FNFLOOR)
                | Some(Operations::FNCEIL)
                | Some(Operations::FNROUND)
                | Some(Operations::FNFACT) => {
                    self.advance()?;
                    let expr = self.parse_primary()?;
                    return Ok(Expr::UnaryOp(Box::new(expr), parsed_tok.operation.unwrap()));
                }
                Some(Operations::FNLOG) => {
                    // Consume log
                    self.advance()?;
                    self.expect(Operations::LPAREN)?;
                    let expo = self.parse_addition_and_subtraction()?;
                    let base = if self
                        .peek()
                        .map_or(false, |tok| tok.operation == Some(Operations::COMMA))
                    {
                        self.advance()?;
                        self.parse_addition_and_subtraction()?
                    } else {
                        Expr::Number(10.0)
                    };
                    self.expect(Operations::RPAREN)?;
                    return Ok(Expr::BinaryOp(
                        Box::new(expo),
                        Operations::FNLOG,
                        Box::new(base),
                    ));
                }
                _ => {}
            }
        }
        let mut expr = self.parse_primary()?;

        if let Some(Token {
            operation: Some(Operations::FNMOD),
            ..
        }) = self.peek()
        {
            self.advance()?;
            let right_expr = self.parse_primary()?;
            expr = Expr::BinaryOp(Box::new(expr), Operations::FNMOD, Box::new(right_expr));
        }

        if let Some(Token {
            operation: Some(Operations::FNFACT),
            ..
        }) = self.peek()
        {
            self.advance()?;
            expr = Expr::UnaryOp(Box::new(expr), Operations::FNFACT);
        }
        Ok(expr)
    }
    fn parse_get_or_set(&mut self) -> Option<Expr> {
        if let Some(Token {
            value: Some(ValueType::Identifier(id)),
            ..
        }) = self.peek()
        {
            self.advance().ok()?;
            if let Some(Token {
                operation: Some(Operations::VARASSIGN),
                ..
            }) = self.peek()
            {
                self.advance().ok()?;
                let expr = self.parse_addition_and_subtraction().ok()?;
                return Some(Expr::Assignment(Box::new(expr), id));
            } else {
                return Some(Expr::Variable(id));
            }
        }
        None
    }
    fn parse_scope(&mut self) -> Result<Expr, String> {
        self.expect(Operations::LBRACE)?;
        let mut exprs = Vec::new();
        while self
            .peek()
            .map_or(false, |tok| tok.operation != Some(Operations::RBRACE))
        {
            exprs.push(self.parse_tokens()?);
        }
        self.expect(Operations::RBRACE)?;
        Ok(Expr::ScopeExp(exprs))
    }
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        self.advance()?;
        let identifier = match self.advance()?.value {
            Some(ValueType::Identifier(id)) => id,
            _ => return Err(format!("Expected identifier after let @ {0}", self.cursor)),
        };
        self.expect(Operations::VARASSIGN)?;
        let expr = self.parse_addition_and_subtraction()?;
        Ok(Expr::Assignment(Box::new(expr), identifier))
    }
    fn parse_unary_minus(&mut self) -> Result<Expr, String> {
        self.advance()?;
        let parsed_exp = self.parse_primary()?;
        Ok(Expr::UnaryOp(Box::new(parsed_exp), Operations::MINUS))
    }
    fn parse_unary_not(&mut self) -> Result<Expr, String> {
        self.advance()?;
        let parsed_exp = self.parse_primary()?;
        Ok(Expr::UnaryOp(Box::new(parsed_exp), Operations::NOT))
    }
    fn parse_custom_function(&mut self) -> Result<Expr, String> {
        self.advance()?;
        //fn id (param) { body }
        let fn_name = match self.advance()?.value {
            Some(ValueType::Identifier(id)) => id,
            _ => return Err(format!("Expected function name @ {0}", self.cursor)),
        };
        self.expect(Operations::LPAREN)?;
        let mut param_vec = Vec::new();
        while let Some(tok) = self.peek() {
            if tok.operation == Some(Operations::RPAREN) {
                self.advance()?;
                break;
            }

            match tok.value {
                Some(ValueType::Identifier(param)) => {
                    param_vec.push(param);
                    self.advance()?;
                    if self
                        .peek()
                        .map_or(false, |t| t.operation == Some(Operations::COMMA))
                    {
                        self.advance()?;
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected alphanumeric parameter @ {0}",
                        self.cursor
                    ))
                }
            }
        }
        let fn_body = self.parse_scope()?;
        Ok(Expr::Function(fn_name, param_vec, Box::new(fn_body)))
    }
    fn parse_custom_function_call(&mut self, id: String) -> Result<Expr, String> {
        //(param)
        self.advance()?;
        let mut param_vec = Vec::new();
        while let Some(tok) = self.peek() {
            if tok.operation == Some(Operations::RPAREN) {
                self.advance()?;
                break;
            }

            param_vec.push(Box::new(self.parse_addition_and_subtraction()?));
            if let Some(Token {
                operation: Some(Operations::COMMA),
                ..
            }) = self.peek()
            {
                self.advance()?;
            }
        }

        Ok(Expr::FunctionCall(id, param_vec))
    }
    /// Handle raw value
    fn parse_primary(&mut self) -> Result<Expr, String> {
        if let Some(tok) = self.peek() {
            match tok.operation {
                Some(Operations::FNDEFINE) => return self.parse_custom_function(),
                _ => {}
            }
        }
        if let Some(expr) = self.parse_get_or_set() {
            if let Expr::Variable(ref fn_name) = expr {
                if self
                    .peek()
                    .map_or(false, |tok| tok.operation == Some(Operations::LPAREN))
                {
                    return self.parse_custom_function_call(fn_name.to_string());
                }
            }
            return Ok(expr);
        }
        if let Some(tok) = self.peek() {
            match tok.operation {
                Some(Operations::LBRACE) => {
                    return self.parse_scope();
                }
                Some(Operations::VARLET) => {
                    return self.parse_assignment();
                }
                Some(Operations::MINUS) => {
                    return self.parse_unary_minus();
                }
                Some(Operations::NOT) => {
                    return self.parse_unary_not();
                }
                _ => {}
            }
        }

        let curr_token = self.advance()?;
        match curr_token.operation {
            Some(Operations::LPAREN) => {
                let parsed_exp = self.parse_addition_and_subtraction()?;
                self.expect(Operations::RPAREN)
                    .map_err(|_| format!("Missing ')' @ {0}", self.cursor))?;
                Ok(parsed_exp)
            }
            _ => match curr_token.value {
                Some(ValueType::Number(val)) => Ok(Expr::Number(val)),
                _ => Err(format!("Expected number @ {0}", self.cursor)),
            },
        }
    }
}
