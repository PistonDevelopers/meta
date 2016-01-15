# piston_meta
A DSL parsing library for human readable text documents

[![Travis](https://img.shields.io/travis/PistonDevelopers/meta.svg?style=flat-square)](https://travis-ci.org/PistonDevelopers/meta)
[![Crates.io](https://img.shields.io/crates/v/piston_meta.svg?style=flat-square)](https://crates.io/crates/piston_meta)

[Documentation](http://docs.piston.rs/piston_meta/piston_meta/)

[Why Piston-Meta?](https://github.com/PistonDevelopers/meta/issues/1)

[self-syntax](https://raw.githubusercontent.com/PistonDevelopers/meta/master/assets/self-syntax.txt)

### Rules

The meta language is used to describe how to read other documents.
First you define some strings to reuse, then some node rules.
The last node is used to read the entire document.

`20 document = [.l(string:"string") .l(node:"node") .w?]`

Strings starts with underscore and can be reused among the rules:

`_opt: "optional"`

Nodes start with a number that gets multiplied with 1000 and used as debug id.
If you get an error `#4003`, then it was caused by a rule in the node starting with 4.

|Rule|Description|
|----|-----------|
|.l(rule)|Separates sub rule with lines.|
|.r?(rule)|Repeats sub rule until it fails, allows zero repetitions.|
|.r!(rule)|Repeats sub rule until it fails, requires at least one repetition.|
|...any_characters?:name|Reads a string until any characters, allows zero characters. Name is optional.|
|...any_characters!:name|Reads a string until any characters, requires at least one character. Name is optional.|
|..any_characters?:name|Reads a string until any characters or whitespace, allows zero characters. Name is optional.|
|..any_characters!:name|Reads a string until any characters or whitespace, requires at least one character. Name is optional.|
|.w?|Reads whitespace. The whitespace is optional.|
|.w!|Reads whitespace. The whitespace is required.|
|?rule|Makes the rule optional.|
|"token":name|Expects a token, sets name to `true`. Name is optional.|
|"token":!name|Expects a token, sets name to `false`. Name is required.|
|!"token":name|Fails if token is read, sets name to `true` if it is not read. Name is optional.|
|!"token":!name|Fails if token is read, sets name to `false` if it is not read. Name is required.|
|.s?(by_rule rule)|Separates rule by another rule, allows zero repetitions.|
|.s!(by_rule rule)|Separates rule by another rule, requires at least one repetition.|
|.s?.(by_rule rule)|Separates rule by another rule, allows trailing.|
|{rules}|Selects a rule. Tries the first rule, then the second etc. Rules are separated by whitespace.|
|[rules]|A sequence of rules. Rules are separated by whitespace.|
|node|Uses a node without a name. The read data is put in the current node.|
|node:name|Uses a node with a name. The read data is put in a new node with the name.|
|.t?:name|Reads a JSON string with a name. The string can be empty. Name is optional.|
|.t!:name|Reads a JSON string with a name. The string can not be empty. Name is optional.|
|.$:name|Reads a number with a name. The name is optional.|
|.$_:name|Reads a number with underscore as visible separator, for example `10_000`. The name is optional.|

### "Hello world" in Piston-Meta

Piston-Meta allows parsing into any structure implementing `MetaReader`, for example `Tokenizer`.
`Tokenizer` stores the tree structure in a flat `Vec` with "start node" and "end node" items.

```Rust
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
```

### How does it work?

1. Piston-Meta contains composable rules that can parse most human readable text formats.
2. Piston-Meta knows how to parse and convert to its own rules, known as "bootstrapping".
3. Therefore, you can tell Piston-Meta how to parse other text formats using a meta language!
