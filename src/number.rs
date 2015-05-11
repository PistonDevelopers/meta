use range::Range;
use read_token;

use {
    MetaReader,
    ParseError,
};

/// Contains information about number.
pub struct Number<'a> {
    /// The property to set.
    pub property: Option<&'a str>,
}

impl<'a> Number<'a> {
    /// Parses number.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        if let Some(range) = read_token::number(chars, offset) {
            let mut text = String::with_capacity(range.length);
            for c in chars.iter().take(range.length) {
                text.push(*c);
            }
            match text.parse::<f64>() {
                Err(err) => Err((range, ParseError::ParseFloatError(err))),
                Ok(val) => {
                    if let Some(property) = self.property {
                        match meta_reader.set_as_f64(property, val, state) {
                            Err(err) => Err((range, err)),
                            Ok(state) => Ok((range, state)),
                        }
                    } else {
                        Ok((range, state.clone()))
                    }
                }
            }
        } else {
            return Err((Range::new(offset, 0), ParseError::ExpectedNumber))
        }
    }
}
