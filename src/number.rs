use range::Range;
use read_token;
use std::rc::Rc;

use {
    DebugId,
    MetaData,
    ParseError,
    ParseResult,
    Tokenizer,
    TokenizerState,
};

/// Contains information about number.
pub struct Number {
    /// The property to set.
    pub property: Option<Rc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
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
        if let Some(range) = read_token::number(chars, offset) {
            let mut text = String::with_capacity(range.length);
            for c in chars.iter().take(range.length) {
                text.push(*c);
            }
            match text.parse::<f64>() {
                Err(err) => Err((range,
                    ParseError::ParseFloatError(err, self.debug_id))),
                Ok(val) => {
                    if let Some(ref property) = self.property {
                        match tokenizer.data(
                            MetaData::F64(property.clone(), val),
                            state,
                            range
                        ) {
                            Err(err) => Err((range, err)),
                            Ok(state) => Ok((range, state, None)),
                        }
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
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn expected_number() {
        let text = "foo";
        let chars: Vec<char> = text.chars().collect();
        let number = Number { debug_id: 0, property: None };
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = number.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((Range::new(0, 0), ParseError::ExpectedNumber(0))));
    }

    #[test]
    fn successful() {
        let text = "foo 1 1.1 10e1 10.0E1";
        let chars: Vec<char> = text.chars().collect();
        let number = Number { debug_id: 0, property: None };
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = number.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Ok((Range::new(4, 1), s, None)));
        let res = number.parse(&mut tokenizer, &s, &chars[6..], 6);
        assert_eq!(res, Ok((Range::new(6, 3), s, None)));
        let res = number.parse(&mut tokenizer, &s, &chars[10..], 10);
        assert_eq!(res, Ok((Range::new(10, 4), s, None)));

        let val: Rc<String> = Rc::new("val".into());
        let number = Number { debug_id: 0, property: Some(val.clone()) };
        let res = number.parse(&mut tokenizer, &s, &chars[15..], 15);
        assert_eq!(res, Ok((Range::new(15, 6), TokenizerState(1), None)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].0, &MetaData::F64(val.clone(), 10.0e1));
    }
}
