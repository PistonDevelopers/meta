use range::Range;

use {
    MetaReader,
    ParseError,
    Rule,
};

/// Stores information about select.
pub struct Select {
    /// The rules to select from.
    pub args: Vec<Rule>,
}

impl Select {
    /// Parses select.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        let mut first_error: Option<(Range, ParseError)> = None;
        for sub_rule in &self.args {
            match sub_rule.parse(meta_reader, state, chars, offset) {
                Ok((range, state)) => {
                    return Ok((Range::new(offset, range.next_offset()), state));
                }
                Err(err) => {
                    first_error = Some(err);
                }
            }
        }
        match first_error {
            None => Err((Range::new(offset, 0), ParseError::InvalidRule(
                "`Select` requires at least one sub rule"))),
            Some(err) => Err(err),
        }
    }
}
