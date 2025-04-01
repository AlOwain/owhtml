mod error;
mod state;
mod token;
mod tokenize;

pub use {error::Err, state::State, token::Token};

pub fn lexer(document: String) -> Vec<Token> {
    let mut iter = document.chars().peekable();
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut state = Default::default();
    loop {
        let token = tokenize::tokenize(&mut state, &mut errors, &mut iter);
        match token {
            Some(Token::EOF) => break,
            Some(token) => tokens.push(token),
            None => continue,
        }
    }
    return tokens;
}
