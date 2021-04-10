extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = r#"
1
 2
  3
  4
 5
  6
    "#;
    let rules = r#"
        2 node = [.$:"num" .l+(node:"node")]
        1 document = [.w? node:"node" .w?]
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
