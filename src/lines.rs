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
            if len == chars.len() { break; }
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
