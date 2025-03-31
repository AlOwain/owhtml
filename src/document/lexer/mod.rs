mod state;
mod token;
mod tokenize;

use token::Token::{self, *};

pub fn lexer(document: String) -> Vec<Token> {
    let mut iter = document.chars().peekable();
    let mut tokens = Vec::new();
    let mut state = Default::default();
    loop {
        let token = tokenize::tokenize(&mut state, &mut iter);
        match token {
            Some(EOF) => break,
            Some(token) => tokens.push(token),
            None => continue,
        }
    }
    return tokens;
}
