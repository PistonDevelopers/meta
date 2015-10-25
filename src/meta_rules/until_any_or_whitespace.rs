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

/// Stores information about reading until whitespace or any of some character.
#[derive(Clone, Debug, PartialEq)]
pub struct UntilAnyOrWhitespace {
    /// The characters to stop at.
    pub any_characters: Arc<String>,
    /// Whether empty data is accepted or not.
    pub optional: bool,
    /// The property to store read text.
    pub property: Option<Arc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl UntilAnyOrWhitespace {
    /// Parses until whitespace or any specified characters.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken
    ) -> ParseResult<TokenizerState> {
        let (range, _) = read_token.until_any_or_whitespace(
            &self.any_characters);
        if range.length == 0 && !self.optional {
            Err(range.wrap(ParseError::ExpectedSomething(self.debug_id)))
        } else {
            if let Some(ref property) = self.property {
                let text = read_token.raw_string(range.length);
                Ok((range, read_data(
                    tokens,
                    range.wrap(
                        MetaData::String(property.clone(), Arc::new(text))),
                    state
                ), None))
            } else {
                Ok((range, state.clone(), None))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::UntilAnyOrWhitespace;
    use range::Range;
    use read_token::ReadToken;
    use std::sync::Arc;

    #[test]
    fn required() {
        let text = "fn ()";
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let name = UntilAnyOrWhitespace {
            debug_id: 0,
            any_characters: Arc::new("(".into()),
            optional: false,
            property: None
        };
        let res = name.parse(&mut tokenizer, &s,
            &ReadToken::new(&text[3..], 3));
        assert_eq!(res, Err(Range::new(3, 0).wrap(
            ParseError::ExpectedSomething(0))));
    }

    #[test]
    fn successful() {
        let text = "fn foo()";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let function_name: Arc<String> = Arc::new("function_name".into());
        let name = UntilAnyOrWhitespace {
            debug_id: 0,
            any_characters: Arc::new("(".into()),
            optional: false,
            property: Some(function_name.clone())
        };
        let res = name.parse(&mut tokens, &s, &ReadToken::new(&text[3..], 3));
        assert_eq!(res, Ok((Range::new(3, 3), TokenizerState(1), None)));
        assert_eq!(tokens.len(), 1);
        assert_eq!(&tokens[0].data,
            &MetaData::String(function_name.clone(), Arc::new("foo".into())));
    }
}
