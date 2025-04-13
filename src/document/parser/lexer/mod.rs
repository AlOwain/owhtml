mod tokenize;

use super::{Err, Token};

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub errors: Vec<Err>,
}
