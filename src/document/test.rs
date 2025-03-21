#![allow(unused_imports)]
use std::fs::File;

use super::{parse::DOMParseError::*, Document};

#[test]
pub fn unannotated_closing_tag() {
    let _ = match File::open("./resources/tests/opening_tag_at_EOF.html") {
        // NOTE(crash): This is just a test case!
        Ok(file) => Document::new(file).map_err(|err| assert_eq!(err, ClosingTagUnclosed)),
        Err(_) => unreachable!(),
    };
}
