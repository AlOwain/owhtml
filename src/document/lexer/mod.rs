use std::str::FromStr;

mod error;
mod state;
mod token;
mod tokenize;

pub use {error::Err, state::State, token::Token};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    errors: Vec<Err>,
}

impl FromStr for Lexer {
    type Err = Err;

    fn from_str(document: &str) -> Result<Self, Self::Err> {
        let mut iter = document.chars().peekable();
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut state = Default::default();
        let mut return_state = None;

        loop {
            let token = tokenize::tokenize(&mut state, &mut return_state, &mut errors, &mut iter);
            match token {
                Some(Token::EOF) => break,
                Some(token) => tokens.push(token),
                None => continue,
            }
        }

        Ok(Lexer { tokens, errors })
    }
}
