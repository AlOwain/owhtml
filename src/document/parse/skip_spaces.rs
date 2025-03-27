use std::iter::Peekable;

// TODO: This should be tested properly.
pub(super) fn skip_spaces(iter: &mut Peekable<impl Iterator<Item = char>>) {
    while let Some(c) = iter.peek() {
        if !c.is_whitespace() {
            break;
        }
        iter.next();
    }
}
