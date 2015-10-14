extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = r#"hi James!"#;
    let rules = r#"
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = stderr_unwrap(rules, syntax2(rules));
    let mut data = vec![];
    stderr_unwrap(text, parse(&rules, text, &mut data));
    json::print(&data);
}
