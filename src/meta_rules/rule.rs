use range::Range;
use std::sync::Arc;
use read_token::ReadToken;

use super::{
    Lines,
    Node,
    Number,
    Optional,
    ParseResult,
    Repeat,
    Select,
    SeparateBy,
    Sequence,
    Text,
    Tag,
    UntilAny,
    UntilAnyOrWhitespace,
    Whitespace,
};
use MetaData;
use tokenizer::TokenizerState;

/// A rule describes how some section of a document should be parsed.
#[derive(Clone, Debug, PartialEq)]
pub enum Rule {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a tag.
    Tag(Tag),
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
    SeparateBy(Box<SeparateBy>),
    /// Repeat rule.
    Repeat(Box<Repeat>),
    /// Repeat rule separated by one or more lines.
    Lines(Box<Lines>),
    /// Read node.
    Node(Node),
    /// Read optional.
    Optional(Box<Optional>),
}

impl Rule {
    /// Parses rule.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule]
    ) -> ParseResult<TokenizerState> {
        match self {
            &Rule::Whitespace(ref w) => {
                w.parse(read_token).map(|r| (r, state.clone(), None))
            }
            &Rule::Tag(ref t) => {
                t.parse(tokens, state, read_token)
            }
            &Rule::UntilAny(ref u) => {
                u.parse(tokens, state, read_token)
            }
            &Rule::UntilAnyOrWhitespace(ref u) => {
                u.parse(tokens, state, read_token)
            }
            &Rule::Text(ref t) => {
                t.parse(tokens, state, read_token)
            }
            &Rule::Number(ref n) => {
                n.parse(tokens, state, read_token)
            }
            &Rule::Select(ref s) => {
                s.parse(tokens, state, read_token, refs)
            }
            &Rule::Sequence(ref s) => {
                s.parse(tokens, state, read_token, refs)
            }
            &Rule::SeparateBy(ref s) => {
                s.parse(tokens, state, read_token, refs)
            }
            &Rule::Repeat(ref r) => {
                r.parse(tokens, state, read_token, refs)
            }
            &Rule::Lines(ref l) => {
                l.parse(tokens, state, read_token, refs)
            }
            &Rule::Node(ref p) => {
                p.parse(tokens, state, read_token, refs)
            }
            &Rule::Optional(ref o) => {
                Ok(o.parse(tokens, state, read_token, refs))
            }
        }
    }

    /// Updates replacing names with the references.
    ///
    /// The references contains the name,
    /// but this can not be borrowed as when the same reference is updated.
    pub fn update_refs(&mut self, names: &[Arc<String>]) {
        match self {
            &mut Rule::Node(ref mut p) => {
                match p.index {
                    None => {
                        // Look through references and update if correct name
                        // is found.
                        let mut found: Option<usize> = None;
                        for (i, r) in names.iter().enumerate() {
                            if &**p.name == &**r {
                                found = Some(i);
                                break;
                            }
                        }
                        match found {
                            None => { return; }
                            Some(i) => {
                                p.index = Some(i);
                                return;
                            }
                        }
                    }
                    Some(_) => {
                        return;
                    }
                };
            }
            &mut Rule::Whitespace(_) => {}
            &mut Rule::Tag(_) => {}
            &mut Rule::UntilAny(_) => {}
            &mut Rule::UntilAnyOrWhitespace(_) => {}
            &mut Rule::Text(_) => {}
            &mut Rule::Number(_) => {}
            &mut Rule::Select(ref mut s) => {
                for sub_rule in &mut s.args {
                    sub_rule.update_refs(names);
                }
            }
            &mut Rule::Sequence(ref mut s) => {
                for sub_rule in &mut s.args {
                    sub_rule.update_refs(names);
                }
            }
            &mut Rule::SeparateBy(ref mut s) => {
                s.rule.update_refs(names);
                s.by.update_refs(names);
            }
            &mut Rule::Repeat(ref mut r) => {
                r.rule.update_refs(names);
            }
            &mut Rule::Lines(ref mut l) => {
                l.rule.update_refs(names);
            }
            &mut Rule::Optional(ref mut o) => {
                o.rule.update_refs(names);
            }
        }
    }
}
