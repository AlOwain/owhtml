use std::str::FromStr;

mod error;
mod state;
mod token;
mod tokenize;

pub use {error::Err, state::State, token::Token};

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub errors: Vec<Err>,
}

impl FromStr for Lexer {
    type Err = ();

    fn from_str(document: &str) -> Result<Self, ()> {
        let mut iter = document.chars().peekable();
        let mut state = Default::default();
        let mut return_state = None;
        let mut lexer = Lexer {
            tokens: Vec::new(),
            errors: Vec::new(),
        };

        loop {
            let token =
                tokenize::tokenize(&mut state, &mut return_state, &mut lexer.errors, &mut iter);
            match token {
                Some(Token::EOF) => break,
                Some(token) => lexer.tokens.push(token),
                None => continue,
            }
        }

        Ok(lexer)
    }
}
