use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    update,
    MetaData,
    MetaReader,
    ParseError,
    Rule,
};

/// Stores information about a parameter.
pub struct Node {
    /// The name of the parameter.
    pub name: Rc<String>,
    /// The properties of the parameter.
    /// This is used to check the property names set by sub rules.
    /// If a property name does not match any of the arguments to the parameter,
    /// then an error is reported.
    pub args: Vec<Rc<String>>,
    /// The property name of parent to set the value.
    pub value: Option<Rc<String>>,
    /// The body of the parameter.
    pub body: Vec<Rule>,
}

impl Node {
    /// Parses parameter.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        start_offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        let mut offset = start_offset;
        let name = self.value.clone().unwrap_or(self.name.clone());
        let mut state = match meta_reader.data(
            MetaData::StartNode(name),
            state,
            Range::empty(offset)
        ) {
            Err(err) => { return Err((Range::new(offset, 0), err)); }
            Ok(state) => state,
        };
        for rule in &self.body {
            state = match rule.parse(meta_reader, &state, chars, offset) {
                Err(err) => { return Err(err); }
                Ok((range, state)) => {
                    update(range, &mut chars, &mut offset);
                    state
                }
            }
        }
        let range = Range::new(start_offset, offset - start_offset);
        match meta_reader.data(MetaData::EndNode, &state, range) {
            Err(err) => { return Err((range, err)); }
            Ok(state) => Ok((range, state)),
        }
    }
}

/// A parameter reference.
pub enum NodeRef {
    /// Points to a parameter by name.
    Name(Rc<String>),
    /// Reference to parameter.
    /// The `bool` flag is used to prevent multiple visits when updating.
    Ref(Rc<RefCell<Node>>, NodeVisit),
}

/// Tells whether a parameter is visited when updated.
pub enum NodeVisit {
    /// The parameter is not being visited.
    Unvisited,
    /// The parameter is being visited.
    Visited
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn parameter_ref() {
        // Create a parameter rule the refers to itself.
        let foo: Rc<String> = Rc::new("foo".into());
        let num: Rc<String> = Rc::new("num".into());
        let node = Rc::new(RefCell::new(Node {
            name: foo.clone(),
            args: vec![],
            value: None,
            body: vec![
                Rule::Number(Number { property: Some(num.clone()) }),
                Rule::Optional(Optional {
                    args: vec![
                        Rule::Whitespace(Whitespace { optional: false }),
                        Rule::Node(NodeRef::Name(foo.clone())),
                    ]
                })
            ]
        }));

        // Replace self referencing names with direct references.
        let refs = vec![(foo.clone(), node.clone())];
        for sub_rule in &mut node.borrow_mut().body {
            sub_rule.update_refs(&refs);
        }

        let text = "1 2 3";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = node.borrow().parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 5), TokenizerState(9))));
        assert_eq!(tokenizer.tokens.len(), 9);
        assert_eq!(&tokenizer.tokens[0].0, &MetaData::StartNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[1].0, &MetaData::F64(num.clone(), 1.0));
        assert_eq!(&tokenizer.tokens[2].0, &MetaData::StartNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[3].0, &MetaData::F64(num.clone(), 2.0));
        assert_eq!(&tokenizer.tokens[4].0, &MetaData::StartNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[5].0, &MetaData::F64(num.clone(), 3.0));
        assert_eq!(&tokenizer.tokens[6].0, &MetaData::EndNode);
        assert_eq!(&tokenizer.tokens[7].0, &MetaData::EndNode);
        assert_eq!(&tokenizer.tokens[8].0, &MetaData::EndNode);
    }
}
