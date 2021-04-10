use range::Range;
use read_token::ReadToken;

use super::{
    ret_err,
    update,
    IndentSettings,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores information about sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct Sequence {
    /// The sequential rules.
    pub args: Vec<Rule>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Sequence {
    /// Parses sequence.
    /// Fails if any sub rule fails.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule],
        indent_settings: &mut IndentSettings,
    ) -> ParseResult<TokenizerState> {
        let start = read_token;
        let mut read_token = *start;
        let mut state = state.clone();
        let mut opt_error = None;
        for sub_rule in &self.args {
            state = match sub_rule.parse(tokens, &state, &read_token, refs, indent_settings) {
                Ok((range, state, err)) => {
                    update(range, err, &mut read_token, &mut opt_error);
                    state
                }
                Err(err) => {
                    return Err(ret_err(err, opt_error));
                }
            }
        }
        Ok((read_token.subtract(start), state, opt_error))
    }
}
