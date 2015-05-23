use range::Range;

use {
    MetaData,
    ParseError,
};

/// Implemented by meta readers.
///
/// A meta reader contains an internal state that corresponds to a virtual
/// tree structure. The meta parser communicates with the meta reader such
/// that parsing is interrupted if any error happens.
pub trait MetaReader {
    /// The state that points to a location in the parsed structure.
    type State: Clone;

    /// Sends meta data.
    fn data(&mut self, data: MetaData, state: &Self::State, range: Range) ->
        Result<Self::State, ParseError>;
}
