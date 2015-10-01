use range::Range;
use std::sync::Arc;

use super::{
    ret_err,
    update,
};
use {
    DebugId,
    MetaData,
    ParseError,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores information about optional.
#[derive(Clone, Debug, PartialEq)]
pub struct Optional {
    /// The optional rule.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Optional {
    /// Parse optional.
    /// Returns the old state if any sub rule fails.
    pub fn parse(
        &self,
        tokens: &mut Vec<(Range, MetaData)>,
        state: &TokenizerState,
        mut chars: &[char],
        mut offset: usize,
        refs: &[(Arc<String>, Rule)]
    ) -> (Range, TokenizerState, Option<(Range, ParseError)>) {
        let start_offset = offset;
        let mut success_state = state.clone();
        let mut opt_error = None;
        success_state = match self.rule.parse(
            tokens, &success_state, chars, offset, refs
        ) {
            Ok((range, state, err)) => {
                update(range, err, &mut chars, &mut offset, &mut opt_error);
                state
            }
            Err(err) => {
                return (Range::new(start_offset, 0), state.clone(),
                    Some(ret_err(err, opt_error)))
            }
        };
        (Range::new(start_offset, offset - start_offset), success_state,
            opt_error)
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::{ Number, Optional, Sequence, Text };
    use range::Range;
    use std::sync::Arc;

    #[test]
    fn fail_but_continue() {
        let text = "2";
        let chars: Vec<char> = text.chars().collect();
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let num: Arc<String> = Arc::new("num".into());
        // Will fail because text is expected first.
        let optional = Optional {
            debug_id: 0,
            rule: Rule::Sequence(Sequence {
                debug_id: 1,
                args: vec![
                    Rule::Text(Text {
                        debug_id: 2,
                        allow_empty: true,
                        property: None
                    }),
                    Rule::Number(Number {
                        debug_id: 3,
                        property: Some(num.clone()),
                        allow_underscore: false,
                    })
                ]
            }),
        };
        let res = optional.parse(&mut tokens, &s, &chars, 0, &[]);
        assert_eq!(res, (Range::new(0, 0), TokenizerState(0),
            Some((Range::new(0, 0), ParseError::ExpectedText(2)))));
        assert_eq!(tokens.len(), 0);
    }
}
