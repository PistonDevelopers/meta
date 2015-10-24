use range::Range;
use read_token::ReadToken;
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

/// Stores information about token.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The text to match against.
    pub text: Arc<String>,
    /// Whether to fail when matching against text.
    pub not: bool,
    /// Whether to set property to true or false (inverted).
    pub inverted: bool,
    /// Which property to set if token matches.
    pub property: Option<Arc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Token {
    /// Parses token.
    /// If the token is linked to a property,
    /// the property will be set.
    /// If the meta reader fails setting the property the error is handled.
    /// If the token is not linked to any property,
    /// the same state will be returned.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken
    ) -> ParseResult<TokenizerState> {
        if let Some(range) = read_token.tag(&self.text) {
            if self.not {
                Err(range.wrap(
                    ParseError::DidNotExpectToken(self.text.clone(),
                    self.debug_id)))
            } else {
                match &self.property {
                    &Some(ref name) => {
                        Ok((range, read_data(
                            tokens,
                            range.wrap(
                                MetaData::Bool(name.clone(), !self.inverted)),
                            &state
                        ), None))
                    }
                    _ => {
                        Ok((range, state.clone(), None))
                    }
                }
            }
        } else {
            if self.not {
                match &self.property {
                    &Some(ref name) => {
                        let range = read_token.start();
                        Ok((range, read_data(
                            tokens,
                            range.wrap(
                                MetaData::Bool(name.clone(), !self.inverted)),
                            &state
                        ), None))
                    }
                    _ => {
                        Ok((read_token.start(), state.clone(), None))
                    }
                }
            } else {
                Err(read_token.start().wrap(
                    ParseError::ExpectedToken(self.text.clone(),
                    self.debug_id)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::Token;
    use std::sync::Arc;
    use range::Range;
    use read_token::ReadToken;

    #[test]
    fn expected_token() {
        let text = ")";
        let chars: Vec<char> = text.chars().collect();
        let start_parenthesis = Token {
            debug_id: 0,
            text: Arc::new("(".into()),
            not: false,
            inverted: false,
            property: None
        };
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokens, &s,
            &ReadToken::new(&chars, 0));
        assert_eq!(res, Err(Range::new(0, 0).wrap(
            ParseError::ExpectedToken(Arc::new("(".into()), 0))));
    }

    #[test]
    fn did_not_expect_token() {
        let text = ")";
        let chars: Vec<char> = text.chars().collect();
        let start_parenthesis = Token {
            debug_id: 0,
            text: Arc::new(")".into()),
            not: true,
            inverted: false,
            property: None
        };
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokens, &s,
            &ReadToken::new(&chars, 0));
        assert_eq!(res, Err(Range::new(0, 1).wrap(
            ParseError::DidNotExpectToken(Arc::new(")".into()), 0))));
    }

    #[test]
    fn successful() {
        let text = "fn foo()";
        let chars: Vec<char> = text.chars().collect();
        let fn_ = Token {
            debug_id: 0,
            text: Arc::new("fn ".into()),
            not: false,
            inverted: false,
            property: None
        };
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let res = fn_.parse(&mut tokens, &s, &ReadToken::new(&chars, 0));
        assert_eq!(res, Ok((Range::new(0, 3), s, None)));
        assert_eq!(tokens.len(), 0);

        // Set bool property.
        let mut tokens = vec![];
        let has_arguments: Arc<String> = Arc::new("has_arguments".into());
        let start_parenthesis = Token {
            debug_id: 0,
            text: Arc::new("(".into()),
            not: false,
            inverted: false,
            property: Some(has_arguments.clone())
        };
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokens, &s,
            &ReadToken::new(&chars[6..], 6));
        assert_eq!(res, Ok((Range::new(6, 1), TokenizerState(1), None)));
        assert_eq!(tokens.len(), 1);
        assert_eq!(&tokens[0].data, &MetaData::Bool(has_arguments.clone(), true));

        // Set inverted bool property.
        let mut tokens = vec![];
        let has_arguments: Arc<String> = Arc::new("has_no_arguments".into());
        let start_parenthesis = Token {
            debug_id: 0,
            text: Arc::new("(".into()),
            not: false,
            inverted: true,
            property: Some(has_arguments.clone())
        };
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokens, &s,
            &ReadToken::new(&chars[6..], 6));
        assert_eq!(res, Ok((Range::new(6, 1), TokenizerState(1), None)));
        assert_eq!(tokens.len(), 1);
        assert_eq!(&tokens[0].data, &MetaData::Bool(has_arguments.clone(), false));
    }
}
