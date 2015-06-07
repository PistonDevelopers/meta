extern crate piston_meta;
extern crate range;

use std::rc::Rc;
use piston_meta::*;

fn main() {
    let text = "foo \"Hello world!\"";
    let foo: Rc<String> = Rc::new("foo".into());
    let rules = Rule::Sequence(Sequence {
        debug_id: 0,
        args: vec![
            Rule::Token(Token {
                debug_id: 1,
                text: foo.clone(),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2,
                optional: false,
            }),
            Rule::Text(Text {
                debug_id: 0,
                allow_empty: true,
                property: Some(foo.clone())
            })
        ]
    });
    let data = parse(&rules, text).unwrap();
    assert_eq!(data.len(), 1);
    if let &MetaData::String(_, ref hello) = &data[0].0 {
        println!("{}", hello);
    }
}
