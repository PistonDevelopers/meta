# piston_meta
A research project of meta parsing and composing for data oriented design

Piston-Meta is a research project under the Piston project to explore the use of meta parsing and composing for data oriented design. Meta parsing is a technique where you use a rules to describe how to transform a text into a tree structure, in a similar way that a parser generator can generate code for reading a text by a grammar, except that the rules in meta parsing can be changed at run time, or even read from a text described by the same rules. Piston-Meta is inspired by OMeta (https://en.wikipedia.org/wiki/OMeta) developed at Viewpoints Research Institute in 2007.

Meta parsing and composing could be very useful in game programming, where you have lots of data with slightly different grammar. This could be domain specific languages for simple scripting, or data to feed a system that requires a specific layout, or a configuration format that you develop while working on a project. The normal way is to read and write the data using a document library, for example JSON, but there are several downsides:

1. Changes in the document structure are entangled with the application code
2. It requires one dependency for each document format
3. Data validation is limited and hard
4. Writing parsing logic manually is error prone and composing is duplicated work
5. Error messages are bad

Piston-Meta solves these problems by using meta rules that fits for working with data, where good error messages are important, and where the abstraction level is the same as JSON building blocks. For more information see https://github.com/PistonDevelopers/meta/issues/1.

*Notice: Parsing is supported but composing is not implemented yet.*

### "Hello world" in Piston-Meta

Piston-Meta allows parsing into any structure implementing `MetaReader`, for example `Tokenizer`.
`Tokenizer` stores the tree structure in a flat `Vec` with "start node" and "end node" items.

```Rust
extern crate piston_meta;
extern crate range;

use std::rc::Rc;
use piston_meta::*;

fn main() {
    let text = "foo \"Hello world!\"";
    let chars: Vec<char> = text.chars().collect();
    let mut tokenizer = Tokenizer::new();
    let s = TokenizerState::new();
    let foo: Rc<String> = Rc::new("foo".into());
    let text = Text {
        allow_empty: true,
        property: Some(foo.clone())
    };
    let _ = text.parse(&mut tokenizer, &s, &chars[4..], 4);
    assert_eq!(tokenizer.tokens.len(), 1);
    if let &MetaData::String(_, ref hello) = &tokenizer.tokens[0].0 {
        println!("{}", hello);
    }
}
```

### New features

Parsing is now working! This library contains the rules and parse algorithms. For examples, see the unit tests. 

The deepest error is picked to make better error messages. When a text is parsed successfully, the result contains an optional error which can be used for additional success checks, for example whether it reached the end of a file.

- `Node` (allows reuse a rule and reference itself)
- `Number` (reads f64)
- `Optional` (a sequence of rules that can fail without failing parent rule)
- `Rule` (an enum for all the rules)
- `Select` (tries one sub rule and continues if it fails)
- `SeparatedBy` (separates a rule by another rule)
- `Sequence` (rules in sequence)
- `Text` (a JSON string)
- `Token` (expects a sequence of characters)
- `UntilAnyOrWhitespace` (reads until it hits any of the specified characters or whitespace)
- `Whitespace` (reads whitespace)

### Referencing other rules or itself

The `Node` struct is the basis for creating rules that can be referenced by other rules.
When writing the rules, use the `NodeRef::Name` and then call `Rule::update_refs` to replace them with references.
`update_refs` walks over the rules with a list of rules you want to put in, and replaces `NodeRef::Name` with `NodeRef::Ref`.

### How to use the library for game development

Just combine rules that describe how a text is interpreted. For example:

```
weapons: fork, sword, gun, lazer_beam
```

Assume we wanted to read this into an array of weapons, we could make up a simple pseudo language for helping us figuring out the rules:

```
array: ["weapons:", w!, sep(until(",") -> item, [",", w?])]
```

`w!` means required `Whitespace` and `w?` means optional `Whitespace`. The allow `->` tells what field to store that data, which is the `property` of that rule. "sep" is an abbreviation for `SeparatedBy`, "until" is for `UntilWhitespaceOrAny`. The stuff in quotes are `Token`.

Now you have 2 options:

1. Write the rules in Rust directly, which requires recompiling each time you change the data format.
2. Write the rules for parsing your pseudo language, then read the rules to read the data from a text document. This allows you to change the data format without recompiling.

The second option is useful combined with an Entity Component System (ECS) because it allows you to expand both the document format and your application with new features without recompiling. The technique is called "bootstrapping" because you are writing rules in Rust that describes how to read rules from a text document which is parsed into rules in Rust. This idea takes some time to get used to, so stick to the first option until you get familiar with the library and the meta thought process.

### Bootstrapping

It is not as hard as it sounds, because you can use the pseudo language to read data to help you write the rules in Rust for it. Just look at the rule and write down a list of what you need to describe it.

```
array: ["weapons:", w!, sep(until(",") -> item, [",", w?])]
```

1. We need a rule for reading stuff like "array:".
2. We need a rule for reading a text string "weapons:" and ",".
3. We need a rule for reading "w!" and "w?".
4. We need a rule for reading "until(<any_characters>)".
5. We need a rule for reading stuff like "[...]" that can contain 2, 3, 4 and itself 5.
6. We need a rule for the whole document that repeats 1 followed by 5, separated by new lines

Write this down in your pseudo language:

```
1. label: [text -> name, ":"]
2. text: ["text -> ", text -> name]
3. whitespace: ["w", sel("!" -> required, "?" -> optional)]
4. until_any_or_whitespace: ["until(", text -> any_characters, ")"]
5. list: ["[", sep([w?, sel(text, whitespace, until_any_or_whitespace, list)], ",")]
6. document: [sep([label, w!, list], "\n"]
```

Now, replace these rules with Piston-Meta rules (pseudo code):

```Rust
let label = Node {
    name: "label",
    body: Sequence {
        args: vec![Text { property: "name" }, Token { text: ":" }]
    }
};
let text = Node {
    name: "text",
    body: Sequence {
        args: vec![
            Token { text: "text -> " },
            Text { property: "name" }
        ]
    }
};
...
```

### Confused?

Here is an overview of the process:

First you need some rules to parse the meta language, which you could read with `Tokenizer` and then convert to rules in Rust. These rules are then used to read the data, which then is converted to the application structure in Rust.

```
rules (for meta language) -> document (meta language describing rules for data) -> meta data
-> rules (for data) -> document (data) -> meta data -> data
```

In general, meta parsing is about this kind of transformation:

```
rules (for X) -> document (X describing Y) -> meta data -> Y
```

This transformation is composable, so you can build an infrastructure around it.
