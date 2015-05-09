use range::Range;

use {
    update,
    MetaReader,
    Rule,
};

/// Stores information about optional.
pub struct Optional<'a> {
    /// The optional rules.
    pub args: &'a [Rule<'a>],
}

impl<'a> Optional<'a> {
    /// Parse optional.
    /// Returns the old state if any sub rule fails.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        mut offset: usize
    ) -> (Range, M::State)
        where M: MetaReader
    {
        let start_offset = offset;
        let mut success_state = state.clone();
        for sub_rule in self.args {
            success_state = match sub_rule.parse(meta_reader, &success_state,
                                         chars, offset) {
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
                Err(_) => {
                    return (Range::new(start_offset, 0), state.clone())
                }
            }
        }
        (Range::new(start_offset, offset - start_offset), success_state)
    }
}
