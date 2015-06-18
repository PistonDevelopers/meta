# piston_meta
A DSL parsing library for human readable text documents

[Why Piston-Meta?](https://github.com/PistonDevelopers/meta/issues/1)

[self-syntax](https://raw.githubusercontent.com/PistonDevelopers/meta/master/assets/self-syntax.txt)

*Notice: Parsing is supported but composing is not implemented yet.*

### "Hello world" in Piston-Meta

Piston-Meta allows parsing into any structure implementing `MetaReader`, for example `Tokenizer`.
`Tokenizer` stores the tree structure in a flat `Vec` with "start node" and "end node" items.

```Rust
extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = "say \"Hello world!\"";
    let rules = "1 \"rule\" [\"say\" w! t?\"foo\"]";
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = bootstrap::convert(
        &parse(&bootstrap::rules(), rules).unwrap(),
        &mut vec![] // stores ignored meta data
    ).unwrap();
    let data = parse(&rules, text).unwrap();
    assert_eq!(data.len(), 1);
    if let &MetaData::String(_, ref hello) = &data[0].1 {
        println!("{}", hello);
    }
}
```

### How does it work?

1. Piston-Meta contains composable rules that can parse most human readable text formats.
2. Piston-Meta knows how to parse and convert to its own rules, known as "bootstrapping".
3. Therefore, you can tell Piston-Meta how to parse other text formats using a meta language!
