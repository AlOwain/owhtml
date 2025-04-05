use std::iter::Peekable;

use super::{token::TagInner, Err, State, Token};

pub(super) struct Tokenizer<CharIter: Iterator<Item = char>> {
    // NOTE: I realize that having to duplicate their definitions
    // might cause issues later on, in keeping them synchronized,
    // but I prefer it to having to type `foo.inner.bar`.
    pub tokens: Vec<Token>,
    pub errors: Vec<Err>,

    pub state: State,
    pub return_state: Option<State>,
    pub source: Peekable<CharIter>,

    pub current_token: Option<Token>,
}

impl<CharIter: Iterator<Item = char>> Iterator for Tokenizer<CharIter> {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        match self.state {
            // 13.2.5.1 Data state
            State::Data => {
                // NOTE(spec): Consume the next input character:
                let letter = match self.source.next() {
                    Some(letter) => letter,
                    // NOTE(spec): Emit an end-of-file token.
                    None => {
                        self.tokens.push(Token::EOF);
                        return None;
                    }
                };

                match letter {
                    '&' => {
                        // NOTE(spec): Set the return state to the data state. Switch to the character reference state.
                        assert!(self.return_state.is_none());
                        self.return_state = Some(State::Data);

                        self.state = State::CharacterReference;
                    }
                    '<' => {
                        // NOTE(spec): Switch to the tag open state.
                        self.state = State::TagOpen;
                    }
                    '\0' => {
                        // NOTE(spec): This is an `unexpected-null-character` parse error.
                        self.errors.push(Err::UnexpectedNullCharacter);

                        // NOTE(spec): Emit the current input character as a character token.
                        unimplemented!("This should emit the current input character as a character token. But it is a null character.");

                        // FIX: What should we do here? What do other parsers do?
                        #[allow(unreachable_code)]
                        self.tokens.push(Token::Character('\0'));

                        return None;
                    }
                    input_char => {
                        // NOTE(spec): Emit the current input character as a character token.
                        self.tokens.push(Token::Character(input_char));
                        return Some(());
                    }
                }
            }

            // 13.2.5.2 RCDATA state
            State::RcData => {
                // NOTE(spec): Consume the next input character:
                let letter = match self.source.next() {
                    Some(letter) => letter,
                    None => {
                        // NOTE(spec): Emit an end-of-file token.
                        self.tokens.push(Token::EOF);
                        return None;
                    }
                };

                match letter {
                    '&' => {
                        // NOTE(spec): Set the return state to the RCDATA state. Switch to the character reference state.
                        self.return_state = Some(State::RcData);
                        self.state = State::CharacterReference;
                    }
                    '<' => {
                        // NOTE(spec): Switch to the RCDATA less-than sign state.
                        self.state = State::RcDataLessThan;
                    }
                    '\0' => {
                        // NOTE(spec): This is an `unexpected-null-character` parse error.
                        self.errors.push(Err::UnexpectedNullCharacter);

                        // NOTE(spec): Emit a `U+FFFD (REPLACEMENT CHARACTER)` character token.
                        self.tokens.push(Token::Character('\u{fffd}'));
                        return Some(());
                    }
                    input_char => {
                        // NOTE(spec): Emit the current input character as a character token.
                        self.tokens.push(Token::Character(input_char));
                        return Some(());
                    }
                }
            }

            // 13.2.5.6 Tag open state
            State::TagOpen => {
                // NOTE(spec): Consume the next input character:
                let letter = match self.source.peek() {
                    Some(letter) => letter,
                    None => {
                        // NOTE(spec): This is an `eof-before-tag-name` parse error.
                        self.errors.push(Err::EOFBeforeTagName);

                        // NOTE(spec): Emit a `U+003C (LESS-THAN SIGN)` character token
                        self.tokens.push(Token::Character('<'));

                        // NOTE(spec): Emit an `end-of-file` token.
                        self.tokens.push(Token::EOF);
                        return None;
                    }
                };

                match letter {
                    '!' => {
                        // NOTE(spec): Switch to the markup declaration open state.
                        self.state = State::MarkupDeclarationOpen;
                    }
                    '/' => {
                        // NOTE(spec): Switch to the end tag open state.
                        self.state = State::EndTagOpen;
                    }
                    'a'..'z' | 'A'..'Z' => {
                        // NOTE(spec): Reconsume in the tag name state.
                        self.state = State::TagName;

                        // NOTE(spec): Create a new start tag token, set its tag name to the empty string.
                        self.current_token = Some(Token::StartTag(TagInner::default()));
                        return Some(());
                    }
                    '?' => {
                        // NOTE(spec): This is an `unexpected-question-mark-instead-of-tag-name` parse error.
                        self.errors
                            .push(Err::UnexpectedQuestionMarkInsteadOfTagName);

                        // NOTE(spec): Reconsume in the bogus comment state.
                        self.state = State::BogusComment;

                        // NOTE(spec): Create a comment token whose data is the empty string.
                        self.current_token = Some(Token::Comment(String::new()));
                        return Some(());
                    }
                    _ => {
                        // NOTE(spec): This is an `invalid-first-character-of-tag-name` parse error.
                        self.errors.push(Err::InvalidFirstCharacterOfTagName);

                        // NOTE(spec): Reconsume in the data state.
                        self.state = State::Data;

                        // NOTE(spec): Emit a `U+003C (LESS-THAN SIGN)` character token.
                        self.tokens.push(Token::Character('<'));
                        return Some(());
                    }
                }
            }
            // 13.2.5.8 Tag name state
            State::TagName => {
                // NOTE(spec): Consume the next input character:
                let letter = match self.source.next() {
                    Some(letter) => letter,
                    None => {
                        // NOTE(spec): This is an `eof-in-tag` parse error.
                        self.errors.push(Err::EOFInTag);

                        // NOTE(spec): Emit an `end-of-file` token.
                        self.tokens.push(Token::EOF);
                        return None;
                    }
                };

                match letter {
                    // NOTE(spec): Tab or `new line` or `U+000C (Form Feed (FF))` or Space
                    _ if letter.is_whitespace() => {
                        // NOTE(spec): Switch to the before attribute name state.
                        self.state = State::BeforeAttributeName;
                    }
                    '/' => {
                        // NOTE(spec): Switch to the self-closing start tag state.
                        self.state = State::SelfClosingStartTag;
                    }
                    '>' => {
                        // NOTE(spec): Switch to the data state.
                        self.state = State::Data;

                        // NOTE(spec): Emit the current tag token.
                        self.tokens.push(
                            // NOTE(crash): The state is never set to `TagName` unless a tag is created afterwords.
                            self.current_token.take().unwrap(),
                        );
                    }

                    'A'..='Z' => {
                        // NOTE(spec): Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current tag token's tag name.
                        self.current_token
                            .as_mut()
                            // NOTE(crash): The state is never set to `TagName` unless a tag is created afterwords.
                            .unwrap()
                            .append_to_tagname(letter.to_ascii_lowercase())
                            // NOTE(crash): Appending to a `TagName` is infallible.
                            .unwrap();
                    }

                    '\0' => {
                        // NOTE(spec): This is an unexpected-null-character parse error.
                        self.errors.push(Err::UnexpectedNullCharacter);

                        // NOTE(spec): Append a `U+FFFD (REPLACEMENT CHARACTER)` character to the current tag token's tag name.
                        self.current_token
                            .as_mut()
                            // NOTE(crash): The state is never set to `TagName` unless a tag is created afterwords.
                            .unwrap()
                            // NOTE(crash): Appending to a `TagName` is infallible.
                            .append_to_tagname('\u{fffd}')
                            .unwrap();
                    }
                    _ => {
                        // NOTE(spec): Append the current input character to the current tag token's tag name.
                        self.current_token
                            .as_mut()
                            // NOTE(crash): The state is never set to `TagName` unless a tag is created afterwords.
                            .unwrap()
                            .append_to_tagname(letter)
                            // NOTE(crash): Appending to a `TagName` is infallible.
                            .unwrap();
                    }
                }
            }

            _ => todo!("{:?} has not been implemented.", self.state),
        }

        Some(())
    }
}
