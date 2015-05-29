extern crate piston_meta;
extern crate range;

use std::rc::Rc;
use piston_meta::*;

fn main() {
    let text = "foo \"Hello world!\"";
    let chars: Vec<char> = text.chars().collect();
    let mut tokenizer = Tokenizer::new();
    let s = TokenizerState::new();
    let foo: Rc<String> = Rc::new("foo".into());
    let text = Text {
        allow_empty: true,
        property: Some(foo.clone())
    };
    let _ = text.parse(&mut tokenizer, &s, &chars[4..], 4);
    assert_eq!(tokenizer.tokens.len(), 1);
    if let &MetaData::String(_, ref hello) = &tokenizer.tokens[0].0 {
        println!("{}", hello);
    }
}
