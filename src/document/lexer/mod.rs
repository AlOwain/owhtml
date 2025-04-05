use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

mod error;
mod state;
mod token;
mod tokenize;

use tokenize::Tokenizer;
pub use {error::Err, state::State, token::Token};

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub errors: Vec<Err>,
}

impl FromStr for Lexer {
    type Err = ();

    fn from_str(document: &str) -> Result<Self, ()> {
        let lexer = Lexer {
            tokens: Vec::new(),
            errors: Vec::new(),
        };
        let iter = document.chars().peekable();
        let mut tokenizer: Tokenizer<Chars> = (lexer, iter).into();

        while let Some(()) = tokenizer.next() {}

        Ok(Lexer {
            tokens: tokenizer.tokens,
            errors: tokenizer.errors,
        })
    }
}

impl<CharIter: Iterator<Item = char>> From<(Lexer, Peekable<CharIter>)> for Tokenizer<CharIter> {
    fn from(tokenizer: (Lexer, Peekable<CharIter>)) -> Tokenizer<CharIter> {
        Tokenizer {
            tokens: tokenizer.0.tokens,
            errors: tokenizer.0.errors,

            state: State::default(),
            return_state: None,
            source: tokenizer.1,

            current_token: None,
        }
    }
}
