use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    ret_err,
    err_update,
    update,
    DebugId,
    Node,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores inforamtion about separated by.
#[derive(Clone)]
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
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize,
        refs: &[(Rc<String>, Rc<RefCell<Node>>)]
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut opt_error = None;
        let mut first = true;
        loop {
            state = match self.rule.parse(
                tokenizer, &state, chars, offset, refs) {
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
    use super::super::*;
    use std::rc::Rc;
    use range::Range;

    #[test]
    fn fail() {
        let text = "[a][a][a]";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let token: Rc<String> = Rc::new("(a)".into());
        let rule = Repeat {
            debug_id: 0,
            optional: false,
            rule: Rule::Token(Token {
                debug_id: 1,
                text: token.clone(),
                inverted: false,
                property: None,
            })
        };
        let res = rule.parse(&mut tokenizer, &s, &chars, 0, &[]);
        assert_eq!(res, Err((Range::new(0, 0),
            ParseError::ExpectedToken(token.clone(), 1))))
    }

    #[test]
    fn success() {
        let text = "(a)(a)(a)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let token: Rc<String> = Rc::new("(a)".into());
        let rule = Repeat {
            debug_id: 0,
            optional: false,
            rule: Rule::Token(Token {
                debug_id: 1,
                text: token.clone(),
                inverted: false,
                property: None,
            })
        };
        let res = rule.parse(&mut tokenizer, &s, &chars, 0, &[]);
        assert_eq!(res, Ok((Range::new(0, 9), TokenizerState(0),
            Some((Range::new(9, 0), ParseError::ExpectedToken(token.clone(), 1))))))
    }
}
