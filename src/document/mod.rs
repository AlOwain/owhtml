#![allow(dead_code)]

use std::{fs::File, io::Read, str::FromStr};

mod parse;

#[derive(Debug, PartialEq, Eq)]
enum ElementType {
    Root,
    Body,
    Bold(String),
    Text(String),
    Div,
}

impl FromStr for ElementType {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.into() {
            "body" => Ok(ElementType::Body),
            "div" => Ok(ElementType::Div),
            "b" => Ok(ElementType::Bold(String::new())),
            _ => Err(format!("Element tag '{string}' has not been implemented.")),
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
    pub fn new(mut location: File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = String::new();
        location.read_to_string(&mut buf)?;
        let dom = buf.parse()?;
        println!("{dom:#?}");
        Ok(dom)
    }
}
