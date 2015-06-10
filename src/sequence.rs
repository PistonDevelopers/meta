use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    ret_err,
    update,
    DebugId,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores information about sequence.
#[derive(Clone)]
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
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize,
        refs: &[(Rc<String>, Rc<RefCell<Rule>>)]
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut opt_error = None;
        for sub_rule in &self.args {
            state = match sub_rule.parse(
                tokenizer, &state, chars, offset, refs
            ) {
                Ok((range, state, err)) => {
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
                Err(err) => {
                    return Err(ret_err(err, opt_error));
                }
            }
        }
        Ok((Range::new(start_offset, offset - start_offset), state, opt_error))
    }
}
