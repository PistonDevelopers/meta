extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = r#"hi James!"#;
    let rules = r#"
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = match syntax_errstr(rules) {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(rules) => rules
    };
    let mut data = vec![];
    match parse_errstr(&rules, text, &mut data) {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(()) => {}
    };
    json::print(&data);
}
