use range::Range;

use {
    update,
    MetaReader,
    ParseError,
    Rule,
};

/// Stores inforamtion about separated by.
pub struct SeparatedBy {
    /// The rule to separate.
    pub rule: Rule,
    /// The rule to separate by.
    pub by: Rule,
    /// Whether the rule must occur at least once.
    pub optional: bool,
    /// Whether the rule can end with separator.
    pub allow_trail: bool,
}

impl SeparatedBy {
    /// Parses rule repeatedly separated by another rule.
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
        let mut first = true;
        loop {
            state = match self.rule.parse(meta_reader, &state, chars, offset) {
                Err(err) => {
                    match (first, self.optional, self.allow_trail) {
                          (true, false, _)
                        | (false, _, false) => { return Err(err); }
                          (true, true, _)
                        | (false, _, true) => { break; }
                    }
                }
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
            };
            state = match self.by.parse(meta_reader, &state, chars, offset) {
                Err(_) => { break; }
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
            };
            first = false;
        }
        Ok((Range::new(start_offset, offset - start_offset), state))
    }
}
