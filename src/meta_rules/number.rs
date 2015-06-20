use range::Range;
use read_token;
use std::rc::Rc;

use super::{
    ParseResult,
};
use {
    DebugId,
    MetaData,
    ParseError,
    Tokenizer,
    TokenizerState,
};

/// Contains information about number.
#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    /// The property to set.
    pub property: Option<Rc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
    /// Whether underscore is allowed as visible separator.
    pub allow_underscore: bool,
}

impl Number {
    /// Parses number.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize
    ) -> ParseResult<TokenizerState> {
        let settings = read_token::NumberSettings {
            allow_underscore: self.allow_underscore
        };
        if let Some(range) = read_token::number(&settings, chars, offset) {
            match read_token::parse_number(&settings, &chars[..range.length]) {
                Err(err) => Err((range,
                    ParseError::ParseNumberError(err, self.debug_id))),
                Ok(val) => {
                    if let Some(ref property) = self.property {
                        Ok((range, tokenizer.data(
                            MetaData::F64(property.clone(), val),
                            state,
                            range
                        ), None))
                    } else {
                        Ok((range, state.clone(), None))
                    }
                }
            }
        } else {
            return Err((Range::new(offset, 0),
                ParseError::ExpectedNumber(self.debug_id)))
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use meta_rules::{ Number };
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn expected_number() {
        let text = "foo";
        let chars: Vec<char> = text.chars().collect();
        let number = Number {
            debug_id: 0,
            property: None,
            allow_underscore: false,
        };
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = number.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((Range::new(0, 0), ParseError::ExpectedNumber(0))));
    }

    #[test]
    fn successful() {
        let text = "foo 1 1.1 10e1 10.0E1 10_000";
        let chars: Vec<char> = text.chars().collect();
        let number = Number {
            debug_id: 0,
            property: None,
            allow_underscore: true,
        };
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = number.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Ok((Range::new(4, 1), s, None)));
        let res = number.parse(&mut tokenizer, &s, &chars[6..], 6);
        assert_eq!(res, Ok((Range::new(6, 3), s, None)));
        let res = number.parse(&mut tokenizer, &s, &chars[10..], 10);
        assert_eq!(res, Ok((Range::new(10, 4), s, None)));
        let res = number.parse(&mut tokenizer, &s, &chars[22..], 22);
        assert_eq!(res, Ok((Range::new(22, 6), s, None)));

        let val: Rc<String> = Rc::new("val".into());
        let number = Number {
            debug_id: 0,
            property: Some(val.clone()),
            allow_underscore: false,
        };
        let res = number.parse(&mut tokenizer, &s, &chars[15..], 15);
        assert_eq!(res, Ok((Range::new(15, 6), TokenizerState(1), None)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].1, &MetaData::F64(val.clone(), 10.0e1));
    }
}
