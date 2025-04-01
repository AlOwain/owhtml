use std::{fs::File, io::Read};

mod lexer;

use lexer::Lexer;

#[derive(Debug)]
pub struct Document(Lexer);

impl Document {
    pub fn new(mut location: File) -> Result<Self, ()> {
        let mut buf = String::new();

        // NOTE(crash): This is intentionally left to crash as
        // this is only a temporary file reading mechanism, it
        // will be changed in the future.
        location.read_to_string(&mut buf).unwrap();

        // NOTE(crash): This will never return Result::Err as the
        // error type is unit `()`.
        Ok(Self(buf.parse().unwrap()))
    }
}
