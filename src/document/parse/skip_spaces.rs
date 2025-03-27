use std::iter::Peekable;

pub(super) fn skip_spaces(iter: &mut Peekable<impl Iterator<Item = char>>) {
    while let Some(c) = iter.peek() {
        if !c.is_whitespace() {
            break;
        }
        iter.next();
    }
}

mod test {
    #![allow(unused_macros, unused_imports)]
    use std::{iter::Peekable, str::Chars};

    macro_rules! call_skip_spaces {
        ($str: literal, $iterate_count: literal, $result: expr) => {
            let mut i: Peekable<Chars> = $str.chars().peekable();
            // TODO(nightly): Use Iterator::advance_by when stable.
            let _ = i.nth($iterate_count - 1).unwrap();
            super::skip_spaces(&mut i);
            assert_eq!(i.peek(), $result);
        };

        ($str: literal, $result: expr) => {
            let mut i: Peekable<Chars> = $str.chars().peekable();
            super::skip_spaces(&mut i);
            assert_eq!(i.peek(), $result);
        };
    }

    #[test]
    #[rustfmt::skip]
    pub fn test_skip_spaces() {
        call_skip_spaces!("",                    None);
        call_skip_spaces!("O X",                 Some(&'O'));
        call_skip_spaces!(" O X",                Some(&'O'));
        call_skip_spaces!("  O X",               Some(&'O'));
        call_skip_spaces!("\nO X",               Some(&'O'));
        call_skip_spaces!(" \n\t\nO X",          Some(&'O'));
        call_skip_spaces!("X  ",           1,    None);
        call_skip_spaces!("X\n\t ",        1,    None);
        call_skip_spaces!("X\n\tO",        3,    Some(&'O'));
    }
}
