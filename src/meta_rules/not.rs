use range::Range;
use read_token::ReadToken;

use std::sync::Arc;

use super::ParseResult;
use {
    DebugId,
    MetaData,
    ParseError,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores information about not.
#[derive(Clone, Debug, PartialEq)]
pub struct Not {
    /// The not rule.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Not {
    /// Parse not.
    /// Fails if sub rule succeeds.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule]
    ) -> ParseResult<TokenizerState> {
        let start = read_token;
        match self.rule.parse(
            tokens, state, read_token, refs
        ) {
            Ok((range, _, _)) => {
                let text = read_token.raw_string(range.length);
                Err(range.wrap(
                    ParseError::DidNotExpectTag(Arc::new(text),
                    self.debug_id)))
            }
            Err(_) => {
                Ok((start.start(), state.clone(), None))
            }
        }
    }
}
