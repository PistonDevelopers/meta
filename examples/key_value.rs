
extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = r#"
        # A monster
        age = 250
        strength = 200
        name = "Big Dragon"
        violent = true
    "#;
    let rules = r##"
        0 document = .l({
            [.w? "#" ..."\n"?]
            [.w? .."="!:"key" .w? "=" .w? {
                .$_:"number"
                {"true":"bool" "false":!"bool"}
                .t?:"string"
                ..""!:"value"
            } .w?]
        })"##;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = stderr_unwrap(rules, syntax(rules));
    let mut data = vec![];
    stderr_unwrap(text, parse(&rules, text, &mut data));
    /* prints

    "key":"age",
    "number":250,
    "key":"strength",
    "number":200,
    "key":"name",
    "string":"Big Dragon",
    "key":"violent",
    "bool":true

    */
    json::print(&data);
}
