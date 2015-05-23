use range::Range;
use read_token;
use std::rc::Rc;

use {
    MetaData,
    MetaReader,
    ParseError,
};

/// Contains information about number.
pub struct Number {
    /// The property to set.
    pub property: Option<Rc<String>>,
}

impl Number {
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
                    if let Some(ref property) = self.property {
                        match meta_reader.data(
                            MetaData::F64(property.clone(), val),
                            state,
                            range
                        ) {
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
