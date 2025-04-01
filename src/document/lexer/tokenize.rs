use std::{iter::Peekable, str::Chars};

use super::{Err, State, Token};

pub struct Tokenizer<'doc> {
    // NOTE: I realize that having to duplicate their definitions
    // might cause issues later on, in keeping them synchronized,
    // but I prefer it to having to type `foo.inner.bar`.
    pub tokens: Vec<Token>,
    pub errors: Vec<Err>,

    pub state: State,
    pub return_state: Option<State>,
    pub source: Peekable<Chars<'doc>>,
}

pub(super) fn tokenize(
    state: &mut State,
    return_state: &mut Option<State>,
    errs: &mut Vec<Err>,
    iter: &mut Peekable<impl Iterator<Item = char>>,
) -> Option<Token> {
    match state {
        // 13.2.5.1 Data state
        State::Data => {
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
                    *state = State::TagOpen;
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
        State::RcData => {
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
        _ => todo!("{state:?} has not been implemented."),
    }

    None
}
