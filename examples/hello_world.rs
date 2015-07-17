extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = r#"say "Hello world!""#;
    let rules = r#"1 "rule" ["say" w! t?"foo"]"#;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = stderr_unwrap(rules, syntax(rules));
    let data = stderr_unwrap(text, parse(&rules, text));
    assert_eq!(data.len(), 1);
    if let &MetaData::String(_, ref hello) = &data[0].1 {
        println!("{}", hello);
    }
}
