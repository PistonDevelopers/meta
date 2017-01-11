#![feature(test)]

extern crate piston_meta;
extern crate test;

use piston_meta::{
    syntax_errstr,
    parse_errstr,
};
use test::Bencher;

#[bench]
fn bench_hello(b: &mut Bencher) {
    b.iter(|| {
        let text = r#"hi James!"#;
        let rules = r#"
            1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
            2 document = say_hi
        "#;
        // Parse rules with meta language and convert to rules for parsing text.
        let rules = match syntax_errstr(rules) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(rules) => rules
        };
        let mut data = vec![];
        match parse_errstr(&rules, text, &mut data) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(()) => {}
        };
    });
}

#[bench]
fn bench_deep_string(b: &mut Bencher) {
    b.iter(|| {
        let text = r#"hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi hi
        foo"#;
        let rules = r#"
            1 foo = [.." "!:"foo" .w! {"foo" foo}]
            2 document = foo
        "#;
        // Parse rules with meta language and convert to rules for parsing text.
        let rules = match syntax_errstr(rules) {
            Err(err) => panic!("{}", err),
            Ok(rules) => rules
        };
        let mut data = vec![];
        match parse_errstr(&rules, text, &mut data) {
            Err(err) => panic!("{}", err),
            Ok(()) => {}
        };
    });
}
