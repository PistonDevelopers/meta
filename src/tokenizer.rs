use range::Range;

use {
    MetaData,
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

    /// Reads meta data.
    pub fn data(&mut self, data: MetaData, state: &TokenizerState, range: Range)
        -> Result<TokenizerState, ParseError>
    {
        if state.0 < self.tokens.len() {
            self.tokens.truncate(state.0);
        }
        self.tokens.push((data, range));
        Ok(TokenizerState(self.tokens.len()))
    }
}

/// Stores the number of tokens received.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TokenizerState(pub usize);

impl TokenizerState {
    /// Creates a new tokenizer state.
    pub fn new() -> TokenizerState { TokenizerState(0) }
}
