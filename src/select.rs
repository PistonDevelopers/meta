use range::Range;

use Rule;
use MetaReader;
use ParseError;

/// Stores information about select.
pub struct Select<'a> {
    /// The rules to select from.
    pub args: &'a [Rule<'a>],
}

impl<'a> Select<'a> {
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
        for sub_rule in self.args {
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
