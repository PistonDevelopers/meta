use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    ret_err,
    update,
    DebugId,
    MetaData,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores information about a node.
pub struct Node {
    /// The name of the node.
    pub name: Rc<String>,
    /// The rule of the node.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Node {
    /// Parses node.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = tokenizer.data(
            MetaData::StartNode(self.name.clone()),
            state,
            Range::empty(offset)
        );
        let mut opt_error = None;
        state = match self.rule.parse(tokenizer, &state, chars, offset) {
            Err(err) => { return Err(ret_err(err, opt_error)); }
            Ok((range, state, err)) => {
                update(range, err, &mut chars, &mut offset, &mut opt_error);
                state
            }
        };
        let range = Range::new(start_offset, offset - start_offset);
        Ok((
            range,
            tokenizer.data(MetaData::EndNode(self.name.clone()), &state, range),
            opt_error
        ))
    }
}

/// A node reference.
#[derive(Clone)]
pub enum NodeRef {
    /// Points to a node by name.
    Name(Rc<String>, DebugId),
    /// Reference to node.
    /// The `bool` flag is used to prevent multiple visits when updating.
    Ref(Rc<RefCell<Node>>, NodeVisit),
}

/// Tells whether a node is visited when updated.
#[derive(Clone)]
pub enum NodeVisit {
    /// The node is not being visited.
    Unvisited,
    /// The node is being visited.
    Visited
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn node_ref() {
        // Create a node rule the refers to itself.
        let foo: Rc<String> = Rc::new("foo".into());
        let num: Rc<String> = Rc::new("num".into());
        let node = Rc::new(RefCell::new(Node {
            debug_id: 0,
            name: foo.clone(),
            rule: Rule::Sequence(Sequence {
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
                                Rule::Node(NodeRef::Name(foo.clone(), 3)),
                            ]
                        }),
                    })),
                ],
            }),
        }));

        // Replace self referencing names with direct references.
        let refs = vec![(foo.clone(), node.clone())];
        let mut rules = Rule::Node(NodeRef::Name(foo.clone(), 0));
        rules.update_refs(&refs);

        let text = "1 2 3";
        let data = parse(&rules, text).unwrap();
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
