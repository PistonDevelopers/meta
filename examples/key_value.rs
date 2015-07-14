
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
        0 "document" l({
            [w? "#" ..."\n"?]
            [w? .."="!"key" w? "=" w? {
                $_"number"
                {"true""bool" "false"!"bool"}
                t?"string"
                ..""!"value"
            } w?]
        })"##;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = bootstrap::convert(
        &parse(&bootstrap::rules(), rules).unwrap(),
        &mut vec![] // stores ignored meta data
    ).unwrap();
    let data = parse(&rules, text);
    match data {
        Ok(data) => {
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
        Err((range, err)) => {
            // Report the error to standard error output.
            ParseStdErr::new(&text).error(range, err);
        }
    }
}
