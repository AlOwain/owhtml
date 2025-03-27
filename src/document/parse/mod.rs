use std::{iter::Peekable, str::FromStr};

mod skip_spaces;
mod tag;

use super::{Document, Element, ElementType};

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

impl Document {
    /*
    DESCRIPTION: This function will consume the iterator, parsing only one tag
        at a time, and using recursion to parse its children.
    RETURNS:
        Ok(_) => Ok(Some(_)) => An element, it will consume the iterator till the end
                                of the closing tag.
               | None        => When it doesn't read anything (or just a closing tag).
    */
    fn parse(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<Option<Element>, DOMParseError> {
        let mut ctx: Option<Element> = None;
        while let Some(letter) = iter.peek() {
            match (letter, &mut ctx) {
                /*
                FIX(validity): This deals with the case of improperly closed text nodes,
                            which we should either ignore, or terminate as an error.
                NOTE: Returning here is because in text element cannot have children,
                    having it return would allow the parent of the text element to parse
                    the next element which would make it the text's sibling; if we continue
                    then the text will parse it as a child as other element types would.

                For example:
                        <body> Hello <div> Twitch </div> world </body>
                would generate the following AST
                                            body
                                            ┌┼┐
                                         p ◄┘│└► p
                                            div
                instead of
                                            body
                                             ↓
                                             p
                                             ↓
                                            div
                NOTE(validity): Of course this creates an issue when a text element does
                    not have a parent. But that should never happen in "valid" HTML.
                TODO: You could check if the next character is a closing tag.
                */
                ('<', Some(val)) if matches!(val.r#type, ElementType::Text(_)) => return Ok(ctx),
                ('<', Some(val)) => {
                    if let Some(child) = Self::parse(iter)? {
                        val.children.push(child);
                    } else {
                        /*
                        NOTE: This breaks here because the `parse` call above
                        consumes the iterator till the end of the tag, and the
                        resulting iterator parsed items returned `None`.

                        Which only happens when the tag is a closing tag, which means
                        this should return the parsed `ctx`.
                        */
                        return Ok(None);
                    }
                }
                ('<', None) => {
                    let (closing, tagtype, attr) = Self::tag(iter)?;
                    if closing {
                        return Ok(None);
                    }

                    ctx = Some(Element {
                        r#type: tagtype,
                        attr,
                        ..Default::default()
                    });
                }

                // FIX: HTML does not ignore whitespace completely as we do,
                // it folds it, so HTML treats `/\n+/` as a single newline,
                // and `/[[:blank:]]+/` as a single space.
                (c, _) if c.is_whitespace() => {
                    iter.next();
                }
                // TODO: This error needs test cases.
                ('>', None) => return Err(UnexpectedClosingTag),
                ('\\', _) => todo!("Escape sequences have not been implemented."),

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

                            // FIX: If the character is a space, you should probably move
                            // the iterator till a non-whitespace character is found.
                            iter.next();
                        }

                        // FIX: Some error tests need to be written on this.

                        // NOTE(UB): What should happen here? This is called when:
                        // - `c` ∉ { '<', '\', ' ' }; which means c is most likely text,
                        // - `ctx` ∉ { None, `Text` }; `ctx` is most likely a `Container`.
                        _ => return Err(UnexpectedSymbol),
                    };
                }
            }
        }

        Ok(ctx)
    }
}

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
