use std::{iter::Peekable, str::FromStr};

use super::{
    skip_spaces::skip_spaces,
    DOMParseError::{self, *},
    ElementType,
};
use crate::Document;

impl Document {
    /*
    TODO: Develop test cases around this.

    DESCRIPTION: parses the contents of an HTML tag beginning by the first
    letter of an iterator, the iterator must start with a '<' symbol as
    the function expects to consume that, you could make it check for it
    to be optionally there, but do we really want that :).

    If the tag is a closing tag, attributes are ignored.
    */
    pub(super) fn tag(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<(bool, ElementType, String), DOMParseError> {
        if !matches!(iter.next(), Some('<')) {
            return Err(MissingOpeningTag);
        }

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
                ' ' => match tag {
                    Some(_) => {
                        skip_spaces(iter);
                    }
                    None => {
                        tag = Some(ElementType::from_str(tag_name.as_str())?);
                    }
                },

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
}

mod test {
    #![allow(unused_macros, unused_imports)]
    use std::{iter::Peekable, str::Chars};

    use super::{
        DOMParseError::*,
        Document,
        ElementType::{self, *},
    };

    macro_rules! call_tag_parser {
        ($str: literal, $result: expr, $stop_char: expr) => {
            let mut i: Peekable<Chars> = $str.chars().peekable();
            assert_eq!(
                Document::tag(&mut i),
                $result.map(|res: (bool, ElementType, &str)| (res.0, res.1, res.2.to_string()))
            );
            assert_eq!(i.peek().map(|c| c.clone()), $stop_char);
        };
    }

    #[test]
    #[rustfmt::skip]
    pub fn parsing_tag() {
        const F: bool = false;
        const T: bool = true;

        call_tag_parser!("b>",  Err(MissingOpeningTag), Some('>'));
        call_tag_parser!(" b>", Err(MissingOpeningTag), Some('b'));
        call_tag_parser!("<b>", Ok((F, Text(0), "")), None);
        call_tag_parser!("<div> ", Ok((F, Container, "")), Some(' '));
        call_tag_parser!("</div>X", Ok((T, Container, "")), Some('X'));
        call_tag_parser!("</div this will not be read>", Ok((T, Container, "")), None);
        // FIX: Failing test! As we currently ignore all whitespace.
        call_tag_parser!("<div this will be read>", Ok((F, Container, "this will be read")), None);
        // FIX: Failing test!
        //     1. As we currently ignore all whitespace.
        //     2. The whitespace we ignore always begins with a literal ' ' (U+0020).
        call_tag_parser!("<p this    will\tnot  have \t\t\n\t that  many\n\n\t whitespace>", Ok((F, Container, "this will not have that many whitespace")), None);

        // TODO: Add more tests
    }
}
