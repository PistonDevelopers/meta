use range::Range;

use super::{
    ret_err,
    err_update,
    update,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores inforamtion about separated by.
#[derive(Clone, Debug, PartialEq)]
pub struct Repeat {
    /// The rule to separate.
    pub rule: Rule,
    /// Whether the rule must occur at least once.
    pub optional: bool,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Repeat {
    /// Parses rule repeatedly.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize,
        refs: &[Rule]
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut opt_error = None;
        let mut first = true;
        loop {
            state = match self.rule.parse(tokens, &state, chars, offset, refs) {
                Err(err) => {
                    if first && !self.optional {
                        return Err(ret_err(err, opt_error));
                    } else {
                        err_update(Some(err), &mut opt_error);
                        break;
                    }
                }
                Ok((range, state, err)) => {
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
            };
            first = false;
        }
        Ok((Range::new(start_offset, offset - start_offset), state, opt_error))
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::{ Repeat, Token };
    use std::sync::Arc;
    use range::Range;

    #[test]
    fn fail() {
        let text = "[a][a][a]";
        let chars: Vec<char> = text.chars().collect();
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let token: Arc<String> = Arc::new("(a)".into());
        let rule = Repeat {
            debug_id: 0,
            optional: false,
            rule: Rule::Token(Token {
                debug_id: 1,
                text: token.clone(),
                not: false,
                inverted: false,
                property: None,
            })
        };
        let res = rule.parse(&mut tokens, &s, &chars, 0, &[]);
        assert_eq!(res, Err(Range::new(0, 0).wrap(
            ParseError::ExpectedToken(token.clone(), 1))))
    }

    #[test]
    fn success() {
        let text = "(a)(a)(a)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let token: Arc<String> = Arc::new("(a)".into());
        let rule = Repeat {
            debug_id: 0,
            optional: false,
            rule: Rule::Token(Token {
                debug_id: 1,
                text: token.clone(),
                not: false,
                inverted: false,
                property: None,
            })
        };
        let res = rule.parse(&mut tokens, &s, &chars, 0, &[]);
        assert_eq!(res, Ok((Range::new(0, 9), TokenizerState(0),
            Some(Range::new(9, 0).wrap(ParseError::ExpectedToken(token.clone(), 1))))))
    }
}
