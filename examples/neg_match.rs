/*

This example shows how to use negative matching on tokens.

For example:

    !"hi":"hi_did_not_occured"

This rule generates:

    "hi_did_not_occured":true

when "hi" was *not* parsed.

One can also use double negative for simplicity:

    !"hi":!"hi"

This will generate:

    "hi":false

*/

extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = r#"hello!"#;
    let rules = r#"
        1 document = [!"hi":"hi_did_not_occured" ...""?]
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
        }
        Ok(()) => {}
    };
    // Prints `"hi_did_not_occured":true`.
    json::print(&data);
}
