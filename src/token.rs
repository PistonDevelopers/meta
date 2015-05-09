use range::Range;
use read_token;

use MetaReader;
use ParseError;

/// Stores information about token.
pub struct Token<'a> {
    /// The text to match against.
    pub text: &'a str,
    /// Whether to set property to true or false (inverted).
    pub inverted: Option<bool>,
    /// Which predicate to set if token matches.
    /// This is the name of the property in current node.
    pub predicate: Option<&'a str>,
}

impl<'a> Token<'a> {
    /// Parses token.
    /// If the token is linked to a property,
    /// the property will be set.
    /// If the meta reader fails setting the property the error is handled.
    /// If the token is not linked to any property,
    /// the same state will be returned.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        if let Some(range) = read_token::token(self.text, chars, offset) {
            match (self.inverted, self.predicate) {
                (Some(inverted), Some(name)) => {
                    match meta_reader.set_as_bool(name, !inverted, &state) {
                        Err(err) => {
                            return Err((range, err));
                        }
                        Ok(state) => {
                            return Ok((range, state));
                        }
                    }
                }
                _ => {
                    return Ok((range, state.clone()))
                }
            }
        } else {
            return Err((Range::new(offset, 0),
                ParseError::ExpectedToken(self.text.into())));
        }
    }
}
