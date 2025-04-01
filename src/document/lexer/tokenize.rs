use std::iter::Peekable;

use super::{Err, State, Token};

pub(super) struct Tokenizer<CharIter: Iterator<Item = char>> {
    // NOTE: I realize that having to duplicate their definitions
    // might cause issues later on, in keeping them synchronized,
    // but I prefer it to having to type `foo.inner.bar`.
    pub tokens: Vec<Token>,
    pub errors: Vec<Err>,

    pub state: State,
    pub return_state: Option<State>,
    pub source: Peekable<CharIter>,
}

impl<CharIter: Iterator<Item = char>> Iterator for Tokenizer<CharIter> {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        match self.state {
            // 13.2.5.1 Data state
            State::Data => {
                // Consume the next input character:
                let letter = match self.source.next() {
                    Some(letter) => letter,
                    // Emit an end-of-file token.
                    None => {
                        self.tokens.push(Token::EOF);
                        return None;
                    }
                };

                match letter {
                    '&' => {
                        // Set the return state to the data state. Switch to the character reference state.
                        assert!(self.return_state.is_none());
                        self.return_state = Some(State::Data);

                        self.state = State::CharacterReference;
                    }
                    '<' => {
                        // Switch to the tag open state.
                        self.state = State::TagOpen;
                    }
                    '\0' => {
                        // This is an `unexpected-null-character` parse error.
                        self.errors.push(Err::UnexpectedNullCharacter);

                        // Emit the current input character as a character token.
                        unimplemented!("This should emit the current input character as a character token. But it is a null character.");

                        // FIX: What should we do here? What do other parsers do?
                        #[allow(unreachable_code)]
                        self.tokens.push(Token::Character('\0'));

                        return None;
                    }
                    input_char => {
                        // Emit the current input character as a character token.
                        self.tokens.push(Token::Character(input_char));
                        return Some(());
                    }
                }
            }

            // 13.2.5.2 RCDATA state
            State::RcData => {
                // Consume the next input character:
                let letter = match self.source.next() {
                    Some(letter) => letter,
                    None => {
                        // Emit an end-of-file token.
                        self.tokens.push(Token::EOF);
                        return Some(());
                    }
                };

                match letter {
                    '&' => {
                        // Set the return state to the RCDATA state. Switch to the character reference state.
                        self.return_state = Some(State::RcData);
                        self.state = State::CharacterReference;
                    }
                    '<' => {
                        // Switch to the RCDATA less-than sign state.
                        self.state = State::RcDataLessThan;
                    }
                    '\0' => {
                        // This is an `unexpected-null-character` parse error.
                        self.errors.push(Err::UnexpectedNullCharacter);

                        // Emit a `U+FFFD (REPLACEMENT CHARACTER)` character token.
                        self.tokens.push(Token::Character('\u{fffd}'));
                        return Some(());
                    }
                    input_char => {
                        // Emit the current input character as a character token.
                        self.tokens.push(Token::Character(input_char));
                        return Some(());
                    }
                }
            }

            // 13.2.5.6 Tag open state
            State::TagOpen => {
                // Consume the next input character:
                let letter = match self.source.peek() {
                    Some(letter) => letter,
                    None => {
                        // This is an `eof-before-tag-name` parse error.
                        self.errors.push(Err::EOFBeforeTagName);

                        // Emit a `U+003C (LESS-THAN SIGN)` character token and an `end-of-file` token.
                        todo!("Emit an EOF token.");
                        #[allow(unreachable_code)]
                        self.tokens.push(Token::Character('<'));
                        return Some(());
                    }
                };

                match letter {
                    '!' => {
                        // Switch to the markup declaration open state.
                        self.state = State::MarkupDeclarationOpen;
                    }
                    '/' => {
                        // Switch to the end tag open state.
                        self.state = State::EndTagOpen;
                    }
                    'a'..'z' | 'A'..'Z' => {
                        use super::token::TagInner;

                        // Reconsume in the tag name state.
                        self.state = State::TagName;

                        // Create a new start tag token, set its tag name to the empty string.
                        self.tokens.push(Token::StartTag(TagInner::default()));
                        return Some(());
                    }
                    '?' => {
                        // This is an `unexpected-question-mark-instead-of-tag-name` parse error.
                        self.errors
                            .push(Err::UnexpectedQuestionMarkInsteadOfTagName);

                        // Reconsume in the bogus comment state.
                        self.state = State::BogusComment;

                        // Create a comment token whose data is the empty string.
                        self.tokens.push(Token::Comment(String::new()));
                        return Some(());
                    }
                    _ => {
                        // This is an `invalid-first-character-of-tag-name` parse error.
                        self.errors.push(Err::InvalidFirstCharacterOfTagName);

                        // Reconsume in the data state.
                        self.state = State::Data;

                        // Emit a `U+003C (LESS-THAN SIGN)` character token.
                        self.tokens.push(Token::Character('<'));
                        return Some(());
                    }
                }
            }
            _ => todo!("{:?} has not been implemented.", self.state),
        }
        Some(())
    }
}
