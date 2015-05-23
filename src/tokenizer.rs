use range::Range;

use {
    MetaData,
    MetaReader,
    ParseError,
};

/// Stores all the meta data sequentially.
pub struct Tokenizer {
    /// The read tokens.
    pub tokens: Vec<(MetaData, Range)>
}

impl MetaReader for Tokenizer {
    type State = usize;


    fn data(&mut self, data: MetaData, state: &Self::State, range: Range)
        -> Result<Self::State, ParseError>
    {
        if *state < self.tokens.len() {
            self.tokens.truncate(*state);
        }
        self.tokens.push((data, range));
        Ok(self.tokens.len())
    }
}
