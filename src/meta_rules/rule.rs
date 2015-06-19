use std::rc::Rc;

use super::{
    Lines,
    Node,
    Number,
    Optional,
    ParseResult,
    Repeat,
    Select,
    SeparatedBy,
    Sequence,
    Text,
    Token,
    UntilAny,
    UntilAnyOrWhitespace,
    Whitespace,
};
use {
    Tokenizer,
    TokenizerState,
};

/// A rule describes how some section of a document should be parsed.
#[derive(Clone, Debug, PartialEq)]
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
    Node(Node),
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
                p.parse(tokenizer, state, chars, offset, refs)
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
                                return;
                            }
                        }
                    }
                    Some(_) => {
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
