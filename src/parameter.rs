use range::Range;

use {
    update,
    MetaData,
    MetaReader,
    ParseError,
    Rule,
};

/// Stores information about a parameter.
pub struct Parameter<'a> {
    /// The name of the parameter.
    pub name: &'a str,
    /// The properties of the parameter.
    /// This is used to check the property names set by sub rules.
    /// If a property name does not match any of the arguments to the parameter,
    /// then an error is reported.
    pub args: &'a [&'a str],
    /// The property name of parent to set the value.
    pub value: Option<&'a str>,
    /// The body of the parameter.
    pub body: &'a [Rule<'a>],
}

impl<'a> Parameter<'a> {
    /// Parses parameter.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        mut offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        let start_offset = offset;
        let name = self.value.unwrap_or(self.name);
        let mut state = match meta_reader.data(
            MetaData::StartNode(name),
            state,
            Range::empty(offset)
        ) {
            Err(err) => { return Err((Range::new(offset, 0), err)); }
            Ok(state) => state,
        };
        for rule in self.body {
            state = match rule.parse(meta_reader, &state, chars, offset) {
                Err(err) => { return Err(err); }
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
            }
        }
        let range = Range::new(start_offset, offset - start_offset);
        match meta_reader.data(MetaData::EndNode, &state, range) {
            Err(err) => { return Err((range, err)); }
            Ok(state) => Ok((range, state)),
        }
    }
}
