use range::Range;
use std::rc::Rc;

use {
    Whitespace,
    Token,
    UntilAny,
    UntilAnyOrWhitespace,
    Text,
    Number,
    NodeRef,
    NodeVisit,
    ParseError,
    ParseResult,
    Select,
    SeparatedBy,
    Repeat,
    Lines,
    Sequence,
    Optional,
    Tokenizer,
    TokenizerState,
};

/// A rule describes how some section of a document should be parsed.
#[derive(Clone)]
pub enum Rule {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a token.
    Token(Token),
    /// Reads until any character.
    UntilAny(UntilAny),
    /// Read until any character or whitespace.
    UntilAnyOrWhitespace(UntilAnyOrWhitespace),
    /// Read text.
    Text(Text),
    /// Read number.
    Number(Number),
    /// Select one of the sub rules.
    /// If the first one does not succeed, try another and so on.
    /// If all sub rules fail, then the rule fails.
    Select(Select),
    /// Run each sub rule in sequence.
    /// If any sub rule fails, the rule fails.
    Sequence(Sequence),
    /// Repeat rule separated by another rule.
    SeparatedBy(Box<SeparatedBy>),
    /// Repeat rule.
    Repeat(Box<Repeat>),
    /// Repeat rule separated by one or more lines.
    Lines(Box<Lines>),
    /// Read node.
    Node(NodeRef),
    /// Read optional.
    Optional(Box<Optional>),
}

impl Rule {
    /// Parses rule.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize,
        refs: &[(Rc<String>, Rule)]
    ) -> ParseResult<TokenizerState> {
        match self {
            &Rule::Whitespace(ref w) => {
                w.parse(chars, offset).map(|r| (r, state.clone(), None))
            }
            &Rule::Token(ref t) => {
                t.parse(tokenizer, state, chars, offset)
            }
            &Rule::UntilAny(ref u) => {
                u.parse(tokenizer, state, chars, offset)
            }
            &Rule::UntilAnyOrWhitespace(ref u) => {
                u.parse(tokenizer, state, chars, offset)
            }
            &Rule::Text(ref t) => {
                t.parse(tokenizer, state, chars, offset)
            }
            &Rule::Number(ref n) => {
                n.parse(tokenizer, state, chars, offset)
            }
            &Rule::Select(ref s) => {
                s.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Sequence(ref s) => {
                s.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::SeparatedBy(ref s) => {
                s.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Repeat(ref r) => {
                r.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Lines(ref l) => {
                l.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Node(ref p) => {
                match p.index.get() {
                    None => {
                        Err((
                            Range::empty(offset),
                            ParseError::InvalidRule(
                                "Node rule is not updated to reference",
                                p.debug_id
                            )
                        ))
                    }
                    Some(i) => {
                        refs[i].1.parse(
                            tokenizer, state, chars, offset, refs
                        )
                    }
                }
            }
            &Rule::Optional(ref o) => {
                Ok(o.parse(tokenizer, state, chars, offset, refs))
            }
        }
    }

    /// Updates replacing names with the references.
    ///
    /// The references contains the name,
    /// but this can not be borrowed as when the same reference is updated.
    pub fn update_refs(&self, refs: &[(Rc<String>, Rule)]) {
        match self {
            &Rule::Node(ref p) => {
                match p.index.get() {
                    None => {
                        // Look through references and update if correct name
                        // is found.
                        let mut found: Option<usize> = None;
                        for (i, r) in refs.iter().enumerate() {
                            if &**p.name == &*r.0 {
                                found = Some(i);
                                break;
                            }
                        }
                        match found {
                            None => { return; }
                            Some(i) => {
                                p.index.set(Some(i));
                                p.node_visit.set(NodeVisit::Visited);
                                refs[i].1.update_refs(refs);
                                return;
                            }
                        }
                    }
                    Some(i) => {
                        // Update the sub rules of the reference,
                        // but only if it has not been visited.
                        if let NodeVisit::Unvisited = p.node_visit.get() {
                            p.node_visit.set(NodeVisit::Visited);
                            refs[i].1.update_refs(refs);
                        }
                        return;
                    }
                };
            }
            &Rule::Whitespace(_) => {}
            &Rule::Token(_) => {}
            &Rule::UntilAny(_) => {}
            &Rule::UntilAnyOrWhitespace(_) => {}
            &Rule::Text(_) => {}
            &Rule::Number(_) => {}
            &Rule::Select(ref s) => {
                for sub_rule in &s.args {
                    sub_rule.update_refs(refs);
                }
            }
            &Rule::Sequence(ref s) => {
                for sub_rule in &s.args {
                    sub_rule.update_refs(refs);
                }
            }
            &Rule::SeparatedBy(ref s) => {
                s.rule.update_refs(refs);
                s.by.update_refs(refs);
            }
            &Rule::Repeat(ref r) => {
                r.rule.update_refs(refs);
            }
            &Rule::Lines(ref l) => {
                l.rule.update_refs(refs);
            }
            &Rule::Optional(ref o) => {
                o.rule.update_refs(refs);
            }
        }
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
                            Rule::Node(NodeRef {
                                name: foo.clone(),
                                debug_id: 3,
                                index: Cell::new(None),
                                node_visit: Cell::new(NodeVisit::Unvisited)
                            }),
                        ]
                    }),
                })),
            ],
        });

        // Replace self referencing names with direct references.
        let refs = vec![(foo.clone(), node)];
        let rules = Rule::Node(NodeRef {
            name: foo.clone(),
            debug_id: 0,
            index: Cell::new(None),
            node_visit: Cell::new(NodeVisit::Unvisited)
        });
        rules.update_refs(&refs);

        let text = "1 2 3";
        let data = parse(&rules, &refs, text).unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(&data[0].1, &MetaData::F64(num.clone(), 1.0));
        assert_eq!(&data[1].1, &MetaData::F64(num.clone(), 2.0));
        assert_eq!(&data[2].1, &MetaData::F64(num.clone(), 3.0));
    }
}
