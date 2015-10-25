use read_token::ReadToken;
use range::Range;
use std::sync::Arc;

use super::{
    ParseResult,
};
use {
    DebugId,
    MetaData,
    ParseError,
};
use tokenizer::{ read_data, TokenizerState };

/// Stores information about text.
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    /// Whether to allow empty string.
    pub allow_empty: bool,
    /// Which property to set if text is read.
    pub property: Option<Arc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Text {
    /// Parses text.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken
    ) -> ParseResult<TokenizerState> {
        if let Some(range) = read_token.string() {
            if !self.allow_empty && range.length == 2 {
                Err(range.wrap(ParseError::EmptyTextNotAllowed(self.debug_id)))
            } else {
                match read_token.parse_string(range.length) {
                    // Focus range to invalid string format.
                    Err(range_err) => {
                        Err(range_err.map(|err|
                            ParseError::ParseStringError(err, self.debug_id)))
                    }
                    Ok(text) => {
                        if let Some(ref property) = self.property {
                            Ok((range, read_data(
                                tokens,
                                range.wrap(MetaData::String(property.clone(),
                                    Arc::new(text))),
                                state
                            ), None))
                        } else {
                            Ok((range, state.clone(), None))
                        }
                    }
                }
            }
        } else {
            Err(read_token.start().wrap(
                ParseError::ExpectedText(self.debug_id)))
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::Text;
    use range::Range;
    use read_token::ReadToken;
    use std::sync::Arc;

    #[test]
    fn expected_text() {
        let text = "23";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let rule = Text {
            debug_id: 0,
            allow_empty: true,
            property: None
        };
        let res = rule.parse(&mut tokens, &s, &ReadToken::new(&text, 0));
        assert_eq!(res, Err(Range::new(0, 0).wrap(ParseError::ExpectedText(0))));
    }

    #[test]
    fn empty_string() {
        let text = "\"\"";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let rule = Text {
            debug_id: 0,
            allow_empty: false,
            property: None
        };
        let res = rule.parse(&mut tokens, &s, &ReadToken::new(&text, 0));
        assert_eq!(res, Err(Range::new(0, 2).wrap(
            ParseError::EmptyTextNotAllowed(0))));
    }

    #[test]
    fn successful() {
        let text = "foo \"hello\"";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let foo: Arc<String> = Arc::new("foo".into());
        let rule = Text {
            debug_id: 0,
            allow_empty: true,
            property: Some(foo.clone())
        };
        let res = rule.parse(&mut tokens, &s, &ReadToken::new(&text[4..], 4));
        assert_eq!(res, Ok((Range::new(4, 7), TokenizerState(1), None)));
        assert_eq!(tokens.len(), 1);
        assert_eq!(&tokens[0].data,
            &MetaData::String(foo.clone(), Arc::new("hello".into())));
    }
}
