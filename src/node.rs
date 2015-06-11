use range::Range;
use std::rc::Rc;
use std::cell::Cell;

use {
    ret_err,
    update,
    DebugId,
    MetaData,
    ParseError,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// A node reference.
#[derive(Clone)]
pub struct Node {
    /// Name of rule.
    pub name: Rc<String>,
    /// The property to set.
    pub property: Option<Rc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
    /// The index to the rule reference.
    pub index: Cell<Option<usize>>,
}

impl Node {
    /// Parses node.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize,
        refs: &[(Rc<String>, Rule)]
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let index = match self.index.get() {
            None => {
                return Err((
                    Range::empty(offset),
                    ParseError::InvalidRule(
                        "Node rule is not updated to reference",
                        self.debug_id
                    )
                ));
            }
            Some(i) => i
        };
        let mut state = if let Some(ref prop) = self.property {
            tokenizer.data(
                MetaData::StartNode(prop.clone()),
                state,
                Range::empty(offset)
            )
        } else {
            state.clone()
        };
        let mut opt_error = None;
        state = match refs[index].1.parse(
            tokenizer, &state, chars, offset, refs
        ) {
            Err(err) => { return Err(ret_err(err, opt_error)); }
            Ok((range, state, err)) => {
                update(range, err, &mut chars, &mut offset, &mut opt_error);
                state
            }
        };
        let range = Range::new(start_offset, offset - start_offset);
        Ok((
            range,
            if let Some(ref prop) = self.property {
                tokenizer.data(
                    MetaData::EndNode(prop.clone()),
                    &state,
                    range
                )
            } else {
                state.clone()
            },
            opt_error
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::rc::Rc;
    use std::cell::Cell;

    #[test]
    fn node_ref() {
        // Create a node rule the refers to itself.
        let foo: Rc<String> = Rc::new("foo".into());
        let num: Rc<String> = Rc::new("num".into());
        let node = Rule::Sequence(Sequence {
            debug_id: 1,
            args: vec![
                Rule::Number(Number {
                    debug_id: 2,
                    property: Some(num.clone()),
                    allow_underscore: false,
                }),
                Rule::Optional(Box::new(Optional {
                    debug_id: 3,
                    rule: Rule::Sequence(Sequence {
                        debug_id: 4,
                        args: vec![
                            Rule::Whitespace(Whitespace {
                                debug_id: 3,
                                optional: false
                            }),
                            Rule::Node(Node {
                                name: foo.clone(),
                                property: Some(foo.clone()),
                                debug_id: 3,
                                index: Cell::new(None),
                            }),
                        ]
                    }),
                })),
            ],
        });

        // Replace self referencing names with direct references.
        let refs = vec![(foo.clone(), node)];
        let rules = Rule::Node(Node {
            name: foo.clone(),
            property: Some(foo.clone()),
            debug_id: 0,
            index: Cell::new(None),
        });
        update_refs(&rules, &refs);

        let text = "1 2 3";
        let data = parse(&rules, &refs, text).unwrap();
        assert_eq!(data.len(), 9);
        assert_eq!(&data[0].1, &MetaData::StartNode(foo.clone()));
        assert_eq!(&data[1].1, &MetaData::F64(num.clone(), 1.0));
        assert_eq!(&data[2].1, &MetaData::StartNode(foo.clone()));
        assert_eq!(&data[3].1, &MetaData::F64(num.clone(), 2.0));
        assert_eq!(&data[4].1, &MetaData::StartNode(foo.clone()));
        assert_eq!(&data[5].1, &MetaData::F64(num.clone(), 3.0));
        assert_eq!(&data[6].1, &MetaData::EndNode(foo.clone()));
        assert_eq!(&data[7].1, &MetaData::EndNode(foo.clone()));
        assert_eq!(&data[8].1, &MetaData::EndNode(foo.clone()));
    }
}
