use range::Range;

use {
    update,
    MetaReader,
    Rule,
};

/// Stores information about optional.
pub struct Optional {
    /// The optional rules.
    pub args: Vec<Rule>,
}

impl Optional {
    /// Parse optional.
    /// Returns the old state if any sub rule fails.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        mut offset: usize
    ) -> (Range, M::State)
        where M: MetaReader
    {
        let start_offset = offset;
        let mut success_state = state.clone();
        for sub_rule in &self.args {
            success_state = match sub_rule.parse(meta_reader, &success_state,
                                         chars, offset) {
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
                Err(_) => {
                    return (Range::new(start_offset, 0), state.clone())
                }
            }
        }
        (Range::new(start_offset, offset - start_offset), success_state)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn fail_but_continue() {
        let text = "2";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let num: Rc<String> = Rc::new("num".into());
        // Will fail because text is expected first.
        let optional = Optional {
            args: vec![
                Rule::Text(Text {
                    allow_empty: true,
                    property: None
                }),
                Rule::Number(Number {
                    property: Some(num.clone())
                })
            ]
        };
        let res = optional.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, (Range::new(0, 0), TokenizerState(0)));
        assert_eq!(tokenizer.tokens.len(), 0);
    }
}
