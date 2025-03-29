use std::str::FromStr;

mod parser;
mod skip_spaces;
mod tag;

use super::{Document, ElementType};

#[derive(Debug, PartialEq, Eq)]
pub enum DOMParseError {
    #[allow(dead_code)]
    // NOTE(UB): We currently read tags as soon as we see an opening
    // tag, and if the file ends preemptively, we ignore that we
    // did not see a closing tag, just because it is a hassle to
    // deal with.
    UnclosedTag,

    EmptyDocument,
    TagUnclosed,
    UnexpectedClosingTag,
    UnexpectedSymbol,
    MissingOpeningTag,
}

use DOMParseError::*;

impl FromStr for Document {
    type Err = DOMParseError;
    fn from_str(document: &str) -> Result<Self, Self::Err> {
        match Document::parse(&mut document.chars().peekable())? {
            Some(e) => Ok(Document {
                doctype: None,
                html: e,
            }),
            // NOTE(UB): What should happen when there are no elements?
            None => Err(EmptyDocument),
        }
    }
}
