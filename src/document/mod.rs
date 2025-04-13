use std::{fs::File, io::Read};

mod parser;

use parser::Lexer;
pub use parser::Token;

#[derive(Debug)]
pub struct Document;

impl Document {
    pub fn new(mut location: File) -> Result<(Vec<Token>, Vec<parser::Err>), ()> {
        let mut buf = String::new();

        // NOTE(crash): This is intentionally left to crash as
        // this is only a temporary file reading mechanism, it
        // will be changed in the future.
        location.read_to_string(&mut buf).unwrap();

        // NOTE(crash): This will never return Result::Err as the
        // error type is unit `()`.
        let parser_result = buf.parse::<Lexer>().unwrap();
        Ok((parser_result.tokens, parser_result.errors))
    }
}
