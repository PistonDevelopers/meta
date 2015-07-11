//! Tracking tokenizer state.

use range::Range;

use {
    MetaData,
};

/// Reads meta data.
pub fn read_data(
    tokens: &mut Vec<(Range, MetaData)>,
    data: MetaData,
    state: &TokenizerState,
    range: Range
) -> TokenizerState {
    if state.0 < tokens.len() {
        tokens.truncate(state.0);
    }
    tokens.push((range, data));
    TokenizerState(tokens.len())
}

/// Stores the number of tokens received.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TokenizerState(pub usize);

impl TokenizerState {
    /// Creates a new tokenizer state.
    pub fn new() -> TokenizerState { TokenizerState(0) }
}
