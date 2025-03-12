use std::{iter::Peekable, str::FromStr};

use super::{Document, Element, ElementType};

impl Document {
    fn parse_tag(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<(bool, ElementType, String), String> {
        let mut attr = String::new();
        let closing = match iter.peek() {
            Some(c) => *c == '/',
            None => return Err("Closing tag unannotated.".to_string()),
        };
        if closing {
            iter.next().unwrap();
        }

        let mut tag: Option<ElementType> = None;

        while let Some(letter) = iter.next() {
            match letter {
                '>' => break,
                ' ' if tag.is_none() => {
                    tag = Some(ElementType::from_str(attr.as_str())?);
                    attr.clear();
                }
                _ => attr.push(letter),
            }
        }

        let tag = tag.unwrap_or(ElementType::from_str(attr.as_str())?);
        Ok((closing, tag, attr))
    }

    fn parse_handler(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<Option<Element>, String> {
        let mut ctx: Option<Element> = None;
        while let Some(letter) = iter.peek() {
            match (letter, &mut ctx) {
                ('<', Some(val)) => {
                    if let Some(child) = Self::parse_handler(iter)? {
                        val.children.push(child);
                    } else {
                        break;
                    }
                }
                ('<', None) => {
                    iter.next();
                    let (closing, tagtype, attr) = Self::parse_tag(iter)?;
                    if closing {
                        break;
                    }

                    ctx = Some(Element {
                        r#type: tagtype,
                        attr,
                        ..Default::default()
                    });
                }
                ('>', _) => return Err("Unexpected closing tag".to_string()),
                ('\\', _) => todo!("Escape sequences have not been implemented."),
                (c, None) if c.is_whitespace() => {
                    iter.next();
                }
                (_, None) => {
                    ctx = Some(Element::default());
                    iter.next();
                }
                (c, Some(val)) => {
                    match &mut val.r#type {
                        ElementType::Text(inner) => inner.push(*c),
                        ElementType::Bold(inner) => inner.push(*c),
                        _ => (),
                    };
                    iter.next();
                }
            }
        }

        Ok(ctx)
    }
}

impl FromStr for Document {
    type Err = String;
    fn from_str(document: &str) -> Result<Self, Self::Err> {
        let root = match Document::parse_handler(&mut document.chars().peekable())? {
            Some(e) => {
                if e.r#type == ElementType::Root {
                    e
                } else {
                    Element {
                        r#type: ElementType::Root,
                        children: vec![e],
                        ..Default::default()
                    }
                }
            }
            None => Element {
                r#type: ElementType::Root,
                ..Default::default()
            },
        };

        Ok(Document {
            doctype: None,
            html: root,
        })
    }
}
