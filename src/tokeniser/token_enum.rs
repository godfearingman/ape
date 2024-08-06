/// Basic enum to hold all operations for our calculator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operations {
    NOT,
    /// A + B
    ADD,
    /// A - B
    MINUS,
    /// A ^ B
    POWER,
    /// A / B
    DIVIDE,
    /// A * B
    MULTIPLY,
    /// (
    LPAREN,
    /// )
    RPAREN,
    /// cos
    FNCOS,
    /// sin
    FNSIN,
    // tan
    FNTAN,
    // !
    FNFACT,
    // log(a(,b)?)
    FNLOG,
    //,
    COMMA,
    // abs(a)
    FNABS,
    // sqrt(a)
    FNSQRT,
    // exp(a)
    FNEXP,
    // a % b
    FNMOD,
    // asin(a)
    FNASIN,
    //acos(a)
    FNACOS,
    //atan(a)
    FNATAN,
    //sinh(a)
    FNSINH,
    //cosh(a)
    FNCOSH,
    //tanh(a)
    FNTANH,
    //floor(a)
    FNFLOOR,
    //ceil(a)
    FNCEIL,
    //round(a)
    FNROUND,
    // let
    VARLET,
    // =
    VARASSIGN,
    // {
    RBRACE,
    // }
    LBRACE,
    // fn
    FNDEFINE,
}
#[derive(Debug, Clone)]
pub enum ValueType {
    Number(f64),
    Identifier(String),
}

/// Tokens are represented via two properties, operation & value
/// Tokens that represent strict values contain None
/// some_token = Token { operation: None, value: Some(5.0) }
#[derive(Debug, Clone)]
pub struct Token {
    /// Field for token operation declared at line 2
    pub operation: Option<Operations>,
    // Field for value instead of operation (See line 14)
    //pub value: Option<f64>,
    pub value: Option<ValueType>,
    pub line_number: i16,
}

/// Custom data type in order to represent a stream of tokens
pub type TokenStream = Vec<Token>;
