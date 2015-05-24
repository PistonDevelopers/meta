use range::Range;

use {
    update,
    MetaReader,
    ParseError,
    Rule,
};

/// Stores information about sequence.
pub struct Sequence {
    /// The sequential rules.
    pub args: Vec<Rule>
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
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        let mut offset = start_offset;
        let mut state = state.clone();
        for sub_rule in &self.args {
            state = match sub_rule.parse(meta_reader, &state, chars, offset) {
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok((Range::new(start_offset, offset - start_offset), state))
    }
}
