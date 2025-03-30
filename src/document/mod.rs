use std::{fs::File, io::Read, str::FromStr};

mod lexer;

#[derive(Debug)]
pub struct Document {
    // NOTE: Should this be an `Option` or an empty
    // `String`, or can the doctype always be inferred.
    doctype: Option<String>,
}
impl Document {
    pub fn new(mut location: File) -> Result<Self, String> {
        // NOTE(crash): This is intentionally left to crash as:
        // 1. This is only a _temporary_ file reading mechanism,
        //   I can imagine it being changed in the future;
        // 2. I am fine with crashing for the price of simplicity, and;
        // 3. The program prefers errors to make it exit.
        let mut buf = String::new();
        location.read_to_string(&mut buf).unwrap();

        Ok(Self { doctype: None })
    }
}
