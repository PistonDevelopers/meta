extern crate piston_meta;
extern crate range;

use range::Range;
use piston_meta::*;

pub fn use_new_self_syntax(rules: &str, text: &str) -> Vec<Range<MetaData>> {
    // Bootstrap rules.
    let new_self_syntax = include_str!("../assets/new-self-syntax.txt");
    let bootstrapped_rules = match syntax_errstr(new_self_syntax) {
        Err(err) => panic!("{}", err),
        Ok(rules) => rules
    };
    // Parse rules with meta language and convert to rules for parsing text.
    let mut doc_rules = vec![];
    match parse_errstr(&bootstrapped_rules, rules, &mut doc_rules) {
        Err(err) => panic!("{}", err),
        Ok(()) => {}
    };
    let mut ignored1 = vec![];
    let doc_rules = bootstrap::convert(
        &doc_rules, &mut ignored1).unwrap();
    // Parse text.
    let mut data = vec![];
    match parse_errstr(&doc_rules, text, &mut data) {
        Err(err) => panic!("{}", err),
        Ok(()) => {}
    };
    data
}

pub fn use_old_self_syntax(rules: &str, text: &str) -> Vec<Range<MetaData>> {
    let rules = match syntax_errstr(rules) {
        Err(err) => panic!("{}", err),
        Ok(rules) => rules
    };
    let mut data = vec![];
    match parse_errstr(&rules, text, &mut data) {
        Err(err) => panic!("{}", err),
        Ok(()) => {}
    };
    data
}

#[test]
fn url_in_multiline_comments() {
    let text = r#"hi James!"#;
    let rules = r#"
        /* this is an url http://www.piston.rs */
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    let _ = use_new_self_syntax(rules, text);
}

#[test]
fn star_in_multiline_comments() {
    let text = r#"hi James!"#;
    let rules = r#"
        /* this is a comment with **** and some **** */
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    let _ = use_new_self_syntax(rules, text);
}

#[test]
fn nested_multiline_comments() {
    let text = r#"hi James!"#;
    let rules = r#"
        /* this is a nested comment /* hey I think /* this */ works */ */
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    let _ = use_new_self_syntax(rules, text);
}

#[test]
#[should_panic(expected = "Expected: `*/`")]
fn nested_multiline_comments_fail() {
    let text = r#"hi James!"#;
    let rules = r#"
        /* this is a nested comment /* hey I think /* this works, no? */ */
        1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
        2 document = say_hi
    "#;
    let _ = use_new_self_syntax(rules, text);
}
