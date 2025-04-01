use std::{fs::File, io::Read};

mod lexer;

use lexer::Lexer;

#[derive(Debug)]
pub struct Document(Lexer);

impl Document {
    pub fn new(mut location: File) -> Result<Self, String> {
        // NOTE(crash): This is intentionally left to crash as
        // this is only a temporary file reading mechanism, it
        // will be changed in the future.
        let mut buf = String::new();
        location.read_to_string(&mut buf).unwrap();
        let l: Lexer = buf.parse().unwrap();

        Ok(Self(l))
    }
}
