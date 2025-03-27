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
