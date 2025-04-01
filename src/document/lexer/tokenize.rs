use std::iter::Peekable;

use super::{Err, State, Token};

pub(super) fn tokenize(
    state: &mut State,
    errs: &mut Vec<Err>,
    iter: &mut Peekable<impl Iterator<Item = char>>,
) -> Option<Token> {
    use State::*;
    match state {
        // 13.2.5.1 Data state
        Data => {
            // Consume the next input character:
            let letter = match iter.next() {
                Some(letter) => letter,
                // Emit an end-of-file token.
                None => return Some(Token::EOF),
            };

            match letter {
                '&' => {
                    todo!("Set the return state to the data state. Switch to the character reference state.");
                }
                '<' => {
                    // Switch to the tag open state.
                    *state = TagOpen;
                }
                '\0' => {
                    todo!("This is an unexpected-null-character parse error. Emit the current input character as a character token.");
                }
                _ => {
                    todo!("Emit the current input character as a character token.")
                }
            }
        }

        // 13.2.5.2 RCDATA state
        RcData => {
            // Consume the next input character:
            let letter = match iter.next() {
                Some(letter) => letter,
                // Emit an end-of-file token.
                None => return Some(Token::EOF),
            };

            match letter {
                '&' => {
                    todo!("Set the return state to the RCDATA state. Switch to the character reference state.");
                }
                '<' => {
                    todo!("Switch to the RCDATA less-than sign state.");
                }
                '\0' => {
                    todo!("This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.");
                }
                _ => {
                    todo!("Emit the current input character as a character token.")
                }
            }
        }
        _ => todo!("{state:?} has been implemented."),
    }
    None
}
