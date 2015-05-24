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

impl Tokenizer {
    /// Creates a new tokenizer.
    pub fn new() -> Tokenizer {
        Tokenizer { tokens: vec![] }
    }
}

/// Stores the number of tokens received.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TokenizerState(pub usize);

impl TokenizerState {
    /// Creates a new tokenizer state.
    pub fn new() -> TokenizerState { TokenizerState(0) }
}

impl MetaReader for Tokenizer {
    type State = TokenizerState;

    fn data(&mut self, data: MetaData, state: &Self::State, range: Range)
        -> Result<Self::State, ParseError>
    {
        if state.0 < self.tokens.len() {
            self.tokens.truncate(state.0);
        }
        self.tokens.push((data, range));
        Ok(TokenizerState(self.tokens.len()))
    }
}
