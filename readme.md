# Project Idea
This project is a learning journey, for myself, in order to create my own compiler alongside my own code generation. So far the journey has been to learn how to do all stages of compilation. 
- Lexical analysis
- Syntax analysis 
- Semantic analysis
- Optimisation
- Code generation 
This will be predominantly written in rust and I will continue to update the repo with new commits regarding anything new I learn.

## Features
- Tokenisation for a high majority of mathematical expressions 
- Parsing of expressions (Support for multi lined expressions)
- Evaluation of expressions (So far, last line that is evaluated is the returned result)
- Support for the majority of arithmetic operators 
- Support for a handful of mathematical functions 
- Variable (re)assignment & invocation 
- Mathematical constants 
- Unary oprators 

## Project Structure

The project currently consists of three main components which are split into their own rust modules:

1. **Tokeniser** (`tokeniser.rs`): tokeniser.rs is responsible for tokenising inputs into a stream of Tokens.
2. **Parser** (`parser.rs`): parser.rs is responsible for parsing the tokens into their own ASTs depending on the line.
3. **Grammar** (`grammar.ebnf`): Handwritten formal grammar for the project.

## Grammar

The expression language follows this simplified EBNF grammar:

```ebnf
expression ::= assignment | addition_subtraction
assignment ::= 'let' identifier '=' expression
addition_subtraction ::= multiplication_division (('+' | '-') multiplication_division)* 
multiplication_division ::= power (('*' | '/') power)* 
power ::= functions ('^' functions)*
functions ::= [various function definitions]
primary ::= number | identifier | '(' expression ')'
number ::= unary_operator? digit+ ('.' digit+)? 
identifier ::= lowercase_letter (lowercase_letter | digit)*
