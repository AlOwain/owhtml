use std::{iter::Peekable, str::FromStr};

use super::{Document, Element, ElementType};

#[derive(Debug, PartialEq, Eq)]
pub enum DOMParseError {
    EmptyDocument,
    TagUnclosed,
    UnexpectedClosingTag,
    UnclosedTag,
    UnexpectedChild,
}

use DOMParseError::*;

impl Document {
    // DESCRIPTION: parses the contents of an HTML tag beginning by the first
    // letter of an iterator, the iterator should not start with a '<' symbol
    // as the caller is expected to consume that.
    //
    // If the tag is a closing tag, attributes are ignored.
    fn parse_tag(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<(bool, ElementType, String), DOMParseError> {
        let mut tag_name = String::new();
        let mut attr = String::new();
        let closing = match iter.peek() {
            Some(c) => *c == '/',
            None => return Err(TagUnclosed),
        };

        if closing {
            iter.next().unwrap(); // If it is closing, skip the '/'
        }

        let mut tag: Option<ElementType> = None;
        while let Some(letter) = iter.next() {
            match letter {
                '>' => break,
                ' ' if tag.is_none() => {
                    tag = Some(ElementType::from_str(tag_name.as_str())?);
                }

                // NOTE: This obviously can be rewritten and made more
                //     concise, by using the same variable for both the
                //     attributes and tag name, but it is deliberately
                //     left as such to be clearer.
                _ if tag.is_none() => tag_name.push(letter),
                // NOTE: Closing tags have no attributes.
                _ if tag.is_some() && !closing => attr.push(letter),
                _ => (),
            }
        }

        let tag = tag.unwrap_or(ElementType::from_str(&tag_name)?);
        Ok((closing, tag, attr))
    }

    fn parse_handler(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<Option<Element>, DOMParseError> {
        let mut ctx: Option<Element> = None;
        while let Some(letter) = iter.peek() {
            match (letter, &mut ctx) {
                ('<', Some(val)) => {
                    // NOTE: Returning here is because in text element cannot have children,
                    //     having it return would allow the parent of the text element to
                    //     parse the next element which would make it the text's sibling;
                    //     if we continue then the text will parse it as a child as other element
                    //     types would. For example:
                    //             <body> Hello <div> Twitch </div> world </body>
                    //     would generate the following AST
                    //                                   body
                    //                                   ┌┼┐
                    //                                p ◄┘│└► p
                    //                                   div
                    //     instead of
                    //                                   body
                    //                                    ↓
                    //                                    p
                    //                                    ↓
                    //                                   div
                    // NOTE(validity): Of course this creates an issue when a text element does
                    //     not have a parent. But that should never happen in valid HTML.
                    if let ElementType::Text(_) = val.r#type {
                        return Ok(ctx);
                    }

                    if let Some(child) = Self::parse_handler(iter)? {
                        val.children.push(child);
                    } else {
                        // NOTE: This breaks here because the `parse_handler` call above
                        // consumes the iterator till the end of the tag, and the
                        // resulting iterator parsed items returned `None`.
                        //
                        // Which only happens when the tag is a closing tag, which means
                        // this should return the parsed `ctx`.
                        break;
                    }
                }
                ('<', None) => {
                    iter.next();
                    let (closing, tagtype, attr) = Self::parse_tag(iter)?;
                    if closing {
                        return Ok(None);
                    }

                    ctx = Some(Element {
                        r#type: tagtype,
                        attr,
                        ..Default::default()
                    });
                }

                // TODO: This error needs test cases.
                ('>', None) => return Err(UnexpectedClosingTag),
                ('\\', _) => todo!("Escape sequences have not been implemented."),

                (c, None) if c.is_whitespace() => {
                    iter.next();
                }
                (_, None) => {
                    ctx = Some(Element {
                        r#type: ElementType::Text(1),
                        ..Default::default()
                    });
                    iter.next();
                }
                (_, Some(val)) => {
                    match &mut val.r#type {
                        ElementType::Text(inner) => {
                            *inner += 1;
                            assert!(val.children.len() == 0);
                            assert!(val.attr.is_empty());
                        }
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
    type Err = DOMParseError;
    fn from_str(document: &str) -> Result<Self, Self::Err> {
        match Document::parse_handler(&mut document.chars().peekable())? {
            Some(e) => Ok(Document {
                doctype: None,
                html: e,
            }),
            // FIX(UB): What should happen when there are no elements?
            None => Err(EmptyDocument),
        }
    }
}
