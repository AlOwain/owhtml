#![allow(dead_code)]

use std::{fs::File, io::Read, str::FromStr};

mod parse;
mod test;
use parse::DOMParseError;

#[derive(Debug, PartialEq, Eq)]
enum ElementType {
    Root,
    Body,
    Bold(String),
    Text(String),
    Div,
}

impl FromStr for ElementType {
    type Err = DOMParseError;

    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        match tag {
            "body" => Ok(ElementType::Body),
            "div" => Ok(ElementType::Div),
            "b" => Ok(ElementType::Bold(String::new())),
            _ => todo!("Element tag '{tag}' has not been implemented."),
        }
    }
}

#[derive(Debug)]
struct SelectorType {
    r#type: ElementType,
    attr: String,
}

#[derive(Debug)]
struct Element {
    r#type: ElementType,
    children: Vec<Element>,
    attr: String,
}

impl Default for Element {
    fn default() -> Self {
        Element {
            r#type: ElementType::Text(String::new()),
            children: vec![],
            attr: "".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct Document {
    doctype: Option<String>,
    html: Element,
}
impl Document {
    pub fn new(mut location: File) -> Result<Self, DOMParseError> {
        // NOTE(crash): This is intentionally left undealt with as
        // 1. This is only a _temporary_ file reading mechanism,
        //     I can imagine it being changed in the future.
        // 2. I am fine with crashing for the price of simplicity.
        // 3. This must not be user-facing!
        let mut buf = String::new();
        location.read_to_string(&mut buf).unwrap();

        Ok(buf.parse()?)
    }
}
