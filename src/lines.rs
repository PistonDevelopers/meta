use range::Range;

use {
    ret_err,
    update,
    DebugId,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores information about lines.
#[derive(Clone)]
pub struct Lines {
    /// The rule to read lines.
    /// This can be a multi-line rule.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Lines {
    /// Parses rule separated by one or more lines.
    /// Ignores lines that only contain whitespace characters.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut opt_error = None;
        loop {
            let len = chars.iter()
                .take_while(|&c| *c != '\n' && c.is_whitespace())
                .count();
            if len == chars.len() {
                offset += len;
                break;
            }
            else if chars[len] == '\n' {
                chars = &chars[len + 1..];
                offset += len + 1;
            } else {
                state = match self.rule.parse(tokenizer, &state, chars, offset) {
                    Err(err) => { return Err(ret_err(err, opt_error)); }
                    Ok((range, state, err)) => {
                        update(range, err, &mut chars, &mut offset, &mut opt_error);
                        state
                    }
                };
            }
        }
        Ok((Range::new(start_offset, offset - start_offset), state, opt_error))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn fail() {
        let text = "
1
2

3


\"error\"
4
        ";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: None,
            }),
        };
        let res = lines.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((Range::new(10, 0), ParseError::ExpectedNumber(1))));
    }

    #[test]
    fn success() {
        let text = "
1
2

3


4
 ";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let val: Rc<String> = Rc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
            }),
        };
        let res = lines.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 13), TokenizerState(4), None)));
    }
}
