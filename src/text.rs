use read_token;
use range::Range;
use std::rc::Rc;

use {
    DebugId,
    MetaData,
    ParseError,
    ParseResult,
    Tokenizer,
    TokenizerState,
};

/// Stores information about text.
#[derive(Clone)]
pub struct Text {
    /// Whether to allow empty string.
    pub allow_empty: bool,
    /// Which property to set if text is read.
    pub property: Option<Rc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Text {
    /// Parses text.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize
    ) -> ParseResult<TokenizerState> {
        if let Some(range) = read_token::string(chars, offset) {
            if !self.allow_empty && range.length == 2 {
                Err((range, ParseError::EmptyTextNotAllowed(self.debug_id)))
            } else {
                match read_token::parse_string(
                    chars, offset, range.next_offset()) {
                    // Focus range to invalid string format.
                    Err(err) => Err((err.range(),
                        ParseError::ParseStringError(err, self.debug_id))),
                    Ok(text) => {
                        if let Some(ref property) = self.property {
                            Ok((range, tokenizer.data(
                                MetaData::String(property.clone(), text),
                                state,
                                range
                            ), None))
                        } else {
                            Ok((range, state.clone(), None))
                        }
                    }
                }
            }
        } else {
            Err((Range::new(offset, 0),
                ParseError::ExpectedText(self.debug_id)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn expected_text() {
        let text = "23";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let text = Text {
            debug_id: 0,
            allow_empty: true,
            property: None
        };
        let res = text.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((Range::new(0, 0), ParseError::ExpectedText(0))));
    }

    #[test]
    fn empty_string() {
        let text = "\"\"";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let text = Text {
            debug_id: 0,
            allow_empty: false,
            property: None
        };
        let res = text.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((Range::new(0, 2),
            ParseError::EmptyTextNotAllowed(0))));
    }

    #[test]
    fn successful() {
        let text = "foo \"hello\"";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let foo: Rc<String> = Rc::new("foo".into());
        let text = Text {
            debug_id: 0,
            allow_empty: true,
            property: Some(foo.clone())
        };
        let res = text.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Ok((Range::new(4, 7), TokenizerState(1), None)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].0,
            &MetaData::String(foo.clone(), "hello".into()));
    }
}
