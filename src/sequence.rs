use range::Range;

use {
    ret_err,
    update,
    DebugId,
    MetaReader,
    ParseResult,
    Rule,
};

/// Stores information about sequence.
pub struct Sequence {
    /// The sequential rules.
    pub args: Vec<Rule>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Sequence {
    /// Parses sequence.
    /// Fails if any sub rule fails.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        start_offset: usize
    ) -> ParseResult<M::State>
        where M: MetaReader
    {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut opt_error = None;
        for sub_rule in &self.args {
            state = match sub_rule.parse(meta_reader, &state, chars, offset) {
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
